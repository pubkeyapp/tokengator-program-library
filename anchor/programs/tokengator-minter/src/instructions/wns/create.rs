use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::{
        create as create_associated_token, get_associated_token_address_with_program_id,
        AssociatedToken, Create as CreateAssociatedToken,
    },
    token_2022::{transfer_checked, Token2022, TransferChecked},
    token_interface::{Mint, TokenAccount},
};
use wen_new_standard::{
    cpi::{
        accounts::{AddMetadata, CreateGroupAccount},
        add_metadata, create_group_account,
    },
    program::WenNewStandard,
    AddMetadataArgs, CreateGroupAccountArgs,
};

use crate::args::*;
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
      mut,
      seeds = [
        PREFIX,
        RECEIPT,
        receipt.sender.as_ref(),
        receipt.receiver.as_ref(),
        receipt.payment_mint.as_ref(),
      ],
      bump = receipt.bump,
      constraint = matches!(receipt.payment_type, ReceiptType::Community) @ TokenGatorMinterError::InvalidReceipt,
      constraint = receipt.receiver.eq(&authority.key()) @ TokenGatorMinterError::InvalidAuthority,
      constraint = receipt.receiver_token_account.eq(&authority_token_account.key()) @ TokenGatorMinterError::InvalidAuthority,
      has_one = payment_mint @ TokenGatorMinterError::InvalidMint,
      constraint = receipt.payment_amount.eq(&args.payment_config.price) @ TokenGatorMinterError::InvalidReceipt
    )]
    pub receipt: Account<'info, Receipt>,

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

    #[account(
      mut,
      token::authority = authority,
      token::mint = payment_mint,
    )]
    pub authority_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    /// CHECK: PDA checks done below
    #[account(mut)]
    pub fee_payer_token_account: UncheckedAccount<'info>,

    #[account(
      constraint = payment_mint.to_account_info().owner.eq(&token_program.key())
    )]
    pub payment_mint: Box<InterfaceAccount<'info, Mint>>,

    pub rent: Sysvar<'info, Rent>,
    pub wns_program: Program<'info, WenNewStandard>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn create(ctx: Context<CreateMinterWNS>, args: CreateMinterWNSArgs) -> Result<()> {
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

    let CreateMinterWNSArgs {
        metadata_config,
        transfer_fee_config,
        interest_config,
        application_config,
        payment_config,
        ..
    } = args;

    // Transfer payment for minter creation
    let expected_fee_payer_token_account = get_associated_token_address_with_program_id(
        &ctx.accounts.fee_payer.key(),
        &ctx.accounts.payment_mint.key(),
        &ctx.accounts.token_program.key(),
    );

    require_eq!(
        expected_fee_payer_token_account,
        ctx.accounts.fee_payer_token_account.key()
    );

    if ctx
        .accounts
        .fee_payer_token_account
        .to_account_info()
        .data_is_empty()
    {
        create_associated_token(CpiContext::new(
            ctx.accounts.associated_token_program.to_account_info(),
            CreateAssociatedToken {
                payer: ctx.accounts.fee_payer.to_account_info(),
                associated_token: ctx.accounts.fee_payer_token_account.to_account_info(),
                authority: ctx.accounts.fee_payer.to_account_info(),
                mint: ctx.accounts.payment_mint.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            },
        ))?;
    }

    transfer_checked(
        CpiContext::new(
            token_extensions_program.to_account_info(),
            TransferChecked {
                authority: ctx.accounts.authority.to_account_info(),
                from: ctx.accounts.authority_token_account.to_account_info(),
                to: ctx.accounts.fee_payer_token_account.to_account_info(),
                mint: ctx.accounts.payment_mint.to_account_info(),
            },
        ),
        payment_config.price.into(),
        ctx.accounts.payment_mint.decimals,
    )?;

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

    let payment_config = PaymentConfig {
        price: payment_config.price,
        amount: payment_config.amount,
        mint: payment_config.mint,
        days: payment_config.days,
        expires_at: Clock::get()?
            .unix_timestamp
            .checked_add(60 * 60 * 24 * i64::try_from(payment_config.days).unwrap())
            .unwrap(),
    };

    let application_payment_config = PaymentConfig {
        price: application_config.payment_config.price,
        amount: application_config.payment_config.amount,
        mint: application_config.payment_config.mint,
        days: application_config.payment_config.days,
        expires_at: 0,
    };

    let application_config = MinterApplicationConfig {
        identities: application_config.identities,
        payment_config: application_payment_config,
    };

    // 1. Saving Minter onchain
    let minter_config = MinterConfig {
        mint: mint.key(),
        application_config,
        interest_config: interest_config.clone(),
        metadata_config: metadata_config.clone(),
        transfer_fee_config: transfer_fee_config.clone(),
    };

    minter.set_inner(Minter {
        bump: ctx.bumps.minter,
        community_id,
        group: group_key,
        name: args.name.clone(),
        description: args.description,
        image_url: args.image_url,
        fee_payer: ctx.accounts.fee_payer.key(),
        authorities: vec![authority.key()],
        minter_config,
        payment_config,
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

    // 3. Updating additional metadata
    if let Some(metadata) = metadata_config.metadata {
        let metadata_args = metadata
            .iter()
            .map(|m| AddMetadataArgs {
                field: m[0].clone(),
                value: m[1].clone(),
            })
            .collect();

        add_metadata(
            CpiContext::new_with_signer(
                wns_program.to_account_info(),
                AddMetadata {
                    payer: fee_payer.to_account_info(),
                    authority: minter.to_account_info(),
                    mint: mint.to_account_info(),
                    system_program: system_program.to_account_info(),
                    token_program: token_extensions_program.to_account_info(),
                },
                signer_seeds,
            ),
            metadata_args,
        )?;
    }

    // 4. Closing receipt
    ctx.accounts.receipt.close(fee_payer.to_account_info())?;

    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateMinterWNSArgs {
    pub community: String,
    pub name: String,
    pub description: String,
    pub image_url: String,
    pub payment_config: PaymentConfigArgs,
    pub application_config: MinterApplicationConfig,
    pub metadata_config: MinterMetadataConfig,
    pub interest_config: Option<MinterInterestConfig>,
    pub transfer_fee_config: Option<MinterTransferFeeConfig>,
}
