use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::{transfer_checked, Token2022, TransferChecked},
    token_interface::{Mint, TokenAccount},
};

#[derive(Accounts)]
#[instruction(args: PrepareForPaymentArgs)]
pub struct PrepareForPayment<'info> {
    #[account(mut)]
    pub funder: Signer<'info>,

    #[account(mut, token::mint = mint)]
    pub funder_token_account: InterfaceAccount<'info, TokenAccount>,

    pub authority: SystemAccount<'info>,

    #[account(
      init,
      payer = funder,
      associated_token::mint = mint,
      associated_token::authority = authority,
      associated_token::token_program = token_program
    )]
    pub authority_token_account: InterfaceAccount<'info, TokenAccount>,

    pub mint: InterfaceAccount<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

pub fn prepare(ctx: Context<PrepareForPayment>, args: PrepareForPaymentArgs) -> Result<()> {
    let token_extensions_program = &ctx.accounts.token_program;

    let funder = &ctx.accounts.funder;
    let authority_token_account = &ctx.accounts.authority_token_account;
    let funder_token_account = &ctx.accounts.funder_token_account;
    let mint = &ctx.accounts.mint;

    transfer_checked(
        CpiContext::new(
            token_extensions_program.to_account_info(),
            TransferChecked {
                authority: funder.to_account_info(),
                from: funder_token_account.to_account_info(),
                mint: mint.to_account_info(),
                to: authority_token_account.to_account_info(),
            },
        ),
        args.payment_amount,
        mint.decimals,
    )?;

    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct PrepareForPaymentArgs {
    pub payment_amount: u64,
}
