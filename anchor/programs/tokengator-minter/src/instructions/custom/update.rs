use anchor_lang::prelude::*;

use crate::constants::*;
use crate::state::*;

#[derive(Accounts)]
#[instruction(args: UpdateMemberMetadataArgs)]
pub struct UpdateMemberMetadata<'info> {
    #[account(
      seeds = [
        PREFIX,
        MINTER,
        minter.minter_config.mint.as_ref(),
        minter.name.as_bytes()
      ],
      bump = minter.bump,
    )]
    pub minter: Account<'info, Minter>,
}

pub fn update(_ctx: Context<UpdateMemberMetadata>, _args: UpdateMemberMetadataArgs) -> Result<()> {
    // TODO: Implement after token group/member extensions are live

    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateMemberMetadataArgs {
    pub label: String,
    pub start_date: Option<i64>,
    pub end_date: Option<i64>,
}
