use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::{mint_to, MintTo, ID as TOKEN_EXTENSIONS_PROGRAM_ID},
    token_interface::{Mint, Token2022, TokenAccount},
};

use crate::constants::*;
use crate::errors::*;
use crate::state::*;

#[derive(Accounts)]
pub struct MintPreset<'info> {
    #[account(
      seeds = [
        PREFIX,
        PRESET,
        &preset.name.as_bytes()
      ],
      bump = preset.bump,
      has_one = fee_payer @ TokenGatorPresetError::UnAuthorized,
      constraint = preset.check_for_authority(&authority.key()) @ TokenGatorPresetError::UnAuthorized,
      constraint = preset.minter_config.mint.eq(&mint.key()) @ TokenGatorPresetError::InvalidMint
    )]
    pub preset: Account<'info, Preset>,

    #[account(
      mut,
      mint::authority = fee_payer,
      mint::freeze_authority = fee_payer,
      mint::token_program = token_program,
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
      init,
      payer = fee_payer,
      associated_token::mint = mint,
      associated_token::authority = authority,
      associated_token::token_program = token_program
    )]
    pub authority_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub fee_payer: Signer<'info>,
    pub authority: Signer<'info>,

    #[account(
      constraint = token_program.key().eq(&TOKEN_EXTENSIONS_PROGRAM_ID) @ TokenGatorPresetError::InvalidTokenProgram
    )]
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn mint(ctx: Context<MintPreset>) -> Result<()> {
    let fee_payer = &ctx.accounts.fee_payer;

    let mint = &ctx.accounts.mint;
    let destination_token_account = &ctx.accounts.authority_token_account;
    let token_extensions_program = &ctx.accounts.token_program;

    let amount_with_decimals = 1u64
        .checked_mul(10u64.checked_pow(mint.decimals.into()).unwrap())
        .unwrap();

    mint_to(
        CpiContext::new(
            token_extensions_program.to_account_info(),
            MintTo {
                mint: mint.to_account_info(),
                authority: fee_payer.to_account_info(),
                to: destination_token_account.to_account_info(),
            },
        ),
        amount_with_decimals,
    )?;

    Ok(())
}
