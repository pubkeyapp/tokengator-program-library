use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::{close_account, CloseAccount, ID as TOKEN_EXTENSIONS_PROGRAM_ID},
    token_interface::{Mint, Token2022},
};

use crate::constants::*;
use crate::errors::*;
use crate::state::*;

#[derive(Accounts)]
pub struct RemovePreset<'info> {
    #[account(
      mut,
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
      mint::authority = preset,
      mint::freeze_authority = preset,
      mint::token_program = token_program,
      constraint = mint.supply == 0 @ TokenGatorPresetError::CannotRemoveNonZeroSupplyPreset
    )]
    pub mint: InterfaceAccount<'info, Mint>,

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

pub fn remove(ctx: Context<RemovePreset>) -> Result<()> {
    let fee_payer = &ctx.accounts.fee_payer;
    let preset = &ctx.accounts.preset;

    let mint = &ctx.accounts.mint;
    let token_extensions_program = &ctx.accounts.token_program;

    let signer_seeds: &[&[&[u8]]] = &[&[PREFIX, PRESET, preset.name.as_bytes(), &[preset.bump]]];

    close_account(CpiContext::new_with_signer(
        token_extensions_program.to_account_info(),
        CloseAccount {
            account: mint.to_account_info(),
            authority: preset.to_account_info(),
            destination: fee_payer.to_account_info(),
        },
        signer_seeds,
    ))?;

    preset.close(fee_payer.to_account_info())?;

    Ok(())
}
