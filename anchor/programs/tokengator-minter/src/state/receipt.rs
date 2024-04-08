use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum ReceiptType {
    User = 0,
    Community = 1,
}

#[account]
pub struct Receipt {
    pub bump: u8,
    pub payment_type: ReceiptType,
    pub created_at: i64,
    pub payment_amount: u64,
    pub sender: Pubkey,
    pub receiver: Pubkey,
    pub sender_token_account: Pubkey,
    pub receiver_token_account: Pubkey,
    pub payment_mint: Pubkey,
}

impl Receipt {
    pub fn size() -> usize {
        8 + // anchor discriminator
        1 + 1 + // payment_type
        1 + // bump
        8 + // created_at
        8 + // payment_amount
        32 + // sender
        32 + // receiver
        32 + // sender_token_account
        32 + // receiver_token_account
        32 // payment_mint
    }
}
