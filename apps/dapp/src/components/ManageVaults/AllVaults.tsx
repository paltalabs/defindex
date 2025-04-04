import { useSorobanReact } from '@soroban-react/core'
import { useEffect } from 'react'

import { shortenAddress } from '@/helpers/address'
import { FactoryMethod, useFactoryCallback } from '@/hooks/useFactory'
import { useVault } from '@/hooks/useVault'

import { setIsVaultsLoading, setVaults, setVaultTVL, setVaultUserBalance } from '@/store/lib/features/walletStore'
import { useAppDispatch, useAppSelector } from '@/store/lib/storeHooks'
import { VaultData } from '@/store/lib/types'

import {
  Box,
  Skeleton,
  Table,
  Text,
  useBreakpointValue,
  VStack,
} from '@chakra-ui/react'
import { nativeToScVal, scValToNative } from '@stellar/stellar-sdk'
import { Tooltip } from '../ui/tooltip'

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
      {address && (
        <Table.Cell>
          <Skeleton height='20px' />
        </Table.Cell>
      )}
    </Table.Row>
  )
}
export const AllVaults = ({
  handleOpenInspect
}: {
    handleOpenInspect: (value: boolean, args?: any) => any
  }) => {
  const { activeChain, address } = useSorobanReact()
  // const { address } = useSorobanReact()
  // const activeChain = { id: "testnet", name: "testnet", networkPassphrase: "Test SDF Network ; September 2015" } // REMOVE_THIS

  const dispatch = useAppDispatch();
  const vaults = useAppSelector(state => state.wallet.vaults)
  const isLoading = vaults.isLoading
  const createdVaults = vaults.createdVaults
  const factory = useFactoryCallback()
  const vault = useVault()
  const isMobile = useBreakpointValue({ base: true, md: false });
  const { getVaultInfo } = useVault()

  const getDefindexVaults = async () => {
    dispatch(setIsVaultsLoading(true))
    try {
      const defindexVaultsRaw: any = await factory(FactoryMethod.TOTAL_VAULTS)
      if (!defindexVaultsRaw) throw new Error('No defindex vaults found');
      const defindexVaults: any = scValToNative(defindexVaultsRaw)
      // const parsedDefindexVaults = scValToNative(defindexVaults)
      const defindexVaultsArray: VaultData[] = []
      dispatch(setIsVaultsLoading(true))

      for (let i = 0; i < defindexVaults; i++) {
        const vaultAddressScVal: any = await factory(FactoryMethod.GET_VAULT_BY_INDEX, [nativeToScVal(i, {type: "u32"})])
        const vaultAddress = scValToNative(vaultAddressScVal)
        const vaultInfo = await getVaultInfo(vaultAddress)
        if (!vaultInfo) continue;
        defindexVaultsArray.push(vaultInfo)
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
  }, [activeChain?.networkPassphrase])

  useEffect(() => {
    createdVaults.forEach(async (v: VaultData) => {
      const TVL = await vault.getTVL(v.address)
      if (TVL) {
        dispatch(setVaultTVL({ value: TVL, address: v.address }))
      }
    })

  }, [createdVaults])

  useEffect(() => {
    if (address) {
      createdVaults.forEach(async (v: VaultData) => {
        const userBalance = await vault.getUserBalance(v.address, address)
        if (userBalance) {
          dispatch(setVaultUserBalance({ address: v.address, vaule: userBalance }))
        }
      })
    }
  }, [createdVaults, address])

  return (
    <Box mx={'auto'} minW={'100%'} p={4}>
      {!isMobile ? (
        <Table.Root interactive>
          <Table.Header>
            <Table.Row>
              <Table.Cell>Name</Table.Cell>
              <Table.Cell textAlign={'center'}>Address</Table.Cell>
              <Table.Cell textAlign={'center'}>TVL</Table.Cell>
              {address && <Table.Cell textAlign={'center'}>User Balance</Table.Cell>}
              <Table.Cell textAlign={'center'}>Asset</Table.Cell>
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
                  <Table.Cell textAlign={'center'}>${vault.TVL}</Table.Cell>
                  {address && <Table.Cell textAlign={'center'}>${vault.userBalance ? `${vault.userBalance}` : 0}</Table.Cell>}
                  <Table.Cell textAlign={'center'}>
                    {vault.assets[0]?.symbol}
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
              <Text>TVL: ${vault.TVL}</Text>
              {address && <Text>User balance: ${vault.userBalance ? `${vault.userBalance}` : 0}</Text>}
              <Text>Asset: {vault.assets[0]?.symbol}</Text>
            </Box>
          ))}
        </VStack>
      )}
    </Box>
  )
}

export default AllVaults
