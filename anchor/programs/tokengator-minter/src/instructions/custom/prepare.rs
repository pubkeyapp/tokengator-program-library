use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::{transfer_checked, Token2022, TransferChecked},
    token_interface::{Mint, TokenAccount},
};

use crate::constants::*;
use crate::state::*;

#[derive(Accounts)]
#[instruction(args: PrepareForPaymentArgs)]
pub struct PrepareForPayment<'info> {
    #[account(
      init,
      space = Receipt::size(),
      payer = fee_payer,
      seeds = [
        PREFIX,
        RECEIPT,
        sender.key().as_ref(),
        receiver.key().as_ref(),
        mint.key().as_ref(),
      ],
      bump,
    )]
    pub receipt: Account<'info, Receipt>,

    #[account(mut)]
    pub fee_payer: Signer<'info>,

    #[account(mut)]
    pub sender: Signer<'info>,

    #[account(mut, token::mint = mint)]
    pub sender_token_account: InterfaceAccount<'info, TokenAccount>,

    pub receiver: SystemAccount<'info>,

    #[account(
      init_if_needed,
      payer = sender,
      associated_token::mint = mint,
      associated_token::authority = receiver,
      associated_token::token_program = token_program
    )]
    pub receiver_token_account: InterfaceAccount<'info, TokenAccount>,

    pub mint: InterfaceAccount<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

pub fn prepare(ctx: Context<PrepareForPayment>, args: PrepareForPaymentArgs) -> Result<()> {
    let token_extensions_program = &ctx.accounts.token_program;

    let sender = &ctx.accounts.sender;
    let receiver = &ctx.accounts.receiver;
    let receiver_token_account = &ctx.accounts.receiver_token_account;
    let sender_token_account = &ctx.accounts.sender_token_account;
    let mint = &ctx.accounts.mint;

    let receipt = &mut ctx.accounts.receipt;

    let created_at = Clock::get()?.unix_timestamp;

    receipt.set_inner(Receipt {
        bump: ctx.bumps.receipt,
        payment_type: args.payment_type,
        created_at,
        sender: sender.key(),
        receiver: receiver.key(),
        payment_amount: args.payment_amount,
        sender_token_account: sender_token_account.key(),
        receiver_token_account: receiver_token_account.key(),
        payment_mint: mint.key(),
    });

    transfer_checked(
        CpiContext::new(
            token_extensions_program.to_account_info(),
            TransferChecked {
                authority: sender.to_account_info(),
                mint: mint.to_account_info(),
                from: sender_token_account.to_account_info(),
                to: receiver_token_account.to_account_info(),
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
    pub payment_type: ReceiptType,
}
