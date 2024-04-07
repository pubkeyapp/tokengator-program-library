import * as anchor from '@coral-xyz/anchor'
import { Program } from '@coral-xyz/anchor'
import {
  AddressLookupTableAccount,
  AddressLookupTableProgram,
  ComputeBudgetProgram,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SYSVAR_RENT_PUBKEY,
  SystemProgram,
  TransactionMessage,
  VersionedTransaction,
} from '@solana/web3.js'
import { TokengatorMinter } from '../target/types/tokengator_minter'
import { WenNewStandard } from '../target/types/wen_new_standard'
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_2022_PROGRAM_ID,
  getAssociatedTokenAddressSync,
  getMint,
  getTokenMetadata,
} from '@solana/spl-token'

const PREFIX = new TextEncoder().encode('tokengator_minter')
const MINTER = new TextEncoder().encode('minter')
const ACTIVITY = new TextEncoder().encode('activity')

enum IdentityProvider {
  Discord = 'Discord',
  GitHub = 'GitHub',
  Google = 'Google',
  Twitter = 'Twitter',
}

function getMinterPda({ programId, mint, name }: { name: string; mint: PublicKey; programId: PublicKey }) {
  return PublicKey.findProgramAddressSync([PREFIX, MINTER, mint.toBuffer(), new TextEncoder().encode(name)], programId)
}

function getActivityPda({ programId, mint, label }: { label: string; mint: PublicKey; programId: PublicKey }) {
  return PublicKey.findProgramAddressSync(
    [PREFIX, ACTIVITY, mint.toBuffer(), new TextEncoder().encode(label)],
    programId,
  )
}

export function getWNSGroupPda(mint: PublicKey, programId: PublicKey) {
  const GROUP_ACCOUNT_SEED = anchor.utils.bytes.utf8.encode('group')
  return PublicKey.findProgramAddressSync([GROUP_ACCOUNT_SEED, mint.toBuffer()], programId)
}

export function getWNSMemberPda(mint: PublicKey, programId: PublicKey) {
  const GROUP_ACCOUNT_SEED = anchor.utils.bytes.utf8.encode('member')
  return PublicKey.findProgramAddressSync([GROUP_ACCOUNT_SEED, mint.toBuffer()], programId)
}

export function getWNSManagerPda(programId: PublicKey) {
  const MANAGER_SEED = anchor.utils.bytes.utf8.encode('manager')
  return PublicKey.findProgramAddressSync([MANAGER_SEED], programId)
}

async function sleep(ms: number) {
  return new Promise((resolve) => {
    return setTimeout(() => {
      resolve(true)
    }, ms)
  })
}

describe('tokengator-minter', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)
  const remoteFeePayer = provider.wallet as anchor.Wallet
  const program = anchor.workspace.TokengatorMinter as Program<TokengatorMinter>
  const wnsProgramId = (anchor.workspace.WenNewStandard as Program<WenNewStandard>).programId

  const authority = Keypair.generate()
  const groupMintKeypair = Keypair.generate()
  const memberMintKeypair = Keypair.generate()

  let lookupTableStore: AddressLookupTableAccount | null

  beforeAll(async () => {
    console.log('Airdropping authority 1 SOL:', authority.publicKey.toString())
    await provider.connection.confirmTransaction({
      ...(await provider.connection.getLatestBlockhash('confirmed')),
      signature: await provider.connection.requestAirdrop(authority.publicKey, 1 * LAMPORTS_PER_SOL),
    })
  })

  it('Create Business Visa TokengatorMinter', async () => {
    const [minter, minterBump] = getMinterPda({
      name: 'Business Visa WNS',
      mint: groupMintKeypair.publicKey,
      programId: program.programId,
    })
    const [group] = getWNSGroupPda(groupMintKeypair.publicKey, wnsProgramId)
    const [manager] = getWNSManagerPda(wnsProgramId)

    const minterTokenAccount = getAssociatedTokenAddressSync(
      groupMintKeypair.publicKey,
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
      name: 'Business Visa WNS',
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
          uri: `https://devnet.tokengator.app/api/metadata/json/${groupMintKeypair.publicKey.toString()}.json`,
          name: 'Business Visa',
          symbol: 'BV',
          metadata: [
            ['preset', 'business-visa'],
            ['community', 'tokengator'],
            ['preset2', 'business-visa2'],
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

    const slot = await provider.connection.getSlot('confirmed')
    const { blockhash, lastValidBlockHeight } = await provider.connection.getLatestBlockhash()
    const [createLookupTableIx, lookupTableAccount] = AddressLookupTableProgram.createLookupTable({
      authority: remoteFeePayer.publicKey,
      payer: remoteFeePayer.publicKey,
      recentSlot: slot - 1,
    })

    const extendLookupTableIx = AddressLookupTableProgram.extendLookupTable({
      addresses: [
        minter,
        group,
        manager,
        minterTokenAccount,
        authority.publicKey,
        remoteFeePayer.publicKey,
        groupMintKeypair.publicKey,
        SYSVAR_RENT_PUBKEY,
        wnsProgramId,
        ASSOCIATED_TOKEN_PROGRAM_ID,
        TOKEN_2022_PROGRAM_ID,
        SystemProgram.programId,
      ],
      authority: remoteFeePayer.publicKey,
      lookupTable: lookupTableAccount,
      payer: remoteFeePayer.publicKey,
    })

    const txMessage = new TransactionMessage({
      instructions: [createLookupTableIx, extendLookupTableIx],
      payerKey: remoteFeePayer.publicKey,
      recentBlockhash: blockhash,
    }).compileToV0Message()

    const tx = new VersionedTransaction(txMessage)
    tx.sign([remoteFeePayer.payer])

    const sig = await provider.connection.sendTransaction(tx)
    await provider.connection.confirmTransaction({ blockhash, lastValidBlockHeight, signature: sig })

    await sleep(750)
    const createMinterWnsIx = await program.methods
      .createMinterWns({
        community: 'pubkey',
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
        manager,
        minterTokenAccount,
        authority: authority.publicKey,
        feePayer: remoteFeePayer.publicKey,
        mint: groupMintKeypair.publicKey,
        rent: SYSVAR_RENT_PUBKEY,
        wnsProgram: wnsProgramId,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([authority, groupMintKeypair])
      .instruction()

    const { value } = await provider.connection.getAddressLookupTable(lookupTableAccount)

    if (value) {
      const { blockhash, lastValidBlockHeight } = await provider.connection.getLatestBlockhash()
      const transactionMessage = new TransactionMessage({
        instructions: [ComputeBudgetProgram.setComputeUnitLimit({ units: 250_000 }), createMinterWnsIx],
        payerKey: remoteFeePayer.publicKey,
        recentBlockhash: blockhash,
      }).compileToV0Message([value])

      const transaction = new VersionedTransaction(transactionMessage)
      transaction.sign([remoteFeePayer.payer, authority, groupMintKeypair])
      const sig = await provider.connection.sendTransaction(transaction, { skipPreflight: true })
      await provider.connection.confirmTransaction({ blockhash, lastValidBlockHeight, signature: sig }, 'confirmed')

      const minterData = await program.account.minter.fetch(minter)
      const mintData = await getMint(
        provider.connection,
        groupMintKeypair.publicKey,
        'confirmed',
        TOKEN_2022_PROGRAM_ID,
      )
      const metadataData = await getTokenMetadata(provider.connection, groupMintKeypair.publicKey)

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

      // Mint
      expect(mintData.decimals).toStrictEqual(0)
      // Change this after wns updates their set
      expect(mintData.mintAuthority).toStrictEqual(manager)
      expect(mintData.freezeAuthority).toStrictEqual(manager)
      expect(mintData.supply).toStrictEqual(1n)

      // Metadata
      expect(metadataData).not.toBeNull()
      expect(metadataData?.name).toStrictEqual(metadataConfig.name)
      expect(metadataData?.symbol).toStrictEqual(metadataConfig.symbol)
      expect(metadataData?.uri).toStrictEqual(metadataConfig.uri)
      expect(metadataData?.mint).toStrictEqual(groupMintKeypair.publicKey)
      expect(metadataData?.updateAuthority).toStrictEqual(minter)
      expect(metadataData?.additionalMetadata).toEqual(metadataConfig.metadata)
    }

    lookupTableStore = value
  })

  it('Mint Business Visa', async () => {
    const [minter] = getMinterPda({
      name: 'Business Visa WNS',
      mint: groupMintKeypair.publicKey,
      programId: program.programId,
    })
    const [group] = getWNSGroupPda(groupMintKeypair.publicKey, wnsProgramId)
    const [member] = getWNSMemberPda(memberMintKeypair.publicKey, wnsProgramId)
    const [manager] = getWNSManagerPda(wnsProgramId)

    const { name, symbol, uri, metadata } = {
      uri: `https://devnet.tokengator.app/api/metadata/json/${memberMintKeypair.publicKey.toString()}.json`,
      name: 'Business Visa #0001',
      symbol: 'BV',
      metadata: [
        ['community', 'deanslist'],
        ['username', 'beeman'],
        ['preset', 'business-visa'],
      ],
    }

    const authorityTokenAccount = getAssociatedTokenAddressSync(
      memberMintKeypair.publicKey,
      authority.publicKey,
      false,
      TOKEN_2022_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID,
    )

    if (lookupTableStore) {
      const mintMinterWnsIx = await program.methods
        .mintMinterWns({ name, symbol, uri, metadata })
        .accounts({
          minter,
          group,
          manager,
          member,
          authorityTokenAccount,
          authority: authority.publicKey,
          feePayer: remoteFeePayer.publicKey,
          mint: memberMintKeypair.publicKey,
          rent: SYSVAR_RENT_PUBKEY,
          wnsProgram: wnsProgramId,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          tokenProgram: TOKEN_2022_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .signers([authority, memberMintKeypair])
        .instruction()

      const { blockhash, lastValidBlockHeight } = await provider.connection.getLatestBlockhash()
      const transactionMessage = new TransactionMessage({
        instructions: [ComputeBudgetProgram.setComputeUnitLimit({ units: 250_000 }), mintMinterWnsIx],
        payerKey: remoteFeePayer.publicKey,
        recentBlockhash: blockhash,
      }).compileToV0Message([lookupTableStore])

      const transaction = new VersionedTransaction(transactionMessage)
      transaction.sign([remoteFeePayer.payer, authority, memberMintKeypair])
      const sig = await provider.connection.sendTransaction(transaction, { skipPreflight: true })
      await provider.connection.confirmTransaction({ blockhash, lastValidBlockHeight, signature: sig }, 'confirmed')

      const mintData = await getMint(
        provider.connection,
        memberMintKeypair.publicKey,
        'confirmed',
        TOKEN_2022_PROGRAM_ID,
      )
      const metadataData = await getTokenMetadata(provider.connection, memberMintKeypair.publicKey)

      const postBalance = await provider.connection.getBalance(authority.publicKey)
      expect(postBalance).toStrictEqual(1 * LAMPORTS_PER_SOL)

      // Mint
      expect(mintData.decimals).toStrictEqual(0)
      // Change this after wns updates their set
      expect(mintData.mintAuthority).toStrictEqual(manager)
      expect(mintData.freezeAuthority).toStrictEqual(manager)
      expect(mintData.supply).toStrictEqual(1n)

      // Metadata
      expect(metadataData).not.toBeNull()
      expect(metadataData?.name).toStrictEqual(name)
      expect(metadataData?.symbol).toStrictEqual(symbol)
      expect(metadataData?.uri).toStrictEqual(uri)
      expect(metadataData?.mint).toStrictEqual(memberMintKeypair.publicKey)
      expect(metadataData?.updateAuthority).toStrictEqual(minter)
      expect(metadataData?.additionalMetadata).toEqual(metadata)
    }
  })

  it('Update Business Visa Additional Metadata', async () => {
    const [minter] = getMinterPda({
      name: 'Business Visa WNS',
      mint: groupMintKeypair.publicKey,
      programId: program.programId,
    })
    const [group] = getWNSGroupPda(groupMintKeypair.publicKey, wnsProgramId)
    const [member] = getWNSMemberPda(memberMintKeypair.publicKey, wnsProgramId)

    if (lookupTableStore) {
      const mintMinterWnsIx = await program.methods
        .updateMemberMetadataWns({ field: 'preset', newValue: 'tourist-visa' })
        .accounts({
          minter,
          group,
          member,
          feePayer: remoteFeePayer.publicKey,
          mint: memberMintKeypair.publicKey,
          wnsProgram: wnsProgramId,
          tokenProgram: TOKEN_2022_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .instruction()

      const { blockhash, lastValidBlockHeight } = await provider.connection.getLatestBlockhash()
      const transactionMessage = new TransactionMessage({
        instructions: [ComputeBudgetProgram.setComputeUnitLimit({ units: 250_000 }), mintMinterWnsIx],
        payerKey: remoteFeePayer.publicKey,
        recentBlockhash: blockhash,
      }).compileToV0Message([lookupTableStore])

      const transaction = new VersionedTransaction(transactionMessage)
      transaction.sign([remoteFeePayer.payer])
      const sig = await provider.connection.sendTransaction(transaction, { skipPreflight: true })
      await provider.connection.confirmTransaction({ blockhash, lastValidBlockHeight, signature: sig }, 'confirmed')

      const metadataData = await getTokenMetadata(provider.connection, memberMintKeypair.publicKey)

      const postBalance = await provider.connection.getBalance(authority.publicKey)
      expect(postBalance).toStrictEqual(1 * LAMPORTS_PER_SOL)

      // Metadata
      expect(metadataData?.additionalMetadata).toEqual([
        ['community', 'deanslist'],
        ['username', 'beeman'],
        ['preset', 'tourist-visa'],
      ])
    }
  })

  it('Create Activity for Business Visa Member', async () => {
    const label = 'gm'
    const [minter] = getMinterPda({
      name: 'Business Visa WNS',
      mint: groupMintKeypair.publicKey,
      programId: program.programId,
    })

    const [group] = getWNSGroupPda(groupMintKeypair.publicKey, wnsProgramId)
    const [member] = getWNSMemberPda(memberMintKeypair.publicKey, wnsProgramId)
    const [activity, activityBump] = getActivityPda({
      label,
      mint: memberMintKeypair.publicKey,
      programId: program.programId,
    })

    if (lookupTableStore) {
      const createActivityIx = await program.methods
        .createActivity({
          label,
          endDate: null,
          startDate: null,
        })
        .accounts({
          minter,
          activity,
          group,
          member,
          feePayer: remoteFeePayer.publicKey,
          mint: memberMintKeypair.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .instruction()

      const { blockhash, lastValidBlockHeight } = await provider.connection.getLatestBlockhash()
      const transactionMessage = new TransactionMessage({
        instructions: [ComputeBudgetProgram.setComputeUnitLimit({ units: 30_000 }), createActivityIx],
        payerKey: remoteFeePayer.publicKey,
        recentBlockhash: blockhash,
      }).compileToV0Message([lookupTableStore])

      const transaction = new VersionedTransaction(transactionMessage)
      transaction.sign([remoteFeePayer.payer])
      const sig = await provider.connection.sendTransaction(transaction, { skipPreflight: true })
      await provider.connection.confirmTransaction({ blockhash, lastValidBlockHeight, signature: sig }, 'confirmed')

      const activityData = await program.account.activity.fetch(activity)

      const postBalance = await provider.connection.getBalance(authority.publicKey)
      expect(postBalance).toStrictEqual(1 * LAMPORTS_PER_SOL)

      // Activity
      expect(activityData.endDate).not.toBeNull()
      expect(activityData.startDate).not.toBeNull()
      expect(activityData.bump).toStrictEqual(activityBump)
      expect(activityData.entries).toStrictEqual([])
      expect(activityData.feePayer).toStrictEqual(remoteFeePayer.publicKey)
      expect(activityData.label).toStrictEqual(label)
      expect(activityData.member).toStrictEqual(member)
      expect(activityData.mint).toStrictEqual(memberMintKeypair.publicKey)
      expect(activityData.minter).toStrictEqual(minter)
    }
  })

  it('Create entry for Activity', async () => {
    const label = 'gm'
    const [activity] = getActivityPda({
      label,
      mint: memberMintKeypair.publicKey,
      programId: program.programId,
    })

    if (lookupTableStore) {
      const entry = {
        message: 'Secured a bounty',
        points: 100,
        timestamp: new anchor.BN(Math.round(Date.now() / 1000)),
        url: 'https://gib.work',
      }

      const appendEntryToActivityIx = await program.methods
        .appendActivityEntry(entry)
        .accounts({
          activity,
          feePayer: remoteFeePayer.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .instruction()

      const { blockhash, lastValidBlockHeight } = await provider.connection.getLatestBlockhash()
      const transactionMessage = new TransactionMessage({
        instructions: [ComputeBudgetProgram.setComputeUnitLimit({ units: 50_000 }), appendEntryToActivityIx],
        payerKey: remoteFeePayer.publicKey,
        recentBlockhash: blockhash,
      }).compileToV0Message([lookupTableStore])

      const transaction = new VersionedTransaction(transactionMessage)
      transaction.sign([remoteFeePayer.payer])
      const sig = await provider.connection.sendTransaction(transaction, { skipPreflight: true })
      await provider.connection.confirmTransaction({ blockhash, lastValidBlockHeight, signature: sig }, 'confirmed')

      const postBalance = await provider.connection.getBalance(authority.publicKey)
      expect(postBalance).toStrictEqual(1 * LAMPORTS_PER_SOL)

      const activityData = await program.account.activity.fetch(activity)
      // Activity
      expect(activityData.entries).toStrictEqual([entry])
    }
  })
})
