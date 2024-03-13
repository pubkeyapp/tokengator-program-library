use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct MinterMetadataConfig {
    pub additional_metadata: Vec<[String; 2]>,
    pub image: Option<String>,
    pub name: String,
    pub symbol: String,
    pub update_authority: Option<Pubkey>,
    pub mint_authority: Option<Pubkey>,
    pub uri: Option<String>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct MinterInterestConfig {
    rate: u64,
    rate_authority: Option<Pubkey>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct MinterTransferFeeConfig {
    transfer_fee_rate: u64,
    transfer_fee_account: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct MinterConfig {
    pub authority: Option<Pubkey>,
    pub decimals: Option<u8>,
    pub fee_payer: Option<Pubkey>,
    pub mint: Option<Pubkey>,
    pub close_authority: Option<Pubkey>,
    pub freeze_authority: Option<Pubkey>,
    pub metadata_config: Option<MinterMetadataConfig>,
    pub interest_config: Option<MinterInterestConfig>,
    pub non_transferrable_config: Option<MinterTransferFeeConfig>,
}

impl MinterConfig {
    pub fn size() -> usize {
        8 + // anchor discriminator
        0
    }
}
