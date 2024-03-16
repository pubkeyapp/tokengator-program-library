use crate::constants::*;
use crate::errors::*;
use crate::state::*;
use crate::utils::*;

use anchor_lang::prelude::*;

#[account]
pub struct Preset {
    // Bump of the PDA
    pub bump: u8,
    // Name of the preset
    pub name: String,
    // Description about the preset
    pub description: String,
    // Image URL of the preset
    pub image_url: String,
    // Remote fee payer
    pub fee_payer: Pubkey,
    // Authorities that have been delegated to
    pub authorities: Vec<Pubkey>,
    // Identities user have added onto
    pub minter_config: MinterConfig,
}

impl Preset {
    pub fn size(authorities: &[Pubkey], metadata_config: &Option<MinterMetadataConfig>) -> usize {
        let authorities_size = 4 + // Vector discriminator
        (authorities.len() * 32); // Total authorities pubkey length

        let minter_config_size = MinterConfig::size(metadata_config);

        8 + // Anchor discriminator
        1 + // bump
        MAX_NAME_SIZE + // name
        MAX_DESCRIPTION_SIZE + // description
        MAX_IMAGE_URL_SIZE + // image_url
        32 + // fee_payer
        authorities_size + // authorities
        minter_config_size // minter_config
    }

    pub fn validate(&self) -> Result<()> {
        let image_url_len = self.image_url.len();
        let description_len = self.description.len();
        let authorities_len = self.authorities.len();

        // Name
        require!(
            is_valid_username(&self.name),
            TokenGatorPresetError::InvalidPresetName
        );

        // Description
        require!(
            description_len > 10 && description_len <= MAX_DESCRIPTION_SIZE,
            TokenGatorPresetError::InvalidPresetDescription
        );

        // Image URL
        require!(
            is_valid_url(&self.image_url),
            TokenGatorPresetError::InvalidPresetImageURL
        );

        require!(
            image_url_len > 0 && image_url_len <= MAX_IMAGE_URL_SIZE,
            TokenGatorPresetError::InvalidPresetImageURL
        );

        // Authorities
        require!(
            authorities_len <= MAX_VECTOR_SIZE.into(),
            TokenGatorPresetError::MaxSizeReached
        );

        // Minter config
        self.minter_config.validate()?;

        Ok(())
    }

    pub fn check_for_authority(&self, authority: &Pubkey) -> bool {
        self.authorities.binary_search(authority).is_ok()
    }
}
