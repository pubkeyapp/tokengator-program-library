import * as anchor from '@coral-xyz/anchor'
import { Program } from '@coral-xyz/anchor'
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram } from '@solana/web3.js'
import { TokengatorMinter } from '../target/types/tokengator_minter'
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_2022_PROGRAM_ID,
  createBurnCheckedInstruction,
  getAccount,
  getAssociatedTokenAddressSync,
  getMint,
  getTokenMetadata,
} from '@solana/spl-token'

const PREFIX = new TextEncoder().encode('tokengator_minter')
const MINTER = new TextEncoder().encode('minter')
const GROUP = new TextEncoder().encode('group')

enum IdentityProvider {
  Discord = 'Discord',
  GitHub = 'GitHub',
  Google = 'Google',
  Twitter = 'Twitter',
}

function getDefaultMetadata(metadata: { name: string; symbol: string; image?: string; mint: PublicKey }) {
  const baseUrl = `https://metadata-tool.deno.dev/metadata`

  const { image, mint, name, symbol } = metadata

  return `${baseUrl}?name=${encodeURIComponent(name)}&symbol=${encodeURIComponent(symbol)}&mint=${mint}${
    image ? `&image=${encodeURIComponent(image)}` : ''
  }`
}

function getMinterPda({ programId, name }: { name: string; programId: PublicKey }) {
  return PublicKey.findProgramAddressSync([PREFIX, MINTER, new TextEncoder().encode(name)], programId)
}

function getGroupPda({ programId, mint }: { mint: PublicKey; programId: PublicKey }) {
  return PublicKey.findProgramAddressSync([PREFIX, GROUP, mint.toBuffer()], programId)
}

describe('tokengator-minter', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)
  const remoteFeePayer = provider.wallet as anchor.Wallet
  const program = anchor.workspace.TokengatorMinter as Program<TokengatorMinter>

  const authority = Keypair.generate()
  const authority2 = Keypair.generate()
  const mintKeypair = Keypair.generate()

  beforeAll(async () => {
    console.log('Airdropping authority 1 SOL:', authority.publicKey.toString())
    await provider.connection.confirmTransaction({
      ...(await provider.connection.getLatestBlockhash('confirmed')),
      signature: await provider.connection.requestAirdrop(authority.publicKey, 1 * LAMPORTS_PER_SOL),
    })
  })

  it('Create Business Visa TokengatorMinter', async () => {
    const [minter, minterBump] = getMinterPda({ name: 'Business Visa', programId: program.programId })
    const [group, groupPda] = getGroupPda({ mint: mintKeypair.publicKey, programId: program.programId })

    const minterTokenAccount = getAssociatedTokenAddressSync(
      mintKeypair.publicKey,
      minter,
      true,
      TOKEN_2022_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID,
    )

    const {
      name,
      description,
      imageUrl,
      paymentConfig,
      minterConfig: {
        applicationConfig: { paymentConfig: appPaymentConfig, identities },
        metadataConfig,
      },
    } = {
      name: 'Business Visa',
      description:
        'The Business Visa preset is a preset that is used for business purposes. The end user pays to obtain the document and it expires after a certain period of time.',
      imageUrl: `https://devnet.tokengator.app/api/preset/business-visa.png`,
      // NEW: We add a payment_config field to the preset
      paymentConfig: {
        // The community can mint 100 Business Visa documents
        amount: 100,
        // At a price of 0.1 SOL
        price: 0.1 * LAMPORTS_PER_SOL,
        // The mint is the SPL Token mint of the payment
        mint: new PublicKey('So11111111111111111111111111111111111111112'),
        // The community can use this preset for 30 days
        days: 30,
        // The expires_at field is calculated based on the current timestamp and the days field at time of minting
        expiresAt: new Date().getTime() + 1000 * 60 * 60 * 24 * 30, // 30 days
      },
      minterConfig: {
        metadataConfig: {
          uri: `https://devnet.tokengator.app/api/metadata/json/${mintKeypair.publicKey.toString()}.json`,
          name: 'Business Visa',
          symbol: 'BV',
          metadata: [
            ['preset', 'business-visa'],
            ['community', 'tokengator'],
          ],
        },
        // NEW: We add an application_config field to the minter_config
        applicationConfig: {
          // In this case, the user needs to link their Discord and Twitter accounts before they can apply
          identities: [IdentityProvider.Discord, IdentityProvider.Twitter],
          // We set the price to 0.01 SOL and the payment is valid for 30 days
          // The expires_at field is calculated based on the current timestamp and the days field at time of minting
          paymentConfig: {
            amount: 1,
            price: 0.01 * LAMPORTS_PER_SOL,
            mint: new PublicKey('So11111111111111111111111111111111111111112'),
            days: 30,
            // The expires_at field is calculated based on the current timestamp and the days field at time of minting
            expiresAt: new Date().getTime() + 1000 * 60 * 60 * 24 * 30, // 30 days
          },
        },
      },
    }

    await program.methods
      .createMinter({
        name,
        imageUrl,
        description,
        interestConfig: null,
        transferFeeConfig: null,
        applicationConfig: {
          identities: identities.map((i) => {
            switch (i) {
              case IdentityProvider.Discord:
                return { discord: {} }
              case IdentityProvider.GitHub:
                return { gitHub: {} }
              case IdentityProvider.Google:
                return { google: {} }
              case IdentityProvider.Twitter:
                return { twitter: {} }
            }
          }),
          paymentConfig: {
            amount: appPaymentConfig.amount,
            days: appPaymentConfig.days,
            expiresAt: new anchor.BN(appPaymentConfig.expiresAt),
            mint: appPaymentConfig.mint,
            price: new anchor.BN(appPaymentConfig.price),
          },
        },
        metadataConfig: {
          metadata: metadataConfig.metadata,
          name: metadataConfig.name,
          symbol: metadataConfig.symbol,
          uri: metadataConfig.uri,
        },
        paymentConfig: {
          amount: paymentConfig.amount,
          days: paymentConfig.days,
          expiresAt: new anchor.BN(paymentConfig.expiresAt),
          mint: paymentConfig.mint,
          price: new anchor.BN(paymentConfig.price),
        },
      })
      .accounts({
        minter,
        group,
        authority: authority.publicKey,
        feePayer: remoteFeePayer.publicKey,
        mint: mintKeypair.publicKey,
        minterTokenAccount,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([authority, mintKeypair])
      .rpc({ commitment: 'confirmed', skipPreflight: true })

    const groupData = await program.account.group.fetch(group)
    const minterData = await program.account.minter.fetch(minter)
    const mintData = await getMint(provider.connection, mintKeypair.publicKey, 'confirmed', TOKEN_2022_PROGRAM_ID)
    const metadataData = await getTokenMetadata(provider.connection, mintKeypair.publicKey)

    const postBalance = await provider.connection.getBalance(authority.publicKey)
    expect(postBalance).toStrictEqual(1 * LAMPORTS_PER_SOL)

    // Minter
    expect(minterBump).toStrictEqual(minterData.bump)
    expect(minterData.authorities).toEqual([authority.publicKey])
    expect(imageUrl).toStrictEqual(minterData.imageUrl)
    expect(name).toStrictEqual(minterData.name)
    expect(description).toStrictEqual(minterData.description)
    expect(minterData.feePayer).toStrictEqual(remoteFeePayer.publicKey)
    expect(minterData.minterConfig.transferFeeConfig).toBeNull()
    expect(minterData.minterConfig.interestConfig).toBeNull()

    // Group
    expect(groupData.mint).toStrictEqual(mintKeypair.publicKey)
    expect(groupData.updateAuthority).toStrictEqual(minter)
    expect(groupData.maxSize).toStrictEqual(100)
    expect(groupData.size).toStrictEqual(0)

    // Mint
    expect(mintData.decimals).toStrictEqual(0)
    expect(mintData.mintAuthority).toStrictEqual(minter)
    expect(mintData.freezeAuthority).toStrictEqual(minter)
    expect(mintData.supply).toStrictEqual(1n)

    // Metadata
    expect(metadataData).not.toBeNull()
    expect(metadataData?.name).toStrictEqual(metadataConfig.name)
    expect(metadataData?.symbol).toStrictEqual(metadataConfig.symbol)
    expect(metadataData?.uri).toStrictEqual(metadataConfig.uri)
    expect(metadataData?.mint).toStrictEqual(mintKeypair.publicKey)
    expect(metadataData?.updateAuthority).toStrictEqual(minter)
    expect(metadataData?.additionalMetadata).toEqual([
      ['preset', 'business-visa'],
      ['community', 'tokengator'],
    ])
  })

  // it('Add Authority', async () => {
  //   const [minter] = getMinterPda({ name: 'Business Visa', programId: program.programId })

  //   await program.methods
  //     .addPresetAuthority({ newAuthority: authority2.publicKey })
  //     .accounts({ minter, authority: authority.publicKey, feePayer: remoteFeePayer.publicKey })
  //     .signers([authority])
  //     .rpc()

  //   const { authorities } = await program.account.minter.fetch(minter)

  //   const postBalance = await provider.connection.getBalance(authority.publicKey)

  //   expect(postBalance).toStrictEqual(1 * LAMPORTS_PER_SOL)
  //   expect(authorities.length).toStrictEqual(2)
  // })

  // it('Mint Business Visa Preset', async () => {
  //   const [minter] = getMinterPda({ name: 'Business Visa', programId: program.programId })
  //   const authorityTokenAccount = getAssociatedTokenAddressSync(
  //     mintKeypair.publicKey,
  //     authority2.publicKey,
  //     false,
  //     TOKEN_2022_PROGRAM_ID,
  //   )

  //   await program.methods
  //     .mintPreset()
  //     .accounts({
  //       associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //       tokenProgram: TOKEN_2022_PROGRAM_ID,
  //       feePayer: remoteFeePayer.publicKey,
  //       authority: authority2.publicKey,
  //       mint: mintKeypair.publicKey,
  //       authorityTokenAccount,
  //       minter,
  //       systemProgram: SystemProgram.programId,
  //     })
  //     .signers([authority2])
  //     .rpc({ commitment: 'confirmed', skipPreflight: true })

  //   const tokenAccountData = await getAccount(
  //     provider.connection,
  //     authorityTokenAccount,
  //     'confirmed',
  //     TOKEN_2022_PROGRAM_ID,
  //   )

  //   expect(tokenAccountData.amount).toStrictEqual(1n)
  //   expect(tokenAccountData.mint).toStrictEqual(mintKeypair.publicKey)
  // })

  // it('Remove Authority', async () => {
  //   const [minter] = getMinterPda({ name: 'Business Visa', programId: program.programId })

  //   await program.methods
  //     .removePresetAuthority({ authorityToRemove: authority2.publicKey })
  //     .accounts({ minter, authority: authority.publicKey, feePayer: remoteFeePayer.publicKey })
  //     .signers([authority])
  //     .rpc()

  //   const { authorities } = await program.account.minter.fetch(minter)
  //   expect(authorities).toEqual([authority.publicKey])
  // })

  // it('Remove Business Visa Preset', async () => {
  //   const [minter] = getMinterPda({ name: 'Business Visa', programId: program.programId })
  //   const authorityTokenAccount = getAssociatedTokenAddressSync(
  //     mintKeypair.publicKey,
  //     authority2.publicKey,
  //     false,
  //     TOKEN_2022_PROGRAM_ID,
  //   )

  //   const { decimals } = await getMint(provider.connection, mintKeypair.publicKey, 'confirmed', TOKEN_2022_PROGRAM_ID)

  //   await program.methods
  //     .removePreset()
  //     .accounts({
  //       associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //       authority: authority.publicKey,
  //       feePayer: remoteFeePayer.publicKey,
  //       mint: mintKeypair.publicKey,
  //       minter,
  //       systemProgram: SystemProgram.programId,
  //       tokenProgram: TOKEN_2022_PROGRAM_ID,
  //     })
  //     .preInstructions([
  //       createBurnCheckedInstruction(
  //         authorityTokenAccount,
  //         mintKeypair.publicKey,
  //         authority2.publicKey,
  //         1,
  //         decimals,
  //         [],
  //         TOKEN_2022_PROGRAM_ID,
  //       ),
  //     ])
  //     .signers([authority2, authority])
  //     .rpc({ skipPreflight: true })

  //   const minterData = await program.account.minter.fetchNullable(minter)
  //   const mintData = await provider.connection.getAccountInfo(mintKeypair.publicKey)
  //   expect(mintData).toBeNull()
  //   expect(minterData).toBeNull()
  // })
})
