use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;
use wen_new_standard::{TokenGroup, TokenGroupMember};

use crate::constants::*;
use crate::errors::*;
use crate::state::*;

#[derive(Accounts)]
#[instruction(args: CreateActivityArgs)]
pub struct CreateActivity<'info> {
    #[account(
      init,
      space = Activity::size(&[]),
      payer = fee_payer,
      seeds = [
        PREFIX,
        ACTIVITY,
        mint.key().as_ref(),
        args.label.as_bytes(),
      ],
      bump
    )]
    pub activity: Account<'info, Activity>,

    #[account(
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

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub fee_payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn create(ctx: Context<CreateActivity>, args: CreateActivityArgs) -> Result<()> {
    let fee_payer = &ctx.accounts.fee_payer;
    let member = &ctx.accounts.member;
    let minter = &ctx.accounts.minter;
    let mint = &ctx.accounts.mint;
    let activity = &mut ctx.accounts.activity;

    let start_date = args
        .start_date
        .unwrap_or(Clock::get().unwrap().unix_timestamp);

    let end_date = args.start_date.unwrap_or(start_date + (60 * 60 * 24 * 30));

    activity.set_inner(Activity {
        bump: ctx.bumps.activity,
        label: args.label,
        start_date,
        end_date,
        fee_payer: fee_payer.key(),
        minter: minter.key(),
        member: member.key(),
        mint: mint.key(),
        entries: vec![],
    });

    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateActivityArgs {
    pub label: String,
    pub start_date: Option<i64>,
    pub end_date: Option<i64>,
}
