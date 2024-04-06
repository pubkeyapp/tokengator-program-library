use anchor_lang::prelude::*;
use anchor_spl::{token_2022::Token2022, token_interface::Mint};
use wen_new_standard::{
    cpi::{
        accounts::{AddMetadata, RemoveMetadata},
        add_metadata, remove_metadata,
    },
    program::WenNewStandard,
    AddMetadataArgs, RemoveMetadataArgs, TokenGroup, TokenGroupMember,
};

use crate::constants::*;
use crate::errors::*;
use crate::state::*;

#[derive(Accounts)]
#[instruction(args: UpdateMemberMetadataWNSArgs)]
pub struct UpdateMemberMetadataWNS<'info> {
    #[account(
      mut,
      seeds = [
        PREFIX,
        MINTER,
        minter.minter_config.mint.as_ref(),
        minter.name.as_bytes()
      ],
      bump = minter.bump,
      has_one = fee_payer @ TokenGatorMinterError::InvalidFeePayer,
      has_one = group @ TokenGatorMinterError::InvalidWNSGroup,
    )]
    pub minter: Account<'info, Minter>,

    #[account(
      constraint = group.update_authority.eq(&minter.key()) @ TokenGatorMinterError::UnAuthorized
    )]
    pub group: Account<'info, TokenGroup>,

    #[account(
      has_one = group @ TokenGatorMinterError::InvalidWNSGroup,
      has_one = mint @ TokenGatorMinterError::InvalidWNSMember
    )]
    pub member: Account<'info, TokenGroupMember>,

    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub fee_payer: Signer<'info>,

    pub wns_program: Program<'info, WenNewStandard>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

pub fn update(
    ctx: Context<UpdateMemberMetadataWNS>,
    args: UpdateMemberMetadataWNSArgs,
) -> Result<()> {
    let minter = &ctx.accounts.minter;
    let mint = &ctx.accounts.mint;
    let fee_payer = &ctx.accounts.fee_payer;

    let token_extensions_program = &ctx.accounts.token_program;
    let system_program = &ctx.accounts.system_program;
    let wns_program = &ctx.accounts.wns_program;

    let signer_seeds: &[&[&[u8]]] = &[&[
        PREFIX,
        MINTER,
        minter.minter_config.mint.as_ref(),
        minter.name.as_bytes(),
        &[minter.bump],
    ]];

    remove_metadata(
        CpiContext::new_with_signer(
            wns_program.to_account_info(),
            RemoveMetadata {
                payer: fee_payer.to_account_info(),
                authority: minter.to_account_info(),
                mint: mint.to_account_info(),
                system_program: system_program.to_account_info(),
                token_program: token_extensions_program.to_account_info(),
            },
            signer_seeds,
        ),
        vec![RemoveMetadataArgs {
            field: args.field.clone(),
            value: String::from(""),
        }],
    )?;

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
        vec![AddMetadataArgs {
            field: args.field,
            value: args.new_value,
        }],
    )?;

    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateMemberMetadataWNSArgs {
    pub field: String,
    pub new_value: String,
}
