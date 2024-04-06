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

    pub fn update_member_metdata(
        ctx: Context<UpdateMemberMetadata>,
        args: UpdateMemberMetadataArgs,
    ) -> Result<()> {
        custom::update(ctx, args)
    }

    pub fn update_member_metadata_wns(
        ctx: Context<UpdateMemberMetadataWNS>,
        args: UpdateMemberMetadataWNSArgs,
    ) -> Result<()> {
        wns::update(ctx, args)
    }

    pub fn remove_minter(ctx: Context<RemoveMinter>) -> Result<()> {
        custom::remove(ctx)
    }

    pub fn create_activity(ctx: Context<CreateActivity>, args: CreateActivityArgs) -> Result<()> {
        activity::create(ctx, args)
    }

    pub fn append_activity_entry(
        ctx: Context<AppendActivityEntry>,
        args: AppendActivityEntryArgs,
    ) -> Result<()> {
        activity::append(ctx, args)
    }
}
