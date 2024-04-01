import * as anchor from '@coral-xyz/anchor'
import { Program } from '@coral-xyz/anchor'
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram } from '@solana/web3.js'
import { TokengatorPreset } from '../target/types/tokengator_preset'
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_2022_PROGRAM_ID,
  createBurnCheckedInstruction,
  getAccount,
  getAssociatedTokenAddressSync,
  getMint,
  getTokenMetadata,
} from '@solana/spl-token'

const PREFIX = new TextEncoder().encode('tokengator_preset')
const PRESET = new TextEncoder().encode('preset')

function getDefaultMetadata(metadata: { name: string; symbol: string; image?: string; mint: PublicKey }) {
  const baseUrl = `https://metadata-tool.deno.dev/metadata`

  const { image, mint, name, symbol } = metadata

  return `${baseUrl}?name=${encodeURIComponent(name)}&symbol=${encodeURIComponent(symbol)}&mint=${mint}${
    image ? `&image=${encodeURIComponent(image)}` : ''
  }`
}

function daysFromNow(days: number) {
  const now = new Date()
  now.setDate(now.getDate() + days)
  return now.toISOString().split('T')[0]
}

function getPresetPda({ programId, name }: { name: string; programId: PublicKey }) {
  return PublicKey.findProgramAddressSync([PREFIX, PRESET, new TextEncoder().encode(name)], programId)
}

describe('tokengator-preset', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)
  const remoteFeePayer = provider.wallet as anchor.Wallet
  const program = anchor.workspace.TokengatorPreset as Program<TokengatorPreset>

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

  it('Create Business Visa TokengatorPreset', async () => {
    const [preset, presetBump] = getPresetPda({ name: 'Business Visa', programId: program.programId })

    const expiresAt = new Date(daysFromNow(14)).toISOString()
    const businessVisaPreset = {
      name: 'Business Visa',
      description: 'Token with Non-Transferable extensions.',
      imageUrl: 'https://raw.githubusercontent.com/pubkeyapp/tokengator-assets/main/developer-portal/image.png',
      metadataConfig: {
        imageUrl: 'https://raw.githubusercontent.com/pubkeyapp/tokengator-assets/main/saga-phone/image.png',
        metadata: [
          ['status', 'active'],
          ['expiresAt', expiresAt],
        ],
        name: 'OPOS Business Visa',
        symbol: 'BUS',
        uri: getDefaultMetadata({
          mint: mintKeypair.publicKey,
          name: 'OPOS Business Visa',
          symbol: 'BUS',
          image: 'https://raw.githubusercontent.com/pubkeyapp/tokengator-assets/main/saga-phone/image.png',
        }),
      },
      decimals: 0,
      interestConfig: null,
      transferFeeConfig: null,
    }

    await program.methods
      .createPreset(businessVisaPreset)
      .accounts({
        authority: authority.publicKey,
        feePayer: remoteFeePayer.publicKey,
        mint: mintKeypair.publicKey,
        preset,
        tokenExtensionsProgram: TOKEN_2022_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([authority, mintKeypair])
      .rpc({ commitment: 'confirmed' })

    const presetData = await program.account.preset.fetch(preset)
    const mintData = await getMint(provider.connection, mintKeypair.publicKey, 'confirmed', TOKEN_2022_PROGRAM_ID)
    const metadataData = await getTokenMetadata(provider.connection, mintKeypair.publicKey)

    const postBalance = await provider.connection.getBalance(authority.publicKey)
    expect(postBalance).toStrictEqual(1 * LAMPORTS_PER_SOL)

    // Preset
    expect(presetBump).toStrictEqual(presetData.bump)
    expect(presetData.authorities).toEqual([authority.publicKey])
    expect(businessVisaPreset.imageUrl).toStrictEqual(presetData.imageUrl)
    expect(businessVisaPreset.name).toStrictEqual(presetData.name)
    expect(businessVisaPreset.description).toStrictEqual(presetData.description)
    expect(presetData.feePayer).toStrictEqual(remoteFeePayer.publicKey)
    expect(presetData.minterConfig.transferFeeConfig).toBeNull()
    expect(presetData.minterConfig.interestConfig).toBeNull()

    // Mint
    expect(mintData.decimals).toStrictEqual(businessVisaPreset.decimals)
    expect(mintData.mintAuthority).toStrictEqual(preset)
    expect(mintData.freezeAuthority).toStrictEqual(preset)
    expect(mintData.supply).toStrictEqual(0n)

    // Metadata
    expect(metadataData).not.toBeNull()
    expect(metadataData?.name).toStrictEqual(businessVisaPreset.metadataConfig.name)
    expect(metadataData?.symbol).toStrictEqual(businessVisaPreset.metadataConfig.symbol)
    expect(metadataData?.uri).toStrictEqual(businessVisaPreset.metadataConfig.uri)
    expect(metadataData?.mint).toStrictEqual(mintKeypair.publicKey)
    expect(metadataData?.updateAuthority).toStrictEqual(preset)
    expect(metadataData?.additionalMetadata).toEqual([
      ['status', 'active'],
      ['expiresAt', expiresAt],
    ])
  })

  it('Add Authority', async () => {
    const [preset] = getPresetPda({ name: 'Business Visa', programId: program.programId })

    await program.methods
      .addPresetAuthority({ newAuthority: authority2.publicKey })
      .accounts({ preset, authority: authority.publicKey, feePayer: remoteFeePayer.publicKey })
      .signers([authority])
      .rpc()

    const { authorities } = await program.account.preset.fetch(preset)

    const postBalance = await provider.connection.getBalance(authority.publicKey)

    expect(postBalance).toStrictEqual(1 * LAMPORTS_PER_SOL)
    expect(authorities.length).toStrictEqual(2)
  })

  it('Mint Business Visa Preset', async () => {
    const [preset] = getPresetPda({ name: 'Business Visa', programId: program.programId })
    const authorityTokenAccount = getAssociatedTokenAddressSync(
      mintKeypair.publicKey,
      authority2.publicKey,
      false,
      TOKEN_2022_PROGRAM_ID,
    )

    await program.methods
      .mintPreset()
      .accounts({
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        feePayer: remoteFeePayer.publicKey,
        authority: authority2.publicKey,
        mint: mintKeypair.publicKey,
        authorityTokenAccount,
        preset,
        systemProgram: SystemProgram.programId,
      })
      .signers([authority2])
      .rpc({ commitment: 'confirmed', skipPreflight: true })

    const tokenAccountData = await getAccount(
      provider.connection,
      authorityTokenAccount,
      'confirmed',
      TOKEN_2022_PROGRAM_ID,
    )

    expect(tokenAccountData.amount).toStrictEqual(1n)
    expect(tokenAccountData.mint).toStrictEqual(mintKeypair.publicKey)
  })

  it('Remove Authority', async () => {
    const [preset] = getPresetPda({ name: 'Business Visa', programId: program.programId })

    await program.methods
      .removePresetAuthority({ authorityToRemove: authority2.publicKey })
      .accounts({ preset, authority: authority.publicKey, feePayer: remoteFeePayer.publicKey })
      .signers([authority])
      .rpc()

    const { authorities } = await program.account.preset.fetch(preset)
    expect(authorities).toEqual([authority.publicKey])
  })

  it('Remove Business Visa Preset', async () => {
    const [preset] = getPresetPda({ name: 'Business Visa', programId: program.programId })
    const authorityTokenAccount = getAssociatedTokenAddressSync(
      mintKeypair.publicKey,
      authority2.publicKey,
      false,
      TOKEN_2022_PROGRAM_ID,
    )

    const { decimals } = await getMint(provider.connection, mintKeypair.publicKey, 'confirmed', TOKEN_2022_PROGRAM_ID)

    await program.methods
      .removePreset()
      .accounts({
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        authority: authority.publicKey,
        feePayer: remoteFeePayer.publicKey,
        mint: mintKeypair.publicKey,
        preset,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .preInstructions([
        createBurnCheckedInstruction(
          authorityTokenAccount,
          mintKeypair.publicKey,
          authority2.publicKey,
          1,
          decimals,
          [],
          TOKEN_2022_PROGRAM_ID,
        ),
      ])
      .signers([authority2, authority])
      .rpc({ skipPreflight: true })

    const presetData = await program.account.preset.fetchNullable(preset)
    const mintData = await provider.connection.getAccountInfo(mintKeypair.publicKey)
    expect(mintData).toBeNull()
    expect(presetData).toBeNull()
  })
})
