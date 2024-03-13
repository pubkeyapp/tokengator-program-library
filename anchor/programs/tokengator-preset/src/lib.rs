#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

pub use instructions::*;

declare_id!("TPLxuiUYiDdJVenaSEhshrLP9EF83MNzKhiHNoFjCPM");

#[program]
pub mod tokengator_preset {
    use super::*;

    pub fn create_preset(ctx: Context<CreatePreset>, args: CreatePresetArgs) -> Result<()> {
        preset::create(ctx, args)
    }

    pub fn add_preset_authority(
        ctx: Context<AddPresetAuthority>,
        args: AddPresetAuthorityArgs,
    ) -> Result<()> {
        preset::add_authority(ctx, args)
    }

    pub fn remove_preset_authority(
        ctx: Context<RemovePresetAuthority>,
        args: RemovePresetAuthorityArgs,
    ) -> Result<()> {
        preset::remove_authority(ctx, args)
    }

    pub fn mint_preset(ctx: Context<MintPreset>) -> Result<()> {
        Ok(())
    }
}
