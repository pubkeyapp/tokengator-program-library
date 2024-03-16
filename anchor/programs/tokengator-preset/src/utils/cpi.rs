use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token_2022::spl_token_2022::{extension::*, instruction::*};
use spl_token_metadata_interface::state::Field;

#[derive(Accounts)]
pub struct InitializeMintNonTransferrable<'info> {
    /// CHECK: CPI Account
    pub mint: AccountInfo<'info>,
}

pub fn initialize_mint_non_transferable<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, InitializeMintNonTransferrable<'info>>,
) -> Result<()> {
    let ix = initialize_non_transferable_mint(ctx.program.key, ctx.accounts.mint.key)?;

    solana_program::program::invoke(&ix, &[ctx.accounts.mint]).map_err(Into::into)
}

#[derive(Accounts)]
pub struct InitializeInterestBearingMint<'info> {
    /// CHECK: CPI Account
    pub mint: AccountInfo<'info>,
}

pub fn initialize_interest_bearing_mint<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, InitializeInterestBearingMint<'info>>,
    rate_authority: Option<Pubkey>,
    rate: i16,
) -> Result<()> {
    let ix = interest_bearing_mint::instruction::initialize(
        ctx.program.key,
        ctx.accounts.mint.key,
        rate_authority,
        rate,
    )?;

    solana_program::program::invoke(&ix, &[ctx.accounts.mint]).map_err(Into::into)
}

#[derive(Accounts)]
pub struct InitializeMetadataPointer<'info> {
    /// CHECK: CPI Account
    pub mint: AccountInfo<'info>,
}

pub fn initialize_metadata_pointer<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, InitializeMetadataPointer<'info>>,
    authority: Option<Pubkey>,
    metadata_address: Option<Pubkey>,
) -> Result<()> {
    let ix = metadata_pointer::instruction::initialize(
        ctx.program.key,
        ctx.accounts.mint.key,
        authority,
        metadata_address,
    )?;

    solana_program::program::invoke(&ix, &[ctx.accounts.mint]).map_err(Into::into)
}

#[derive(Accounts)]
pub struct InitializeTransferFeeConfig<'info> {
    /// CHECK: CPI Account
    pub mint: AccountInfo<'info>,
}

pub fn initialize_transfer_fee_config<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, InitializeTransferFeeConfig<'info>>,
    transfer_fee_config_authority: Option<&Pubkey>,
    withdraw_withheld_authority: Option<&Pubkey>,
    transfer_fee_basis_points: u16,
    maximum_fee: u64,
) -> Result<()> {
    let ix = transfer_fee::instruction::initialize_transfer_fee_config(
        ctx.program.key,
        ctx.accounts.mint.key,
        transfer_fee_config_authority,
        withdraw_withheld_authority,
        transfer_fee_basis_points,
        maximum_fee,
    )?;

    solana_program::program::invoke(&ix, &[ctx.accounts.mint]).map_err(Into::into)
}

#[derive(Accounts)]
pub struct InitializeMetadata<'info> {
    /// CHECK: CPI Account
    pub metadata: AccountInfo<'info>,
    /// CHECK: CPI Account
    pub update_authority: AccountInfo<'info>,
    /// CHECK: CPI Account
    pub mint: AccountInfo<'info>,
    /// CHECK: CPI Account
    pub mint_authority: AccountInfo<'info>,
}

pub fn intialize_metadata<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, InitializeMetadata<'info>>,
    name: String,
    symbol: String,
    uri: String,
) -> Result<()> {
    let ix = spl_token_metadata_interface::instruction::initialize(
        ctx.program.key,
        ctx.accounts.metadata.key,
        ctx.accounts.update_authority.key,
        ctx.accounts.mint.key,
        ctx.accounts.mint_authority.key,
        name,
        symbol,
        uri,
    );

    solana_program::program::invoke(
        &ix,
        &[
            ctx.accounts.metadata,
            ctx.accounts.update_authority,
            ctx.accounts.mint,
            ctx.accounts.mint_authority,
        ],
    )
    .map_err(Into::into)
}

#[derive(Accounts)]
pub struct UpdateMetadataField<'info> {
    /// CHECK: CPI Account
    pub metadata: AccountInfo<'info>,
    /// CHECK: CPI Account
    pub update_authority: AccountInfo<'info>,
}

pub fn update_metadata_field<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, UpdateMetadataField<'info>>,
    field: String,
    value: String,
) -> Result<()> {
    let ix = spl_token_metadata_interface::instruction::update_field(
        ctx.program.key,
        ctx.accounts.metadata.key,
        ctx.accounts.update_authority.key,
        Field::Key(field),
        value,
    );

    solana_program::program::invoke(&ix, &[ctx.accounts.metadata, ctx.accounts.update_authority])
        .map_err(Into::into)
}
