import { Button } from '@mantine/core'
import { Keypair } from '@solana/web3.js'
import { useInitializeAccountMutation } from '../data-access/use-initialize-account-mutation'

export function TokengatorPresetUiCreate() {
  const initializeMutation = useInitializeAccountMutation()

  return (
    <Button onClick={() => initializeMutation.mutateAsync(Keypair.generate())} loading={initializeMutation.isPending}>
      Create
    </Button>
  )
}
