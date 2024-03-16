use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::*;
use crate::state::*;

#[derive(Accounts)]
pub struct RemovePresetAuthority<'info> {
    #[account(
      mut,
      seeds = [
        PREFIX,
        PRESET,
        &preset.name.as_bytes()
      ],
      bump = preset.bump,
      has_one = fee_payer @ TokenGatorPresetError::UnAuthorized,
      constraint = preset.check_for_authority(&authority.key()) @ TokenGatorPresetError::UnAuthorized
    )]
    pub preset: Account<'info, Preset>,

    #[account(
      mut,
      constraint = fee_payer.key().ne(&authority.key()) @ TokenGatorPresetError::InvalidFeePayer
    )]
    pub fee_payer: Signer<'info>,

    pub authority: Signer<'info>,
}

pub fn remove_authority(
    ctx: Context<RemovePresetAuthority>,
    args: RemovePresetAuthorityArgs,
) -> Result<()> {
    let preset = &mut ctx.accounts.preset;
    let authority_to_remove = args.authority_to_remove;

    require!(
        preset.authorities.len() > 1,
        TokenGatorPresetError::CannotRemoveSoloAuthority
    );

    match preset.authorities.binary_search(&authority_to_remove) {
        Ok(authority_to_remove_index) => {
            preset.authorities.remove(authority_to_remove_index);
        }
        Err(_) => return err!(TokenGatorPresetError::AuthorityNonExistant),
    }

    preset.validate()?;

    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct RemovePresetAuthorityArgs {
    pub authority_to_remove: Pubkey,
}
