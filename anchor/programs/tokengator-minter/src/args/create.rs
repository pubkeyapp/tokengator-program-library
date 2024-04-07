use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct PaymentConfigArgs {
    pub amount: u16,
    pub price: u64,
    pub mint: Pubkey,
    pub days: u8,
}
