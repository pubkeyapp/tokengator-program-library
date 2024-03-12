// Here we export some useful types and functions for interacting with the Anchor program.
import { Cluster, PublicKey } from '@solana/web3.js'
import type { TokengatorPreset } from '../target/types/tokengator_preset'
import { IDL as TokengatorPresetIDL } from '../target/types/tokengator_preset'

// Re-export the generated IDL and type
export { TokengatorPreset, TokengatorPresetIDL }

// After updating your program ID (e.g. after running `anchor keys sync`) update the value below.
export const TOKENGATOR_PRESET_PROGRAM_ID = new PublicKey('TPLxuiUYiDdJVenaSEhshrLP9EF83MNzKhiHNoFjCPM')

// This is a helper function to get the program ID for the TokengatorPreset program depending on the cluster.
export function getTokengatorPresetProgramId(cluster: Cluster) {
  switch (cluster) {
    case 'devnet':
    case 'testnet':
    case 'mainnet-beta':
    default:
      return TOKENGATOR_PRESET_PROGRAM_ID
  }
}
