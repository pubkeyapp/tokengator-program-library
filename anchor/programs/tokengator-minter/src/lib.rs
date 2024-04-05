#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

pub use instructions::*;

declare_id!("gatorxhhdfsTpfkgB2QM4qwkwgSem2UgzPXLaD2ZGoY");

#[program]
pub mod tokengator_minter {
    use super::*;

    pub fn create_minter(ctx: Context<CreateMinter>, args: CreateMinterArgs) -> Result<()> {
        minter::create(ctx, args)
    }

    pub fn create_minter_wns(
        ctx: Context<CreateMinterWNS>,
        args: CreateMinterWNSArgs,
    ) -> Result<()> {
        minter::create_wns(ctx, args)
    }

    // pub fn add_preset_authority(
    //     ctx: Context<AddPresetAuthority>,
    //     args: AddPresetAuthorityArgs,
    // ) -> Result<()> {
    //     minter::add_authority(ctx, args)
    // }

    // pub fn remove_preset_authority(
    //     ctx: Context<RemovePresetAuthority>,
    //     args: RemovePresetAuthorityArgs,
    // ) -> Result<()> {
    //     minter::remove_authority(ctx, args)
    // }

    pub fn mint_minter_wns(ctx: Context<MintMinterWNS>, args: MintMinterWNSArgs) -> Result<()> {
        minter::mint_wns(ctx, args)
    }

    // pub fn remove_preset(ctx: Context<RemovePreset>) -> Result<()> {
    //     minter::remove(ctx)
    // }
}
