import { Anchor, AnchorProps, Button, Group, Menu, Select, Table, Text, TextInput } from '@mantine/core'
import { modals } from '@mantine/modals'
import { UiCopy, UiStack, UiWarning } from '@pubkey-ui/core'
import { useConnection } from '@solana/wallet-adapter-react'
import { IconNetwork, IconNetworkOff, IconTrash } from '@tabler/icons-react'
import { useQuery } from '@tanstack/react-query'
import { ReactNode, useState } from 'react'
import { Link } from 'react-router-dom'
import { ClusterNetwork, useCluster } from './cluster-data-access'

export function ExplorerLink({
  copy,
  path,
  label = 'View on Explorer',
  ...props
}: { path: string; copy?: string; label?: ReactNode } & AnchorProps) {
  const { getExplorerUrl } = useCluster()
  return (
    <Group align="start" gap={4} wrap="nowrap">
      {copy ? <UiCopy text={copy} /> : null}
      <Anchor href={getExplorerUrl(path)} target="_blank" rel="noopener noreferrer" {...props}>
        {label}
      </Anchor>
    </Group>
  )
}

export function ClusterUiSelect() {
  const { clusters, setCluster, cluster } = useCluster()
  return (
    <Menu shadow="md" width={200}>
      <Menu.Target>
        <Button>{cluster.name}</Button>
      </Menu.Target>

      <Menu.Dropdown>
        {clusters.map((item) => (
          <Menu.Item
            key={item.name}
            onClick={() => setCluster(item)}
            leftSection={item.active ? <IconNetwork /> : <IconNetworkOff />}
          >
            {item.name}
          </Menu.Item>
        ))}
        <Menu.Divider />
        <Menu.Item component={Link} to="/clusters">
          Manage Clusters
        </Menu.Item>
      </Menu.Dropdown>
    </Menu>
  )
}

export function ClusterChecker({ children }: { children: ReactNode }) {
  const { cluster } = useCluster()
  const { connection } = useConnection()

  const query = useQuery({
    queryKey: ['version', { cluster, endpoint: connection.rpcEndpoint }],
    queryFn: () => connection.getVersion(),
    retry: 1,
  })
  if (query.isLoading) {
    return null
  }
  if (query.isError || !query.data) {
    return (
      <UiWarning
        mb="xl"
        styles={{
          root: { display: 'flex', justifyContent: 'center' },
          title: { justifyContent: 'center' },
        }}
        title="Error connecting to cluster"
        icon={undefined}
        message={
          <Group justify="center">
            <Text>
              Error connecting to cluster <strong>{cluster.name}</strong>
            </Text>
            <Button variant="light" color="yellow" size="xs" onClick={() => query.refetch()}>
              Refresh
            </Button>
          </Group>
        }
      />
    )
  }
  return children
}

export function ClusterUiModal() {
  const { addCluster } = useCluster()
  const [name, setName] = useState('')
  const [network, setNetwork] = useState<ClusterNetwork | undefined>(ClusterNetwork.Devnet)
  const [endpoint, setEndpoint] = useState('')

  return (
    <Button
      onClick={() => {
        modals.open({
          title: 'Add Cluster',
          children: (
            <UiStack>
              <TextInput type="text" placeholder="Name" value={name} onChange={(e) => setName(e.target.value)} />
              <TextInput
                type="text"
                placeholder="Endpoint"
                value={endpoint}
                onChange={(e) => setEndpoint(e.target.value)}
              />
              <Select
                value={network}
                onChange={(value) => setNetwork(value as ClusterNetwork)}
                data={[
                  { value: ClusterNetwork.Custom, label: 'Custom' },
                  { value: ClusterNetwork.Devnet, label: 'Devnet' },
                  { value: ClusterNetwork.Testnet, label: 'Testnet' },
                  { value: ClusterNetwork.Mainnet, label: 'Mainnet' },
                ]}
              />
              <Group justify="flex-end">
                <Button
                  onClick={() => {
                    addCluster({ name, network, endpoint })
                    modals.closeAll()
                  }}
                >
                  Save
                </Button>
              </Group>
            </UiStack>
          ),
        })
      }}
    >
      Add Cluster
    </Button>
  )
}

export function ClusterUiTable() {
  const { clusters, setCluster, deleteCluster } = useCluster()
  return (
    <div>
      <Table>
        <Table.Thead>
          <Table.Tr>
            <Table.Th>Name / Network / Endpoint</Table.Th>
            <Table.Th align="center">Actions</Table.Th>
          </Table.Tr>
        </Table.Thead>
        <Table.Tbody>
          {clusters.map((item) => (
            <Table.Tr key={item.name}>
              <Table.Td>
                <Text size="lg">
                  {item?.active ? (
                    item.name
                  ) : (
                    <Anchor component="button" title="Select cluster" onClick={() => setCluster(item)}>
                      {item.name}
                    </Anchor>
                  )}
                </Text>
                <Text size="xs">Network: {item.network ?? 'custom'}</Text>
                <div>{item.endpoint}</div>
              </Table.Td>
              <Table.Td>
                <Button
                  disabled={item?.active}
                  onClick={() => {
                    if (!window.confirm('Are you sure?')) return
                    deleteCluster(item)
                  }}
                >
                  <IconTrash size={16} />
                </Button>
              </Table.Td>
            </Table.Tr>
          ))}
        </Table.Tbody>
      </Table>
    </div>
  )
}
