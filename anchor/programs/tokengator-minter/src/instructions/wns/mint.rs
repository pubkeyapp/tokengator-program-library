use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::{get_associated_token_address_with_program_id, AssociatedToken},
    token_2022::ID as TOKEN_EXTENSIONS_PROGRAM_ID,
    token_interface::Token2022,
};
use wen_new_standard::{
    cpi::{
        accounts::{AddGroup, AddMetadata, CreateMintAccount},
        add_metadata, add_mint_to_group, create_mint_account,
    },
    program::WenNewStandard,
    AddMetadataArgs, CreateMintAccountArgs, Manager, TokenGroup,
};

use crate::constants::*;
use crate::errors::*;
use crate::state::*;
use crate::utils::*;

#[derive(Accounts)]
pub struct MintMinterWNS<'info> {
    /** WNS ACCOUNTS */
    pub manager: Account<'info, Manager>,
    #[account(mut)]
    pub group: Account<'info, TokenGroup>,
    #[account(mut)]
    /// CHECK: PDA checks done below
    pub member: UncheckedAccount<'info>,
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
      constraint = matches!(receipt.payment_type, ReceiptType::User) @ TokenGatorMinterError::InvalidReceipt,
      constraint = receipt.receiver.eq(&authority.key()) @ TokenGatorMinterError::InvalidAuthority,
      constraint = receipt.sender.eq(&receiver.key()) @ TokenGatorMinterError::InvalidReceiver,
      constraint = receipt.payment_amount.eq(&minter.minter_config.application_config.payment_config.price) @ TokenGatorMinterError::InvalidReceipt
    )]
    pub receipt: Account<'info, Receipt>,

    #[account(
      mut,
      seeds = [
        PREFIX,
        MINTER,
        &minter.minter_config.mint.as_ref(),
        &minter.name.as_bytes()
      ],
      bump = minter.bump,
      has_one = fee_payer @ TokenGatorMinterError::UnAuthorized,
      constraint = minter.check_for_authority(&authority.key()) @ TokenGatorMinterError::UnAuthorized,
    )]
    pub minter: Account<'info, Minter>,

    #[account(mut)]
    pub mint: Signer<'info>,

    #[account(mut)]
    /// CHECK: Checks done inside the handler function
    pub receiver_token_account: UncheckedAccount<'info>,

    #[account(mut)]
    pub fee_payer: Signer<'info>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account()]
    pub receiver: SystemAccount<'info>,

    #[account(
      constraint = token_program.key().eq(&TOKEN_EXTENSIONS_PROGRAM_ID) @ TokenGatorMinterError::InvalidTokenProgram
    )]
    pub token_program: Program<'info, Token2022>,
    pub wns_program: Program<'info, WenNewStandard>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn mint(ctx: Context<MintMinterWNS>, args: MintMinterWNSArgs) -> Result<()> {
    let minter = &mut ctx.accounts.minter;
    let group = &mut ctx.accounts.group;
    let member = &mut ctx.accounts.member;
    let manager = &ctx.accounts.manager;

    let receiver = &ctx.accounts.receiver;
    let fee_payer = &ctx.accounts.fee_payer;
    let mint = &ctx.accounts.mint;
    let receiver_token_account = &ctx.accounts.receiver_token_account;

    let rent = &ctx.accounts.rent;
    let wns_program = &ctx.accounts.wns_program;
    let token_extensions_program = &ctx.accounts.token_program;
    let system_program = &ctx.accounts.system_program;
    let associated_token_program = &ctx.accounts.associated_token_program;

    let receiver_key = receiver.key();
    let minter_key = minter.key();
    let mint_key = mint.key();
    let member_key = member.key();
    let group_key = group.key();
    let manager_key = manager.key();

    let expected_receiver_token_account = get_associated_token_address_with_program_id(
        &receiver_key,
        &mint_key,
        &token_extensions_program.key(),
    );
    require_eq!(
        expected_receiver_token_account,
        receiver_token_account.key(),
        TokenGatorMinterError::InvalidAuthorityTokenAccount
    );

    check_for_wns_accounts(
        &group.mint,
        &group_key,
        &manager_key,
        &Some(mint_key),
        &Some(member_key),
    )?;

    let signer_seeds: &[&[&[u8]]] = &[&[
        PREFIX,
        MINTER,
        minter.minter_config.mint.as_ref(),
        minter.name.as_bytes(),
        &[minter.bump],
    ]];

    let MintMinterWNSArgs {
        name, symbol, uri, ..
    } = args;

    // 1. Creating member mint
    create_mint_account(
        CpiContext::new_with_signer(
            wns_program.to_account_info(),
            CreateMintAccount {
                associated_token_program: associated_token_program.to_account_info(),
                authority: minter.to_account_info(),
                manager: manager.to_account_info(),
                mint: mint.to_account_info(),
                mint_token_account: receiver_token_account.to_account_info(),
                payer: fee_payer.to_account_info(),
                receiver: receiver.to_account_info(),
                rent: rent.to_account_info(),
                system_program: system_program.to_account_info(),
                token_program: token_extensions_program.to_account_info(),
            },
            signer_seeds,
        ),
        CreateMintAccountArgs {
            name,
            symbol,
            uri,
            permanent_delegate: Some(minter_key),
        },
    )?;

    // 2. Updating additional metadata
    if let Some(metadata) = args.metadata {
        let issued_at = Clock::get()?.unix_timestamp;
        let expires_at = issued_at
            .checked_add(
                60 * 60
                    * 24
                    * i64::try_from(minter.minter_config.application_config.payment_config.days)
                        .unwrap(),
            )
            .unwrap();

        let metadata_args: Vec<AddMetadataArgs> = [
            metadata,
            vec![["issued_at".to_owned(), issued_at.to_string()]],
            vec![["expires_at".to_owned(), expires_at.to_string()]],
        ]
        .concat()
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

    // 3. Adding member to group
    add_mint_to_group(CpiContext::new_with_signer(
        wns_program.to_account_info(),
        AddGroup {
            authority: minter.to_account_info(),
            group: group.to_account_info(),
            member: member.to_account_info(),
            manager: manager.to_account_info(),
            mint: mint.to_account_info(),
            payer: fee_payer.to_account_info(),
            system_program: system_program.to_account_info(),
            token_program: token_extensions_program.to_account_info(),
        },
        signer_seeds,
    ))?;

    // 4. Updating minter
    minter
        .minter_config
        .application_config
        .payment_config
        .expires_at = Clock::get()?.unix_timestamp
        + (60
            * 60
            * 24
            * i64::try_from(minter.minter_config.application_config.payment_config.days).unwrap());

    minter.validate()?;

    // 5. Closing Receipt
    ctx.accounts.receipt.close(fee_payer.to_account_info())?;

    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct MintMinterWNSArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub metadata: Option<Vec<[String; 2]>>,
}
