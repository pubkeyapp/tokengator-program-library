import { toastError } from '@pubkey-ui/core'
import { Keypair } from '@solana/web3.js'
import { useMutation, useQueryClient } from '@tanstack/react-query'
import { uiToastLink } from '../../account/account-data-access'

import { useTokengatorPresetProgram } from './use-tokengator-preset-program'

export function useInitializeAccountMutation() {
  const { cluster, getExplorerUrl, program } = useTokengatorPresetProgram()
  const client = useQueryClient()

  return useMutation({
    mutationKey: ['tokengator-preset', 'initialize', { cluster }],
    mutationFn: (keypair: Keypair) =>
      program.methods.initialize().accounts({ tokengatorPreset: keypair.publicKey }).signers([keypair]).rpc(),
    onSuccess: (signature) => {
      uiToastLink({ link: getExplorerUrl(`tx/${signature}`), label: 'View Transaction' })
      return client.invalidateQueries({ queryKey: ['tokengator-preset', 'all', { cluster }] })
    },
    onError: () => toastError('Failed to initialize account'),
  })
}
