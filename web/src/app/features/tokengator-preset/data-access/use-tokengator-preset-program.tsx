import { Program } from '@coral-xyz/anchor'
import { useConnection } from '@solana/wallet-adapter-react'
import { Cluster } from '@solana/web3.js'
import { getTokengatorPresetProgramId, TokengatorPresetIDL } from '@tokengator-starter/anchor'
import { useMemo } from 'react'
import { useCluster } from '../../cluster/cluster-data-access'
import { useAnchorProvider } from '../../solana/solana-provider'

export function useTokengatorPresetProgram() {
  const { connection } = useConnection()
  const { cluster, getExplorerUrl } = useCluster()

  const provider = useAnchorProvider()
  const programId = useMemo(() => getTokengatorPresetProgramId(cluster.network as Cluster), [cluster])
  const program = new Program(TokengatorPresetIDL, programId, provider)

  return {
    cluster,
    connection,
    getExplorerUrl,
    program,
    programId,
  }
}
