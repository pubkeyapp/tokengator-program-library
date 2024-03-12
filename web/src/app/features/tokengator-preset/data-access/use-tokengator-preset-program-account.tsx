import { PublicKey } from '@solana/web3.js'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { uiToastLink } from '../../account/account-data-access'
import { useCluster } from '../../cluster/cluster-data-access'
import { useTokengatorPresetProgram } from './use-tokengator-preset-program'

export function useTokengatorPresetProgramAccount({ account }: { account: PublicKey }) {
  const { cluster, getExplorerUrl } = useCluster()
  const client = useQueryClient()
  const { program } = useTokengatorPresetProgram()

  const accountQuery = useQuery({
    queryKey: ['tokengator-preset', 'fetch', { cluster, account }],
    queryFn: () => program.account.tokengatorPreset.fetch(account),
  })

  const closeMutation = useMutation({
    mutationKey: ['tokengator-preset', 'close', { cluster, account }],
    mutationFn: () => program.methods.close().accounts({ tokengatorPreset: account }).rpc(),
    onSuccess: (tx) => {
      uiToastLink({ link: getExplorerUrl(`tx/${tx}`), label: 'View Transaction' })
      return client.invalidateQueries({ queryKey: ['tokengator-preset', 'all', { cluster }] })
    },
  })

  const decrementMutation = useMutation({
    mutationKey: ['tokengator-preset', 'decrement', { cluster, account }],
    mutationFn: () => program.methods.decrement().accounts({ tokengatorPreset: account }).rpc(),
    onSuccess: (tx) => {
      uiToastLink({ link: getExplorerUrl(`tx/${tx}`), label: 'View Transaction' })
      return accountQuery.refetch()
    },
  })

  const incrementMutation = useMutation({
    mutationKey: ['tokengator-preset', 'increment', { cluster, account }],
    mutationFn: () => program.methods.increment().accounts({ tokengatorPreset: account }).rpc(),
    onSuccess: (tx) => {
      uiToastLink({ link: getExplorerUrl(`tx/${tx}`), label: 'View Transaction' })
      return accountQuery.refetch()
    },
  })

  const setMutation = useMutation({
    mutationKey: ['tokengator-preset', 'set', { cluster, account }],
    mutationFn: (value: number) => program.methods.set(value).accounts({ tokengatorPreset: account }).rpc(),
    onSuccess: (tx) => {
      uiToastLink({ link: getExplorerUrl(`tx/${tx}`), label: 'View Transaction' })
      return accountQuery.refetch()
    },
  })

  return {
    accountQuery,
    closeMutation,
    decrementMutation,
    incrementMutation,
    setMutation,
  }
}
