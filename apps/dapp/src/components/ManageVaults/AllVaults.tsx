import { useEffect } from 'react'
import { shortenAddress } from '@/helpers/shortenAddress'
import { FactoryMethod, useFactoryCallback } from '@/hooks/useFactory'
import { useVault } from '@/hooks/useVault'
import { useSorobanReact } from '@soroban-react/core'
import { setIsVaultsLoading, setVaults, VaultData } from '@/store/lib/features/walletStore'
import { useAppDispatch, useAppSelector } from '@/store/lib/storeHooks'
import { scValToNative } from '@stellar/stellar-sdk'
import { Tooltip } from '../ui/tooltip'
import { StatRoot, StatUpTrend } from '../ui/stat'
import {
  Box,
  Skeleton,
  Table,
  Text,
  useBreakpointValue,
  VStack,
} from '@chakra-ui/react'

const SkeletonRow = () => {
  const { address } = useSorobanReact()
  return (
    <Table.Row>
      <Table.Cell>
        <Skeleton height='20px' />
      </Table.Cell>
      <Table.Cell>
        <Skeleton height='20px' />
      </Table.Cell>
      <Table.Cell>
        <Skeleton height='20px' />
      </Table.Cell>
      <Table.Cell>
        <Skeleton height='20px' />
      </Table.Cell>
    </Table.Row>
  )
}
export const AllVaults = ({
  handleOpenInspect
}: {
    handleOpenInspect: (value: boolean, args?: any) => any
  }) => {
  const { activeChain, address } = useSorobanReact()
  const dispatch = useAppDispatch();
  const vaults = useAppSelector(state => state.wallet.vaults)
  const isLoading = vaults.isLoading
  const createdVaults = vaults.createdVaults
  const factory = useFactoryCallback()
  const isMobile = useBreakpointValue({ base: true, md: false });
  const { getVaultInfo } = useVault()

  const getDefindexVaults = async () => {
    dispatch(setIsVaultsLoading(true))
    try {
      const defindexVaults: any = await factory(FactoryMethod.DEPLOYED_DEFINDEXES)
      if (!defindexVaults) throw new Error('No defindex vaults found');
      const parsedDefindexVaults = scValToNative(defindexVaults)
      const defindexVaultsArray: VaultData[] = []
      dispatch(setIsVaultsLoading(true))
      for (let vault in parsedDefindexVaults) {
        vault = parsedDefindexVaults[vault]
        const newData = await getVaultInfo(vault)
        if (!newData) continue;
        defindexVaultsArray.push(newData)
      }
      if (defindexVaultsArray.length === 0) throw new Error('No defindex vaults found');
      dispatch(setVaults(defindexVaultsArray))
      dispatch(setIsVaultsLoading(false))
    } catch (e: any) {
      dispatch(setIsVaultsLoading(false))
      console.error(e)
    }
  }


  useEffect(() => {
    getDefindexVaults()
  }, [activeChain?.networkPassphrase, address])

  return (
    <Box mx={'auto'} minW={'100%'} p={4}>
      {!isMobile ? (
        <Table.Root interactive>
          <Table.Header>
            <Table.Row>
              <Table.Cell>Name</Table.Cell>
              <Table.Cell textAlign={'center'}>Address</Table.Cell>
              <Table.Cell textAlign={'center'}>Balance</Table.Cell>
              <Table.Cell textAlign={'center'}>% APR</Table.Cell>
            </Table.Row>
          </Table.Header>
          {isLoading && <Table.Body>
              <SkeletonRow />
              <SkeletonRow />
              <SkeletonRow />
            <SkeletonRow />
          </Table.Body>}
          {(!isLoading && createdVaults?.length != undefined) && <Table.Body>
              {createdVaults.map((vault: VaultData, i: number) => (
                <Table.Row key={i} onClick={() => { handleOpenInspect(true, vault) }} css={{ cursor: 'pointer' }}>
                  <Table.Cell>{vault.name ? vault.name : vault.address}</Table.Cell>
                  <Table.Cell textAlign={'center'}>
                    <Tooltip content={vault.address}>
                      <p>
                        {vault.address ? shortenAddress(vault.address) : '-'}
                      </p>
                    </Tooltip>
                  </Table.Cell>
                  <Table.Cell textAlign={'center'}>${vault.totalValues}</Table.Cell>
                  <Table.Cell textAlign={'center'}>
                    <StatRoot>
                      <StatUpTrend justifyContent={'center'}>
                        {vault.name?.includes('blend') ? 11.31 : 23.36}
                      </StatUpTrend>
                    </StatRoot>
                  </Table.Cell>
                </Table.Row>
              ))}
          </Table.Body>}
        </Table.Root>
      ) : (
          <VStack>
          {createdVaults.map((vault: VaultData, i: number) => (
            <Box key={i} p={4} shadow="md" borderWidth="1px" borderRadius="lg" w="100%" onClick={() => { handleOpenInspect(true, vault) }} css={{ cursor: 'pointer' }}>
              <Text fontSize="lg" fontWeight="bold">{vault.name ? vault.name : shortenAddress(vault.address)}</Text>
              <Text >Address: {shortenAddress(vault.address)}</Text>
              <Text>Balance: ${vault.totalValues}</Text>
              <Text>APR: {vault.name?.includes('Blend USDC') ? '11.31' : '23.36'}%</Text>
            </Box>
          ))}
        </VStack>
      )}
    </Box>
  )
}

export default AllVaults
