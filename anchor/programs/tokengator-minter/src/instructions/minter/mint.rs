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
        MINTER,
        &minter.minter_config.mint.as_ref(),
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
      constraint = token_program.key().eq(&TOKEN_EXTENSIONS_PROGRAM_ID) @ TokenGatorMinterError::InvalidTokenProgram
    )]
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn mint(ctx: Context<MintPreset>) -> Result<()> {
    let minter = &ctx.accounts.minter;

    let mint = &ctx.accounts.mint;
    let destination_token_account = &ctx.accounts.authority_token_account;
    let token_extensions_program = &ctx.accounts.token_program;

    let mint_key = mint.key();

    let amount_with_decimals = 1u64
        .checked_mul(10u64.checked_pow(mint.decimals.into()).unwrap())
        .unwrap();

    let signer_seeds: &[&[&[u8]]] = &[&[
        PREFIX,
        MINTER,
        mint_key.as_ref(),
        minter.name.as_bytes(),
        &[minter.bump],
    ]];

    mint_to(
        CpiContext::new_with_signer(
            token_extensions_program.to_account_info(),
            MintTo {
                mint: mint.to_account_info(),
                authority: minter.to_account_info(),
                to: destination_token_account.to_account_info(),
            },
            signer_seeds,
        ),
        amount_with_decimals,
    )?;

    Ok(())
}
