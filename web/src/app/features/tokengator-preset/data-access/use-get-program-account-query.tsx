import { useQuery } from '@tanstack/react-query'

import { useTokengatorPresetProgram } from './use-tokengator-preset-program'

export function useGetProgramAccountQuery() {
  const { programId, cluster, connection } = useTokengatorPresetProgram()

  return useQuery({
    queryKey: ['get-program-account', { cluster }],
    queryFn: () => connection.getParsedAccountInfo(programId),
  })
}
