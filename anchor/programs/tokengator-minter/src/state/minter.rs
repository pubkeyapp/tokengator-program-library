use crate::constants::*;
use crate::errors::*;
use crate::state::*;
use crate::utils::*;

use anchor_lang::prelude::*;

#[account]
pub struct Minter {
    // Bump of the PDA
    pub bump: u8,
    // Community ID
    pub community_id: Pubkey,
    // WNS Group
    pub group: Pubkey,
    // Name of the Minter
    pub name: String,
    // Description about the Minter
    pub description: String,
    // Image URL of the Minter
    pub image_url: String,
    // Remote fee payer
    pub fee_payer: Pubkey,
    // Authorities that have been delegated to
    pub authorities: Vec<Pubkey>,
    // Payment configuration for this minter instance
    pub payment_config: PaymentConfig,
    // Identities user have added onto
    pub minter_config: MinterConfig,
}

impl Minter {
    pub fn size(
        authorities: &[Pubkey],
        application_config: &MinterApplicationConfig,
        metadata_config: &MinterMetadataConfig,
    ) -> usize {
        let authorities_size = 4 + // Vector discriminator
        (authorities.len() * 32); // Total authorities pubkey length

        let payment_config_size = PaymentConfig::size();
        let minter_config_size = MinterConfig::size(application_config, metadata_config);

        8 + // Anchor discriminator
        1 + // bump
        32 + // community_id
        32 + // group
        MAX_NAME_SIZE + // name
        MAX_DESCRIPTION_SIZE + // description
        MAX_IMAGE_URL_SIZE + // image_url
        32 + // fee_payer
        authorities_size + // authorities
        8 + // payment_expires_at
        payment_config_size + // payment_config
        minter_config_size // minter_config
    }

    pub fn validate(&self) -> Result<()> {
        let image_url_len = self.image_url.len();
        let description_len = self.description.len();
        let authorities_len = self.authorities.len();

        // Name
        require!(
            is_valid_username(&self.name),
            TokenGatorMinterError::InvalidMinterName
        );

        // Description
        require!(
            description_len > 10 && description_len <= MAX_DESCRIPTION_SIZE,
            TokenGatorMinterError::InvalidMinterDescription
        );

        // Image URL
        require!(
            is_valid_url(&self.image_url),
            TokenGatorMinterError::InvalidMinterImageURL
        );

        require!(
            image_url_len > 0 && image_url_len <= MAX_IMAGE_URL_SIZE,
            TokenGatorMinterError::InvalidMinterImageURL
        );

        // Authorities
        require!(
            authorities_len <= MAX_VECTOR_SIZE.into(),
            TokenGatorMinterError::MaxSizeReached
        );

        // Payment config
        self.payment_config.validate()?;

        // Minter config
        self.minter_config.validate()?;

        Ok(())
    }

    pub fn check_for_authority(&self, authority: &Pubkey) -> bool {
        self.authorities.binary_search(authority).is_ok()
    }
}
