import { Group, Text } from '@mantine/core'
import { UiCard, UiStack } from '@pubkey-ui/core'
import { useWallet } from '@solana/wallet-adapter-react'
import { ellipsify } from '../account/ui/ellipsify'
import { ExplorerLink } from '../cluster/cluster-ui'
import { WalletButton } from '../solana/solana-provider'
import { useTokengatorPresetProgram } from './data-access/use-tokengator-preset-program'

import { TokengatorPresetUiCreate } from './data/tokengator-preset-ui-create'
import { TokengatorPresetUiList } from './data/tokengator-preset-ui-list'

export default function TokengatorPresetFeature() {
  const { publicKey } = useWallet()
  const { programId } = useTokengatorPresetProgram()

  return publicKey ? (
    <UiStack>
      <UiCard title="TokenGator Preset">
        <Text>
          Create a new account by clicking the "Create" button. The state of a account is stored on-chain and can be
          manipulated by calling the program's methods (increment, decrement, set, and close).
        </Text>
        <Group justify="flex-end">
          <ExplorerLink path={`account/${programId}`} label={ellipsify(programId.toString())} />
          <TokengatorPresetUiCreate />
        </Group>
      </UiCard>

      <TokengatorPresetUiList />
    </UiStack>
  ) : (
    <WalletButton />
  )
}
