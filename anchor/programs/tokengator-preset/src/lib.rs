#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("TPLxuiUYiDdJVenaSEhshrLP9EF83MNzKhiHNoFjCPM");

#[program]
pub mod tokengator_preset {
    use super::*;

  pub fn close(_ctx: Context<CloseTokengatorPreset>) -> Result<()> {
    Ok(())
  }

  pub fn decrement(ctx: Context<Update>) -> Result<()> {
    ctx.accounts.tokengator_preset.count = ctx.accounts.tokengator_preset.count.checked_sub(1).unwrap();
    Ok(())
  }

  pub fn increment(ctx: Context<Update>) -> Result<()> {
    ctx.accounts.tokengator_preset.count = ctx.accounts.tokengator_preset.count.checked_add(1).unwrap();
    Ok(())
  }

  pub fn initialize(_ctx: Context<InitializeTokengatorPreset>) -> Result<()> {
    Ok(())
  }

  pub fn set(ctx: Context<Update>, value: u8) -> Result<()> {
    ctx.accounts.tokengator_preset.count = value.clone();
    Ok(())
  }
}

#[derive(Accounts)]
pub struct InitializeTokengatorPreset<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
  init,
  space = 8 + TokengatorPreset::INIT_SPACE,
  payer = payer
  )]
  pub tokengator_preset: Account<'info, TokengatorPreset>,
  pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct CloseTokengatorPreset<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
  mut,
  close = payer, // close account and return lamports to payer
  )]
  pub tokengator_preset: Account<'info, TokengatorPreset>,
}

#[derive(Accounts)]
pub struct Update<'info> {
  #[account(mut)]
  pub tokengator_preset: Account<'info, TokengatorPreset>,
}

#[account]
#[derive(InitSpace)]
pub struct TokengatorPreset {
  count: u8,
}
