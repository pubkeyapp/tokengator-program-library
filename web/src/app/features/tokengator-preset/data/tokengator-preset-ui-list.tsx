import { UiDebug, UiInfo, UiLoader, UiStack } from '@pubkey-ui/core'
import { useAccountsQuery } from '../data-access/use-accounts-query'
import { useGetProgramAccountQuery } from '../data-access/use-get-program-account-query'
import { TokengatorPresetUiCard } from './tokengator-preset-ui-card'

export function TokengatorPresetUiList() {
  const accountsQuery = useAccountsQuery()
  const gpaQuery = useGetProgramAccountQuery()

  if (gpaQuery.isLoading) {
    return <UiLoader />
  }
  if (!gpaQuery.data?.value) {
    return (
      <UiInfo message="Program account not found. Make sure you have deployed the program and are on the correct cluster." />
    )
  }

  return (
    <UiStack>
      {accountsQuery.isLoading ? (
        <UiLoader />
      ) : accountsQuery.data?.length ? (
        <UiStack>
          {accountsQuery.data?.map((account) => (
            <TokengatorPresetUiCard key={account.publicKey.toString()} account={account.publicKey} />
          ))}
        </UiStack>
      ) : (
        <UiInfo title="No accounts" message="No accounts found. Create one above to get started." />
      )}
      <UiDebug data={{ accounts: accountsQuery.data, accountsErr: accountsQuery.error, gpa: gpaQuery.data }} open />
    </UiStack>
  )
}
