import * as anchor from '@coral-xyz/anchor'
import { Program } from '@coral-xyz/anchor'
import { Keypair } from '@solana/web3.js'
import { TokengatorPreset } from "../target/types/tokengator_preset";

describe("tokengator-preset", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)
  const payer = provider.wallet as anchor.Wallet

  const program = anchor.workspace.TokengatorPreset as Program<TokengatorPreset>;

  const tokengatorPresetKeypair = Keypair.generate()

  it('Initialize TokengatorPreset', async () => {
    await program.methods
      .initialize()
      .accounts({
        tokengatorPreset: tokengatorPresetKeypair.publicKey,
        payer: payer.publicKey,
      })
      .signers([tokengatorPresetKeypair])
      .rpc()

    const currentCount = await program.account.tokengatorPreset.fetch(tokengatorPresetKeypair.publicKey)

    expect(currentCount.count).toEqual(0)
  })

  it('Increment TokengatorPreset', async () => {
    await program.methods.increment().accounts({ tokengatorPreset: tokengatorPresetKeypair.publicKey }).rpc()

    const currentCount = await program.account.tokengatorPreset.fetch(tokengatorPresetKeypair.publicKey)

    expect(currentCount.count).toEqual(1)
  })

  it('Increment TokengatorPreset Again', async () => {
    await program.methods.increment().accounts({ tokengatorPreset: tokengatorPresetKeypair.publicKey }).rpc()

    const currentCount = await program.account.tokengatorPreset.fetch(tokengatorPresetKeypair.publicKey)

    expect(currentCount.count).toEqual(2)
  })

  it('Decrement TokengatorPreset', async () => {
    await program.methods.decrement().accounts({ tokengatorPreset: tokengatorPresetKeypair.publicKey }).rpc()

    const currentCount = await program.account.tokengatorPreset.fetch(tokengatorPresetKeypair.publicKey)

    expect(currentCount.count).toEqual(1)
  })

  it('Set tokengatorPreset value', async () => {
    await program.methods.set(42).accounts({ tokengatorPreset: tokengatorPresetKeypair.publicKey }).rpc()

    const currentCount = await program.account.tokengatorPreset.fetch(tokengatorPresetKeypair.publicKey)

    expect(currentCount.count).toEqual(42)
  })

  it('Set close the tokengatorPreset account', async () => {
    await program.methods
      .close()
      .accounts({
        payer: payer.publicKey,
        tokengatorPreset: tokengatorPresetKeypair.publicKey,
      })
      .rpc()

    // The account should no longer exist, returning null.
    const userAccount = await program.account.tokengatorPreset.fetchNullable(tokengatorPresetKeypair.publicKey)
    expect(userAccount).toBeNull()
  })
})
