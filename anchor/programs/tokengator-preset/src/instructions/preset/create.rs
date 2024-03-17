use anchor_lang::{
    prelude::*,
    system_program::{self, CreateAccount},
};
use anchor_spl::token_2022::{
    initialize_mint2, initialize_mint_close_authority,
    spl_token_2022::{extension::*, state::Mint},
    InitializeMint2, InitializeMintCloseAuthority, Token2022,
};
use spl_pod::optional_keys::OptionalNonZeroPubkey;
use spl_token_metadata_interface::state::TokenMetadata;

use crate::errors::*;
use crate::state::*;
use crate::{constants::*, utils::cpi::*};

#[derive(Accounts)]
#[instruction(args: CreatePresetArgs)]
pub struct CreatePreset<'info> {
    #[account(
      init,
      payer = fee_payer,
      space = Preset::size(&[authority.key()], &args.metadata_config),
      seeds = [
        PREFIX,
        PRESET,
        &args.name.as_bytes()
      ],
      bump
    )]
    pub preset: Account<'info, Preset>,

    #[account(mut)]
    pub mint: Signer<'info>,

    #[account(
      mut,
      constraint = fee_payer.key().ne(&authority.key()) @ TokenGatorPresetError::InvalidFeePayer
    )]
    pub fee_payer: Signer<'info>,
    pub authority: Signer<'info>,

    pub token_extensions_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

pub fn create(ctx: Context<CreatePreset>, args: CreatePresetArgs) -> Result<()> {
    let preset = &mut ctx.accounts.preset;

    let authority = &ctx.accounts.authority;
    let fee_payer = &ctx.accounts.fee_payer;
    let mint = &ctx.accounts.mint;

    let token_extensions_program = &ctx.accounts.token_extensions_program;
    let system_program = &ctx.accounts.system_program;

    let CreatePresetArgs {
        metadata_config,
        transfer_fee_config,
        interest_config,
        ..
    } = args;

    // 1. Saving preset onchain
    let minter_config = MinterConfig {
        mint: mint.key(),
        decimals: args.decimals,
        interest_config: interest_config.clone(),
        metadata_config: metadata_config.clone(),
        transfer_fee_config: transfer_fee_config.clone(),
    };

    preset.set_inner(Preset {
        bump: ctx.bumps.preset,
        authorities: vec![authority.key()],
        description: args.description,
        name: args.name,
        image_url: args.image_url,
        fee_payer: ctx.accounts.fee_payer.key(),
        minter_config,
    });

    preset.validate()?;

    // 2. Creating Mint account
    let mut mint_extension_types = vec![
        ExtensionType::MintCloseAuthority,
        ExtensionType::NonTransferable,
    ];

    if metadata_config.is_some() {
        mint_extension_types.push(ExtensionType::MetadataPointer);
    }

    if transfer_fee_config.is_some() {
        mint_extension_types.push(ExtensionType::TransferFeeConfig);
    }

    if interest_config.is_some() {
        mint_extension_types.push(ExtensionType::InterestBearingConfig)
    }

    let mint_size = ExtensionType::try_calculate_account_len::<Mint>(&mint_extension_types)?;

    let metadata_size = if let Some(metadata_config) = metadata_config.clone() {
        let additional_metdata: Vec<(String, String)> =
            if let Some(additional_metadata) = metadata_config.metadata {
                let mut metadata_tuple: Vec<(String, String)> = vec![];
                for metadata_pair in additional_metadata {
                    metadata_tuple.push((metadata_pair[0].clone(), metadata_pair[1].clone()));
                }
                metadata_tuple
            } else {
                vec![]
            };

        let metadata = TokenMetadata {
            update_authority: OptionalNonZeroPubkey::try_from(Some(fee_payer.key())).unwrap(),
            mint: mint.key(),
            name: metadata_config.name,
            symbol: metadata_config.symbol,
            uri: metadata_config.uri,
            additional_metadata: additional_metdata,
        };

        metadata.tlv_size_of()
    } else {
        Ok(0)
    }?;

    let rent_lamports = Rent::get()?.minimum_balance(mint_size + metadata_size);

    system_program::create_account(
        CpiContext::new(
            system_program.to_account_info(),
            CreateAccount {
                from: fee_payer.to_account_info(),
                to: mint.to_account_info(),
            },
        ),
        rent_lamports,
        u64::try_from(mint_size).unwrap(),
        token_extensions_program.key,
    )?;

    // 3. Initializing extensions
    initialize_mint_close_authority(
        CpiContext::new(
            token_extensions_program.to_account_info(),
            InitializeMintCloseAuthority {
                mint: mint.to_account_info(),
            },
        ),
        Some(fee_payer.key),
    )?;

    initialize_mint_non_transferable(CpiContext::new(
        token_extensions_program.to_account_info(),
        InitializeMintNonTransferrable {
            mint: mint.to_account_info(),
        },
    ))?;

    if interest_config.is_some() {
        let interest_config = interest_config.unwrap();
        initialize_interest_bearing_mint(
            CpiContext::new(
                token_extensions_program.to_account_info(),
                InitializeInterestBearingMint {
                    mint: mint.to_account_info(),
                },
            ),
            Some(fee_payer.key()),
            interest_config.rate,
        )?;
    }

    if transfer_fee_config.is_some() {
        let transfer_fee_config = transfer_fee_config.unwrap();

        initialize_transfer_fee_config(
            CpiContext::new(
                token_extensions_program.to_account_info(),
                InitializeTransferFeeConfig {
                    mint: mint.to_account_info(),
                },
            ),
            Some(fee_payer.key),
            Some(fee_payer.key),
            transfer_fee_config.transfer_fee_basis_points,
            transfer_fee_config.max_fee_rate,
        )?;
    }

    if metadata_config.is_some() {
        initialize_metadata_pointer(
            CpiContext::new(
                token_extensions_program.to_account_info(),
                InitializeMetadataPointer {
                    mint: mint.to_account_info(),
                },
            ),
            Some(fee_payer.key()),
            Some(mint.key()),
        )?;
    }

    // 4. Initializing mint
    initialize_mint2(
        CpiContext::new(
            token_extensions_program.to_account_info(),
            InitializeMint2 {
                mint: mint.to_account_info(),
            },
        ),
        args.decimals,
        fee_payer.key,
        Some(fee_payer.key),
    )?;

    // 5. Initializing metadata and adding additional metadata fields
    if metadata_config.is_some() {
        let MinterMetadataConfig {
            name,
            symbol,
            uri,
            metadata,
            ..
        } = metadata_config.unwrap();

        intialize_metadata(
            CpiContext::new(
                token_extensions_program.to_account_info(),
                InitializeMetadata {
                    metadata: mint.to_account_info(),
                    mint: mint.to_account_info(),
                    mint_authority: fee_payer.to_account_info(),
                    update_authority: fee_payer.to_account_info(),
                },
            ),
            name,
            symbol,
            uri,
        )?;

        if let Some(metadata) = metadata {
            for field_value_pair in metadata {
                update_metadata_field(
                    CpiContext::new(
                        token_extensions_program.to_account_info(),
                        UpdateMetadataField {
                            metadata: mint.to_account_info(),
                            update_authority: fee_payer.to_account_info(),
                        },
                    ),
                    field_value_pair[0].clone(),
                    field_value_pair[1].clone(),
                )?;
            }
        }
    }

    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreatePresetArgs {
    pub name: String,
    pub description: String,
    pub image_url: String,
    pub decimals: u8,
    pub metadata_config: Option<MinterMetadataConfig>,
    pub interest_config: Option<MinterInterestConfig>,
    pub transfer_fee_config: Option<MinterTransferFeeConfig>,
}
