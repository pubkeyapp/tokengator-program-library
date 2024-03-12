import { useWallet } from '@solana/wallet-adapter-react'
import { useKeypair } from '../../keypair/data-access'

import { AccountUiBalanceCheck } from './account-ui-balance-check'

export function AccountUiChecker() {
  const { publicKey } = useWallet()
  if (!publicKey) {
    return null
  }
  return <AccountUiBalanceCheck address={publicKey} />
}

export function AccountUiCheckerKeypair() {
  const { keypair } = useKeypair()
  if (!keypair.solana) {
    return null
  }
  return <AccountUiBalanceCheck label="Keypair account not found" address={keypair.solana?.publicKey} />
}
