import { Button, Group } from '@mantine/core'
import { UiCard, UiLoader } from '@pubkey-ui/core'
import { PublicKey } from '@solana/web3.js'
import { useMemo } from 'react'
import { ellipsify } from '../../account/ui/ellipsify'
import { ExplorerLink } from '../../cluster/cluster-ui'
import { useTokengatorPresetProgramAccount } from '../data-access/use-tokengator-preset-program-account'

export function TokengatorPresetUiCard({ account }: { account: PublicKey }) {
  const { accountQuery, incrementMutation, setMutation, decrementMutation, closeMutation } =
    useTokengatorPresetProgramAccount({ account })

  const count = useMemo(() => accountQuery.data?.count ?? 0, [accountQuery.data?.count])

  return accountQuery.isLoading ? (
    <UiLoader />
  ) : (
    <UiCard title={<Button onClick={() => accountQuery.refetch()}>{count}</Button>}>
      <Group>
        <Button loading={incrementMutation.isPending} onClick={() => incrementMutation.mutateAsync()}>
          Increment
        </Button>
        <Button
          loading={setMutation.isPending}
          onClick={() => {
            const value = window.prompt('Set value to:', count.toString() ?? '0')
            if (!value || parseInt(value) === count || isNaN(parseInt(value))) {
              return
            }
            return setMutation.mutateAsync(parseInt(value))
          }}
        >
          Set
        </Button>
        <Button loading={decrementMutation.isPending} onClick={() => decrementMutation.mutateAsync()}>
          Decrement
        </Button>
      </Group>
      <Group justify="flex-end">
        <ExplorerLink path={`account/${account}`} label={ellipsify(account.toString())} />
        <Button
          loading={closeMutation.isPending}
          onClick={() => {
            if (!window.confirm('Are you sure you want to close this account?')) {
              return
            }
            return closeMutation.mutateAsync()
          }}
        >
          Close
        </Button>
      </Group>
    </UiCard>
  )
}
