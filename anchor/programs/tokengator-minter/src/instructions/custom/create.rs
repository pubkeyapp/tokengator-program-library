use anchor_lang::{
    prelude::*,
    system_program::{self, CreateAccount},
};
use anchor_spl::{
    associated_token::{
        create as create_associated_token, get_associated_token_address_with_program_id,
        AssociatedToken, Create as CreateAssociatedToken,
    },
    token_2022::{
        initialize_mint2, initialize_mint_close_authority, mint_to, InitializeMint2,
        InitializeMintCloseAuthority, MintTo, Token2022,
    },
};
use spl_pod::optional_keys::OptionalNonZeroPubkey;
use spl_token_2022::{extension::ExtensionType, state::Mint};
use spl_token_metadata_interface::state::TokenMetadata;

/* Group extension related imports */
// use anchor_spl::token_2022::{
//     set_authority, spl_token_2022::instruction::AuthorityType, SetAuthority,
// };
// use spl_token_group_interface::state::TokenGroup;
// use spl_type_length_value::state::{TlvState, TlvStateBorrowed};

use crate::constants::*;
use crate::errors::*;
use crate::state::*;
use crate::utils::*;

#[derive(Accounts)]
#[instruction(args: CreateMinterArgs)]
pub struct CreateMinter<'info> {
    /* Custom account till group extensions become live */
    #[account(
        init,
        payer = fee_payer,
        space = Group::size(),
        seeds = [
            PREFIX,
            GROUP,
            mint.key().as_ref()
        ],
        bump
    )]
    pub group: Account<'info, Group>,

    #[account(
      init,
      payer = fee_payer,
      space = Minter::size(&[authority.key()], &args.application_config, &args.metadata_config),
      seeds = [
        PREFIX,
        MINTER,
        mint.key().as_ref(),
        &args.name.as_bytes()
      ],
      bump
    )]
    pub minter: Account<'info, Minter>,

    /// CHECK: Checks done inside the handler function
    #[account(mut)]
    pub minter_token_account: UncheckedAccount<'info>,

    #[account(mut)]
    pub mint: Signer<'info>,

    #[account(
      mut,
      constraint = fee_payer.key().ne(&authority.key()) @ TokenGatorMinterError::InvalidFeePayer
    )]
    pub fee_payer: Signer<'info>,
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn create(ctx: Context<CreateMinter>, args: CreateMinterArgs) -> Result<()> {
    let minter = &mut ctx.accounts.minter;
    let group = &mut ctx.accounts.group;

    let authority = &ctx.accounts.authority;
    let fee_payer = &ctx.accounts.fee_payer;
    let mint = &ctx.accounts.mint;
    let minter_token_account = &ctx.accounts.minter_token_account;

    let token_extensions_program = &ctx.accounts.token_program;
    let system_program = &ctx.accounts.system_program;
    let associated_token_program = &ctx.accounts.associated_token_program;

    let minter_key = minter.key();
    let mint_key = mint.key();
    let group_key = group.key();

    let community_id = fetch_community_id(&args.community);

    let CreateMinterArgs {
        metadata_config,
        transfer_fee_config,
        interest_config,
        application_config,
        payment_config,
        ..
    } = args;

    // 1. Saving Minter onchain
    let minter_config = MinterConfig {
        mint: mint.key(),
        application_config: application_config.clone(),
        interest_config: interest_config.clone(),
        metadata_config: metadata_config.clone(),
        transfer_fee_config: transfer_fee_config.clone(),
    };

    minter.set_inner(Minter {
        bump: ctx.bumps.minter,
        community_id,
        group: group_key,
        name: args.name.clone(),
        description: args.description,
        image_url: args.image_url,
        fee_payer: ctx.accounts.fee_payer.key(),
        authorities: vec![authority.key()],
        payment_config,
        minter_config,
    });

    minter.validate()?;

    // 2. Creating Mint account
    let mut mint_extension_types = vec![
        ExtensionType::MintCloseAuthority,
        ExtensionType::NonTransferable,
        ExtensionType::MetadataPointer,
        ExtensionType::GroupPointer,
    ];

    if transfer_fee_config.is_some() {
        mint_extension_types.push(ExtensionType::TransferFeeConfig);
    }

    if interest_config.is_some() {
        mint_extension_types.push(ExtensionType::InterestBearingConfig)
    }

    let additional_metdata: Vec<(String, String)> =
        if let Some(additional_metadata) = &metadata_config.metadata {
            let mut metadata_tuple: Vec<(String, String)> = vec![];
            for metadata_pair in additional_metadata {
                metadata_tuple.push((metadata_pair[0].clone(), metadata_pair[1].clone()));
            }
            metadata_tuple
        } else {
            vec![]
        };

    let metadata = TokenMetadata {
        update_authority: OptionalNonZeroPubkey::try_from(Some(minter.key())).unwrap(),
        mint: mint_key,
        name: metadata_config.name.clone(),
        symbol: metadata_config.symbol.clone(),
        uri: metadata_config.uri.clone(),
        additional_metadata: additional_metdata,
    };

    let mint_size = ExtensionType::try_calculate_account_len::<Mint>(&mint_extension_types)?;
    let metadata_size = metadata.tlv_size_of()?;
    // let group_size = TlvStateBorrowed::get_base_len() + size_of::<TokenGroup>();
    // let rent_lamports = Rent::get()?.minimum_balance(mint_size + metadata_size + group_size);

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
        Some(&minter_key),
    )?;

    initialize_mint_non_transferable(CpiContext::new(
        token_extensions_program.to_account_info(),
        InitializeMintNonTransferrable {
            mint: mint.to_account_info(),
        },
    ))?;

    initialize_metadata_pointer(
        CpiContext::new(
            token_extensions_program.to_account_info(),
            InitializeMetadataPointer {
                mint: mint.to_account_info(),
            },
        ),
        Some(minter_key),
        Some(mint_key),
    )?;

    initialize_group_pointer(
        CpiContext::new(
            token_extensions_program.to_account_info(),
            InitializeGroupPointer {
                mint: mint.to_account_info(),
            },
        ),
        Some(minter_key),
        Some(group_key),
    )?;

    if interest_config.is_some() {
        let interest_config = interest_config.unwrap();
        initialize_interest_bearing_mint(
            CpiContext::new(
                token_extensions_program.to_account_info(),
                InitializeInterestBearingMint {
                    mint: mint.to_account_info(),
                },
            ),
            Some(minter_key),
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
            Some(&minter_key),
            Some(&minter_key),
            transfer_fee_config.transfer_fee_basis_points,
            transfer_fee_config.max_fee_rate,
        )?;
    }

    // 4. Initializing mint
    let signer_seeds: &[&[&[u8]]] = &[&[
        PREFIX,
        MINTER,
        mint_key.as_ref(),
        minter.name.as_bytes(),
        &[minter.bump],
    ]];

    initialize_mint2(
        CpiContext::new_with_signer(
            token_extensions_program.to_account_info(),
            InitializeMint2 {
                mint: mint.to_account_info(),
            },
            signer_seeds,
        ),
        0,
        &minter_key,
        Some(&minter_key),
    )?;

    // 5. Initializing metadata and adding additional metadata fields
    let MinterMetadataConfig {
        name,
        symbol,
        uri,
        metadata,
        ..
    } = metadata_config;

    intialize_metadata(
        CpiContext::new_with_signer(
            token_extensions_program.to_account_info(),
            InitializeMetadata {
                metadata: mint.to_account_info(),
                mint: mint.to_account_info(),
                mint_authority: minter.to_account_info(),
                update_authority: minter.to_account_info(),
            },
            signer_seeds,
        ),
        name,
        symbol,
        uri,
    )?;

    if let Some(metadata) = metadata {
        for field_value_pair in metadata {
            update_metadata_field(
                CpiContext::new_with_signer(
                    token_extensions_program.to_account_info(),
                    UpdateMetadataField {
                        metadata: mint.to_account_info(),
                        update_authority: minter.to_account_info(),
                    },
                    signer_seeds,
                ),
                field_value_pair[0].clone(),
                field_value_pair[1].clone(),
            )?;
        }
    }

    // 6. Initializing group
    // Setting this after Group extensions are deployed
    // initialize_group(
    //     CpiContext::new_with_signer(
    //         token_extensions_program.to_account_info(),
    //         InitializeGroup {
    //             group: mint.to_account_info(),
    //             mint: mint.to_account_info(),
    //             mint_authority: minter.to_account_info(),
    //         },
    //         signer_seeds,
    //     ),
    //     Some(minter_key.clone()),
    //     100,
    // )?;
    group.set_inner(Group {
        update_authority: minter_key,
        mint: mint_key,
        max_size: 100,
        size: 0,
    });

    // 7. Minting Collection NFT and freezing Mint authority
    let expected_minter_token_account = get_associated_token_address_with_program_id(
        &minter_key,
        &mint_key,
        token_extensions_program.key,
    );

    require_eq!(
        expected_minter_token_account,
        minter_token_account.key(),
        TokenGatorMinterError::InvalidMinterTokenAccount
    );

    create_associated_token(CpiContext::new(
        associated_token_program.to_account_info(),
        CreateAssociatedToken {
            associated_token: minter_token_account.to_account_info(),
            authority: minter.to_account_info(),
            mint: mint.to_account_info(),
            payer: fee_payer.to_account_info(),
            system_program: system_program.to_account_info(),
            token_program: token_extensions_program.to_account_info(),
        },
    ))?;

    mint_to(
        CpiContext::new_with_signer(
            token_extensions_program.to_account_info(),
            MintTo {
                authority: minter.to_account_info(),
                mint: mint.to_account_info(),
                to: minter_token_account.to_account_info(),
            },
            signer_seeds,
        ),
        1,
    )?;

    /* Uncomment this after group extensions are live */
    // set_authority(
    //     CpiContext::new_with_signer(
    //         token_extensions_program.to_account_info(),
    //         SetAuthority {
    //             account_or_mint: mint.to_account_info(),
    //             current_authority: minter.to_account_info(),
    //         },
    //         signer_seeds,
    //     ),
    //     AuthorityType::MintTokens,
    //     None,
    // )?;

    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateMinterArgs {
    pub community: String,
    pub name: String,
    pub description: String,
    pub image_url: String,
    pub payment_config: PaymentConfig,
    pub application_config: MinterApplicationConfig,
    pub metadata_config: MinterMetadataConfig,
    pub interest_config: Option<MinterInterestConfig>,
    pub transfer_fee_config: Option<MinterTransferFeeConfig>,
}
