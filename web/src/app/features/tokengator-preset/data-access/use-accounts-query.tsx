import { useQuery } from '@tanstack/react-query'
import { useTokengatorPresetProgram } from './use-tokengator-preset-program'

export function useAccountsQuery() {
  const { cluster, program } = useTokengatorPresetProgram()
  return useQuery({
    queryKey: ['tokengator-preset', 'all', { cluster }],
    queryFn: () => program.account.tokengatorPreset.all(),
  })
}
