use anchor_lang::prelude::*;

use crate::constants::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum IdentityProvider {
    Discord = 0,
    GitHub = 1,
    Google = 2,
    Twitter = 3,
}

impl IdentityProvider {
    pub fn get_provider(&self) -> String {
        match self {
            IdentityProvider::Discord => "Discord".to_owned(),
            IdentityProvider::GitHub => "GitHub".to_owned(),
            IdentityProvider::Google => "Google".to_owned(),
            IdentityProvider::Twitter => "Twitter".to_owned(),
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct PaymentConfig {
    pub amount: u16,
    pub price: u64,
    pub mint: Pubkey,
    pub days: u8,
    pub expires_at: i64,
}

impl PaymentConfig {
    pub fn size() -> usize {
        2 + // amount
        8 + // price
        32 + // mint
        1 // days
    }

    pub fn validate(&self) -> Result<()> {
        // TODO: Implement Validation
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct MinterMetadataConfig {
    pub name: String,
    pub symbol: String,
    pub metadata: Option<Vec<[String; 2]>>,
    pub uri: String,
}

impl MinterMetadataConfig {
    pub fn size(metadata: &Option<Vec<[String; 2]>>) -> usize {
        let metadata_size: usize = if let Some(metadata) = metadata {
            metadata.len() * (MAX_METADATA_KEY_VALUE_SIZE * 2)
        } else {
            0
        };

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
    pub rate: i16,
}

impl MinterInterestConfig {
    pub fn size() -> usize {
        2 // rate
    }

    pub fn validate(&self) -> Result<()> {
        // TODO: Implement validation
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct MinterTransferFeeConfig {
    pub transfer_fee_basis_points: u16,
    pub max_fee_rate: u64,
}

impl MinterTransferFeeConfig {
    pub fn size() -> usize {
        2 + // transfer_fee_basis_points
        8 // max_fee_rate
    }

    pub fn validate(&self) -> Result<()> {
        // TODO: Implement validation
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct MinterApplicationConfig {
    pub identities: Vec<IdentityProvider>,
    pub payment_config: PaymentConfig,
}

impl MinterApplicationConfig {
    pub fn size(identities: &[IdentityProvider]) -> usize {
        let payment_config_size = PaymentConfig::size();

        4 + // Vector discriminator
        (identities.len() * (1 + 1)) + // Identity
        payment_config_size // payment_config
    }

    pub fn validate(&self) -> Result<()> {
        // TODO: Implement validation
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct MinterConfig {
    pub mint: Pubkey,
    pub application_config: MinterApplicationConfig,
    pub metadata_config: MinterMetadataConfig,
    pub interest_config: Option<MinterInterestConfig>,
    pub transfer_fee_config: Option<MinterTransferFeeConfig>,
}

impl MinterConfig {
    pub fn size(
        application_config: &MinterApplicationConfig,
        metadata_config: &MinterMetadataConfig,
    ) -> usize {
        32 + // mint
        MinterApplicationConfig::size(&application_config.identities) + // application_config
        MinterMetadataConfig::size(&metadata_config.metadata) + // metadata_config
        1 + MinterInterestConfig::size() + // interest_config
        1 + MinterTransferFeeConfig::size() // transfer_fee_config
    }

    pub fn validate(&self) -> Result<()> {
        // TODO: Implement validation

        self.application_config.validate()?;
        self.metadata_config.validate()?;

        if let Some(interest_config) = &self.interest_config {
            interest_config.validate()?;
        }

        if let Some(transfer_fee_config) = &self.transfer_fee_config {
            transfer_fee_config.validate()?;
        }

        Ok(())
    }
}
