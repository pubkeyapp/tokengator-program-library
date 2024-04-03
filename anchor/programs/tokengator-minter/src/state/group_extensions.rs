// Custom group account till group extensions are live

use anchor_lang::prelude::*;

#[account]
pub struct Group {
    pub update_authority: Pubkey,
    pub mint: Pubkey,
    pub size: u32,
    pub max_size: u32,
}

impl Group {
    pub fn size() -> usize {
        8 + // anchor discriminator
        32 + // update_authority
        32 + // mint
        4 + // size
        4 // max_size
    }
}
