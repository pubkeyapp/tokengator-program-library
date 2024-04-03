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
        MINTER,
        &minter.name.as_bytes()
      ],
      bump = minter.bump,
      has_one = fee_payer @ TokenGatorMinterError::UnAuthorized,
      constraint = minter.check_for_authority(&authority.key()) @ TokenGatorMinterError::UnAuthorized,
      constraint = minter.minter_config.mint.eq(&mint.key()) @ TokenGatorMinterError::InvalidMint
    )]
    pub minter: Account<'info, Minter>,

    #[account(
      mut,
      mint::authority = minter,
      mint::freeze_authority = minter,
      mint::token_program = token_program,
      constraint = mint.supply == 0 @ TokenGatorMinterError::CannotRemoveNonZeroSupplyMinter
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub fee_payer: Signer<'info>,
    pub authority: Signer<'info>,

    #[account(
      constraint = token_program.key().eq(&TOKEN_EXTENSIONS_PROGRAM_ID) @ TokenGatorMinterError::InvalidTokenProgram
    )]
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn remove(ctx: Context<RemovePreset>) -> Result<()> {
    let fee_payer = &ctx.accounts.fee_payer;
    let minter = &ctx.accounts.minter;

    let mint = &ctx.accounts.mint;
    let token_extensions_program = &ctx.accounts.token_program;

    let signer_seeds: &[&[&[u8]]] = &[&[PREFIX, MINTER, minter.name.as_bytes(), &[minter.bump]]];

    close_account(CpiContext::new_with_signer(
        token_extensions_program.to_account_info(),
        CloseAccount {
            account: mint.to_account_info(),
            authority: minter.to_account_info(),
            destination: fee_payer.to_account_info(),
        },
        signer_seeds,
    ))?;

    minter.close(fee_payer.to_account_info())?;

    Ok(())
}
