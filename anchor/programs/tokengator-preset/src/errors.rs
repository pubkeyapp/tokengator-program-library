use anchor_lang::prelude::*;

#[error_code]
pub enum TokenGatorPresetError {
    #[msg("Account not owned by program")]
    InvalidAccountOwner,
    #[msg("Account unauthorized to perform this action")]
    UnAuthorized,
    #[msg("Authority already exists")]
    AuthorityAlreadyExists,
    #[msg("Authority does not exist")]
    AuthorityNonExistant,
    #[msg("Cannot remove last remaining authority")]
    CannotRemoveSoloAuthority,
    #[msg("Invalid preset name")]
    InvalidPresetName,
    #[msg("Invalid preset description")]
    InvalidPresetDescription,
    #[msg("Invalid Image Url")]
    InvalidPresetImageURL,
    #[msg("Array reached max size")]
    MaxSizeReached,
}
