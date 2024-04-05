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
        MINTER,
        &minter.minter_config.mint.as_ref(),
        &minter.name.as_bytes()
      ],
      bump = minter.bump,
      has_one = fee_payer @ TokenGatorMinterError::UnAuthorized,
      constraint = minter.check_for_authority(&authority.key()) @ TokenGatorMinterError::UnAuthorized
    )]
    pub minter: Account<'info, Minter>,

    pub authority: Signer<'info>,

    #[account(
      mut,
      constraint = fee_payer.key().ne(&authority.key()) @ TokenGatorMinterError::InvalidFeePayer
    )]
    pub fee_payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn add_authority(ctx: Context<AddPresetAuthority>, args: AddPresetAuthorityArgs) -> Result<()> {
    let minter = &mut ctx.accounts.minter;
    let fee_payer = &ctx.accounts.fee_payer;
    let system_program = &ctx.accounts.system_program;

    let new_authority = args.new_authority;

    match minter.authorities.binary_search(&new_authority) {
        Ok(_) => return err!(TokenGatorMinterError::AuthorityAlreadyExists),
        Err(new_authority_index) => minter
            .authorities
            .insert(new_authority_index, new_authority),
    }

    let new_preset_size = Minter::size(
        &minter.authorities,
        &minter.minter_config.application_config,
        &minter.minter_config.metadata_config,
    );

    realloc_account(
        minter.to_account_info(),
        new_preset_size,
        fee_payer.to_account_info(),
        system_program.to_account_info(),
    )?;

    minter.validate()?;

    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct AddPresetAuthorityArgs {
    pub new_authority: Pubkey,
}
