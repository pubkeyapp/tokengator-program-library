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
        MINTER,
        &minter.minter_config.mint.as_ref(),
        &minter.name.as_bytes()
      ],
      bump = minter.bump,
      has_one = fee_payer @ TokenGatorMinterError::UnAuthorized,
      constraint = minter.check_for_authority(&authority.key()) @ TokenGatorMinterError::UnAuthorized
    )]
    pub minter: Account<'info, Minter>,

    #[account(
      mut,
      constraint = fee_payer.key().ne(&authority.key()) @ TokenGatorMinterError::InvalidFeePayer
    )]
    pub fee_payer: Signer<'info>,

    pub authority: Signer<'info>,
}

pub fn remove_authority(
    ctx: Context<RemovePresetAuthority>,
    args: RemovePresetAuthorityArgs,
) -> Result<()> {
    let minter = &mut ctx.accounts.minter;
    let authority_to_remove = args.authority_to_remove;

    require!(
        minter.authorities.len() > 1,
        TokenGatorMinterError::CannotRemoveSoloAuthority
    );

    match minter.authorities.binary_search(&authority_to_remove) {
        Ok(authority_to_remove_index) => {
            minter.authorities.remove(authority_to_remove_index);
        }
        Err(_) => return err!(TokenGatorMinterError::AuthorityNonExistant),
    }

    minter.validate()?;

    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct RemovePresetAuthorityArgs {
    pub authority_to_remove: Pubkey,
}
