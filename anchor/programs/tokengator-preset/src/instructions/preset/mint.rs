use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct MintPreset<'info> {
    pub system_program: Program<'info, System>,
}
