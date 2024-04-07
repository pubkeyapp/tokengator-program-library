use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::*;
use crate::state::*;
use crate::utils::realloc_account;

#[derive(Accounts)]
#[instruction(args: AppendActivityEntryArgs)]
pub struct AppendActivityEntry<'info> {
    #[account(
      mut,
      seeds = [
        PREFIX,
        ACTIVITY,
        activity.mint.as_ref(),
        activity.label.as_bytes(),
      ],
      bump = activity.bump,
      has_one = fee_payer @ TokenGatorMinterError::InvalidFeePayer,
    )]
    pub activity: Account<'info, Activity>,

    #[account(mut)]
    pub fee_payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn append(ctx: Context<AppendActivityEntry>, args: AppendActivityEntryArgs) -> Result<()> {
    let fee_payer = &ctx.accounts.fee_payer;
    let system_program = &ctx.accounts.system_program;
    let activity = &mut ctx.accounts.activity;

    let timestamp = args.timestamp.unwrap_or(Clock::get()?.unix_timestamp);

    let entry = Entry {
        timestamp,
        message: args.message,
        url: args.url,
        points: args.points.unwrap_or(0),
    };

    activity.entries.push(entry);

    let new_activity_size = Activity::size(&activity.entries);
    realloc_account(
        activity.to_account_info(),
        new_activity_size,
        fee_payer.to_account_info(),
        system_program.to_account_info(),
    )?;

    activity.validate()?;

    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct AppendActivityEntryArgs {
    pub timestamp: Option<i64>,
    pub message: String,
    pub url: Option<String>,
    pub points: Option<u8>,
}
