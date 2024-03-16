use anchor_lang::prelude::*;

use crate::constants::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct MinterMetadataConfig {
    pub image: Option<String>,
    pub name: String,
    pub symbol: String,
    pub metadata: Option<Vec<[String; 2]>>,
    pub uri: Option<String>,
    // pub update_authority: Option<Pubkey>,
    // pub mint_authority: Option<Pubkey>,
}

impl MinterMetadataConfig {
    pub fn size(metadata: &Option<Vec<[String; 2]>>) -> usize {
        let metadata_size: usize = if let Some(metadata) = metadata {
            metadata.len() * (MAX_METADATA_KEY_VALUE_SIZE * 2)
        } else {
            0
        };

        1 + MAX_IMAGE_URL_SIZE + // image
        MAX_NAME_SIZE + // name
        MAX_SYMBOL_SIZE + // symbol
        1 + (4 + metadata_size) + // metadata
        MAX_URI_SIZE // uri
    }

    pub fn validate(&self) -> Result<()> {
        // TODO: Implement validation
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct MinterInterestConfig {
    rate: u64,
    rate_authority: Option<Pubkey>,
}

impl MinterInterestConfig {
    pub fn size() -> usize {
        8 + // rate
        1 + 32 // rate_authority
    }

    pub fn validate(&self) -> Result<()> {
        // TODO: Implement validation
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct MinterTransferFeeConfig {
    transfer_fee_rate: u64,
    transfer_fee_account: Pubkey,
}

impl MinterTransferFeeConfig {
    pub fn size() -> usize {
        8 + // transfer_fee_rate
        32 // transfer_fee_account
    }

    pub fn validate(&self) -> Result<()> {
        // TODO: Implement validation
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct MinterConfig {
    pub mint: Pubkey,
    pub decimals: u8,
    pub fee_payer: Pubkey,
    // pub authority: Option<Pubkey>,
    // pub close_authority: Option<Pubkey>,
    pub freeze_authority: Option<Pubkey>,
    pub metadata_config: Option<MinterMetadataConfig>,
    pub interest_config: Option<MinterInterestConfig>,
    pub transfer_fee_config: Option<MinterTransferFeeConfig>,
}

impl MinterConfig {
    pub fn size(metadata_config: &Option<MinterMetadataConfig>) -> usize {
        let metadata = if let Some(metadata_config) = metadata_config {
            &metadata_config.metadata
        } else {
            &None
        };

        8 + // anchor discriminator
        32 + // mint
        1 + // decimals
        32 + // fee_payer
        1 + 32 + // freeze_authority
        1 + MinterMetadataConfig::size(metadata) + // metadata_config
        1 + MinterInterestConfig::size() + // interest_config
        1 + MinterTransferFeeConfig::size() // transfer_fee
    }

    pub fn validate(&self) -> Result<()> {
        // TODO: Implement validation

        if let Some(metadata_config) = &self.metadata_config {
            metadata_config.validate()?;
        }

        if let Some(interest_config) = &self.interest_config {
            interest_config.validate()?;
        }

        if let Some(transfer_fee_config) = &self.transfer_fee_config {
            transfer_fee_config.validate()?;
        }

        Ok(())
    }
}
