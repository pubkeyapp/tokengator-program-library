#![allow(clippy::result_large_err)]
#![allow(ambiguous_glob_reexports)]

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
        custom::create(ctx, args)
    }

    pub fn create_minter_wns(
        ctx: Context<CreateMinterWNS>,
        args: CreateMinterWNSArgs,
    ) -> Result<()> {
        wns::create(ctx, args)
    }

    pub fn add_minter_authority(
        ctx: Context<AddMinterAuthority>,
        args: AddMinterAuthorityArgs,
    ) -> Result<()> {
        authority::add(ctx, args)
    }

    pub fn remove_minter_authority(
        ctx: Context<RemoveMinterAuthority>,
        args: RemoveMinterAuthorityArgs,
    ) -> Result<()> {
        authority::remove(ctx, args)
    }

    pub fn mint_minter_wns(ctx: Context<MintMinterWNS>, args: MintMinterWNSArgs) -> Result<()> {
        wns::mint(ctx, args)
    }

    pub fn remove_minter(ctx: Context<RemoveMinter>) -> Result<()> {
        custom::remove(ctx)
    }
}
