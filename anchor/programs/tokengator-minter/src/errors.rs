use anchor_lang::prelude::*;

#[error_code]
pub enum TokenGatorMinterError {
    #[msg("Account not owned by program")]
    InvalidAccountOwner,
    #[msg("Invalid Fee payer")]
    InvalidFeePayer,
    #[msg("Account unauthorized to perform this action")]
    UnAuthorized,
    #[msg("Authority already exists")]
    AuthorityAlreadyExists,
    #[msg("Authority does not exist")]
    AuthorityNonExistant,
    #[msg("Cannot remove last remaining authority")]
    CannotRemoveSoloAuthority,
    #[msg("Invalid minter token account")]
    InvalidMinterTokenAccount,
    #[msg("Invalid minter name")]
    InvalidMinterName,
    #[msg("Invalid minter description")]
    InvalidMinterDescription,
    #[msg("Invalid Image Url")]
    InvalidMinterImageURL,
    #[msg("Array reached max size")]
    MaxSizeReached,
    #[msg("Invalid mint account passed")]
    InvalidMint,
    #[msg("Token extensions program required")]
    InvalidTokenProgram,
    #[msg("Cannot remove minter of non-zero supply")]
    CannotRemoveNonZeroSupplyMinter,
}
