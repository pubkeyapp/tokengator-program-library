use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::*;
use crate::state::*;
use crate::utils::*;

#[derive(Accounts)]
pub struct AddPresetAuthority<'info> {
    #[account(
      mut,
      seeds = [
        PREFIX,
        PRESET,
        &preset.name.as_bytes()
      ],
      bump = preset.bump,
      constraint = preset.check_for_authority(&authority.key()) @ TokenGatorPresetError::UnAuthorized
    )]
    pub preset: Account<'info, Preset>,

    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn add_authority(ctx: Context<AddPresetAuthority>, args: AddPresetAuthorityArgs) -> Result<()> {
    let preset = &mut ctx.accounts.preset;

    let authority = &mut ctx.accounts.authority;
    let system_program = &ctx.accounts.system_program;

    let new_authority = args.new_authority;

    match preset.authorities.binary_search(&new_authority) {
        Ok(_) => return err!(TokenGatorPresetError::AuthorityAlreadyExists),
        Err(new_authority_index) => preset
            .authorities
            .insert(new_authority_index, new_authority),
    }

    let new_preset_size = Preset::size(&preset.authorities, &preset.minter_config.metadata_config);

    realloc_account(
        preset.to_account_info(),
        new_preset_size,
        authority.to_account_info(),
        system_program.to_account_info(),
    )?;

    preset.validate()?;

    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct AddPresetAuthorityArgs {
    pub new_authority: Pubkey,
}
