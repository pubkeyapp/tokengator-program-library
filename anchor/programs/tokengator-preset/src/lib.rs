#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

pub use instructions::*;

declare_id!("TPLx9nrMKbk3uQoHHL2pxepyYx4A4RH58EAPqsYE9ir");

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
        preset::mint(ctx)
    }

    pub fn remove_preset(ctx: Context<RemovePreset>) -> Result<()> {
        preset::remove(ctx)
    }
}
