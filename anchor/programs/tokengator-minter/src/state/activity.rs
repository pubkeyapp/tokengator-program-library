use anchor_lang::prelude::*;

use crate::constants::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Entry {
    pub timestamp: i64,
    pub message: String,
    pub url: Option<String>,
    pub points: u64,
}

impl Entry {
    pub fn size() -> usize {
        8 + // timestamp
        MAX_ENTRY_MESSAGE_SIZE + // message
        1 + MAX_ENTRY_URL_SIZE + // url
        8 // points
    }

    pub fn validate(&self) -> Result<()> {
        // TODO: Implement validation
        Ok(())
    }
}

#[account]
pub struct Activity {
    pub bump: u8,
    pub label: String,
    pub start_date: i64,
    pub end_date: i64,
    pub fee_payer: Pubkey,
    pub minter: Pubkey,
    pub member: Pubkey,
    pub mint: Pubkey,
    pub entries: Vec<Entry>,
}

impl Activity {
    pub fn size(entries: &[Entry]) -> usize {
        8 + // anchor discriminator
        1 + // bump
        MAX_LABEL_SIZE + // label
        8 + // start_date
        8 + // end_date
        32 + // fee_payer
        32 + // minter
        32 + // member
        32 + // mint
        4 + // vector discriminator
        (entries.len() * Entry::size()) // entries
    }

    pub fn validate(&self) -> Result<()> {
        // TODO: Implement validation
        for entry in &self.entries {
            entry.validate()?;
        }
        Ok(())
    }
}
