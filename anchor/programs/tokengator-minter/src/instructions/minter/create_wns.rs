use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::{get_associated_token_address_with_program_id, AssociatedToken},
    token_2022::Token2022,
};
use wen_new_standard::{
    cpi::{accounts::CreateGroupAccount, create_group_account},
    program::WenNewStandard,
    CreateGroupAccountArgs,
};

use crate::constants::*;
use crate::errors::*;
use crate::state::*;
use crate::utils::*;

#[derive(Accounts)]
#[instruction(args: CreateMinterWNSArgs)]
pub struct CreateMinterWNS<'info> {
    /** WNS ACCOUNTS */
    #[account(mut)]
    /// CHECK: PDA checks done below
    pub group: UncheckedAccount<'info>,
    /// CHECK: PDA checks done below
    pub manager: UncheckedAccount<'info>,
    /** */

    #[account(
      init,
      payer = fee_payer,
      space = Minter::size(&[authority.key()], &args.application_config, &args.metadata_config),
      seeds = [
        PREFIX,
        MINTER,
        mint.key().as_ref(),
        &args.name.as_bytes()
      ],
      bump
    )]
    pub minter: Account<'info, Minter>,

    #[account(mut)]
    /// CHECK: Checks done inside the handler function
    pub minter_token_account: UncheckedAccount<'info>,

    #[account(mut)]
    pub mint: Signer<'info>,

    #[account(
      mut,
      constraint = fee_payer.key().ne(&authority.key()) @ TokenGatorMinterError::InvalidFeePayer
    )]
    pub fee_payer: Signer<'info>,
    pub authority: Signer<'info>,

    pub rent: Sysvar<'info, Rent>,
    pub wns_program: Program<'info, WenNewStandard>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn create_wns(ctx: Context<CreateMinterWNS>, args: CreateMinterWNSArgs) -> Result<()> {
    let minter = &mut ctx.accounts.minter;
    let group = &mut ctx.accounts.group;
    let manager = &mut ctx.accounts.manager;

    let authority = &ctx.accounts.authority;
    let fee_payer = &ctx.accounts.fee_payer;
    let mint = &ctx.accounts.mint;
    let minter_token_account = &ctx.accounts.minter_token_account;

    let rent = &ctx.accounts.rent;
    let wns_program = &ctx.accounts.wns_program;
    let token_extensions_program = &ctx.accounts.token_program;
    let system_program = &ctx.accounts.system_program;
    let associated_token_program = &ctx.accounts.associated_token_program;

    let minter_key = minter.key();
    let mint_key = mint.key();
    let group_key = group.key();
    let manager_key = manager.key();

    let community_id = fetch_community_id(&args.community);

    let expected_minter_token_account = get_associated_token_address_with_program_id(
        &minter_key,
        &mint_key,
        &token_extensions_program.key(),
    );
    require_eq!(
        expected_minter_token_account,
        minter_token_account.key(),
        TokenGatorMinterError::InvalidMinterTokenAccount
    );

    check_for_wns_accounts(&mint_key, &group_key, &manager_key, &None, &None)?;

    let CreateMinterWNSArgs {
        metadata_config,
        transfer_fee_config,
        interest_config,
        application_config,
        payment_config,
        ..
    } = args;

    // 1. Saving Minter onchain
    let minter_config = MinterConfig {
        mint: mint.key(),
        application_config: application_config.clone(),
        interest_config: interest_config.clone(),
        metadata_config: metadata_config.clone(),
        transfer_fee_config: transfer_fee_config.clone(),
    };

    minter.set_inner(Minter {
        bump: ctx.bumps.minter,
        community_id,
        name: args.name.clone(),
        description: args.description,
        image_url: args.image_url,
        fee_payer: ctx.accounts.fee_payer.key(),
        authorities: vec![authority.key()],
        payment_config,
        minter_config,
    });

    minter.validate()?;

    // 2. Creating WNS group
    let signer_seeds: &[&[&[u8]]] = &[&[
        PREFIX,
        MINTER,
        mint_key.as_ref(),
        minter.name.as_bytes(),
        &[minter.bump],
    ]];
    create_group_account(
        CpiContext::new_with_signer(
            wns_program.to_account_info(),
            CreateGroupAccount {
                group: group.to_account_info(),
                manager: manager.to_account_info(),
                mint: mint.to_account_info(),
                mint_token_account: minter_token_account.to_account_info(),
                payer: fee_payer.to_account_info(),
                authority: minter.to_account_info(),
                receiver: minter.to_account_info(),
                rent: rent.to_account_info(),
                system_program: system_program.to_account_info(),
                token_program: token_extensions_program.to_account_info(),
                associated_token_program: associated_token_program.to_account_info(),
            },
            signer_seeds,
        ),
        CreateGroupAccountArgs {
            name: metadata_config.name,
            symbol: metadata_config.symbol,
            uri: metadata_config.uri,
            max_size: 100,
        },
    )?;

    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateMinterWNSArgs {
    pub community: String,
    pub name: String,
    pub description: String,
    pub image_url: String,
    pub payment_config: PaymentConfig,
    pub application_config: MinterApplicationConfig,
    pub metadata_config: MinterMetadataConfig,
    pub interest_config: Option<MinterInterestConfig>,
    pub transfer_fee_config: Option<MinterTransferFeeConfig>,
}
