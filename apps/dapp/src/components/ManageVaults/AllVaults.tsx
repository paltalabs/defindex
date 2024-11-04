import { shortenAddress } from '@/helpers/shortenAddress'
import { FactoryMethod, useFactoryCallback } from '@/hooks/useFactory'
import { VaultMethod, useVaultCallback } from '@/hooks/useVault'
import { fetchDefaultAddresses, setIsVaultsLoading, setVaults, VaultData } from '@/store/lib/features/walletStore'
import { useAppDispatch, useAppSelector } from '@/store/lib/storeHooks'
import { ArrowLeftIcon, SettingsIcon, WarningTwoIcon } from '@chakra-ui/icons'
import {
  Box,
  IconButton,
  Skeleton,
  Stack,
  Table,
  Text,
  useBreakpointValue,
  VStack,
} from '@chakra-ui/react'
import { useSorobanReact } from '@soroban-react/core'
import { scValToNative } from '@stellar/stellar-sdk'
import { useEffect, useState } from 'react'
import { Tooltip } from '../ui/tooltip'
import { StatRoot, StatUpTrend } from '../ui/stat'

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
  handleOpenDeployVault,
  handleOpenDeposit
}: {
    handleOpenDeployVault: (method: string, value: boolean, args?: any) => any,
  handleOpenDeposit: (method: string, args?: any) => any
}) => {
  const vault = useVaultCallback()
  const { activeChain, address } = useSorobanReact()
  const dispatch = useAppDispatch();
  const vaults = useAppSelector(state => state.wallet.vaults)
  const isLoading = vaults.isLoading
  const createdVaults = vaults.createdVaults
  const factory = useFactoryCallback()
  const isMobile = useBreakpointValue({ base: true, md: false });

  const getVaultInfo = async (selectedVault: string) => {
    try {
      //dispatch(setIsVaultsLoading(true))
      const [manager, emergencyManager, feeReceiver, name, strategies, totalValues] = await Promise.all([
        vault(VaultMethod.GETMANAGER, selectedVault, undefined, false).then((res: any) => scValToNative(res)),
        vault(VaultMethod.GETEMERGENCYMANAGER, selectedVault, undefined, false).then((res: any) => scValToNative(res)),
        vault(VaultMethod.GETFEERECEIVER, selectedVault, undefined, false).then((res: any) => scValToNative(res)),
        vault(VaultMethod.GETNAME, selectedVault, undefined, false).then((res: any) => scValToNative(res)),
        vault(VaultMethod.GETSTRATEGIES, selectedVault, undefined, false).then((res: any) => scValToNative(res)),
        vault(VaultMethod.GETTOTALVALUES, selectedVault, undefined, false).then((res: any) => scValToNative(res)),
      ]);
      const newData: VaultData = {
        name: name || '',
        address: selectedVault,
        manager: manager,
        emergencyManager: emergencyManager,
        feeReceiver: feeReceiver,
        strategies: strategies || [],
        totalValues: totalValues || 0,
      }
      return newData
    } catch (e: any) {
      if (e.toString().includes('MissingValue')) {
        console.warn(`The vault ${shortenAddress(selectedVault)} is missing some values, some features may not work as expected`)
      } else {
        console.error(e)
      }
      return {
        name: '',
        address: selectedVault,
        manager: undefined,
        emergencyManager: undefined,
        feeReceiver: undefined,
        strategies: [],
        totalValues: 0,
      }
    } finally {
      dispatch(setIsVaultsLoading(false))
    }
  }

  const getDefaultVaults = async () => {
    dispatch(setIsVaultsLoading(true))
    dispatch(fetchDefaultAddresses(activeChain?.networkPassphrase!))
  }
  useEffect(() => {

    getDefaultVaults()
  }, [activeChain?.networkPassphrase, address])

  const getDefindexVaults = async () => {
    try {
      const defindexVaults: any = await factory(FactoryMethod.DEPLOYED_DEFINDEXES)
      if (!defindexVaults) throw new Error('No defindex vaults found');
      const parsedDefindexVaults = scValToNative(defindexVaults)
      const defindexVaultsArray = []
      dispatch(setIsVaultsLoading(true))
      for (let vault in parsedDefindexVaults) {
        vault = parsedDefindexVaults[vault]
        const newData = await getVaultInfo(vault)
        defindexVaultsArray.push(newData)
      }
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
        <Table.Root >
          <Table.Header>
            <Table.Row>
              <Table.Cell>Name</Table.Cell>
              <Table.Cell textAlign={'center'}>Address</Table.Cell>
              <Table.Cell textAlign={'center'}>Balance</Table.Cell>
              <Table.Cell textAlign={'center'}>% APR</Table.Cell>
                {address && (
                <Table.Cell textAlign={'center'}>Options</Table.Cell>
                )}
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
                <Table.Row key={i}>
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
                  {address && (
                    <Table.Cell textAlign={'center'}>
                      <Tooltip content={'Deposit'}>
                        <IconButton
                          mx={1}
                          colorScheme='blue'
                          aria-label='deposit'
                          size='sm'
                          onClick={() => handleOpenDeposit(VaultMethod.DEPOSIT, vault)}
                        >
                          <ArrowLeftIcon css={{ transform: 'rotate(90deg)' }} />
                        </IconButton>
                      </Tooltip>
                      <Tooltip content={'Withdraw'}>
                        <IconButton
                          mx={1}
                          colorScheme='orange'
                          aria-label='withdraw'
                          size='sm'

                          onClick={() => handleOpenDeposit(VaultMethod.WITHDRAW, vault)}
                        >
                          <ArrowLeftIcon css={{ transform: 'rotate(-90deg)' }} />
                        </IconButton>
                      </Tooltip>
                      {(address == vault.manager) &&
                        <Tooltip content={'Rebalance'}>
                          <IconButton
                            mx={1}
                            colorScheme='teal'
                            aria-label='rebalance'
                            size='sm'
                            onClick={() => handleOpenDeployVault('edit_vault', true, vault)}
                          >
                            <SettingsIcon />
                          </IconButton>
                        </Tooltip>}
                      {(address == vault.emergencyManager || address == vault.manager) &&
                        <Tooltip content={'Emergency withdraw'}>
                          <IconButton
                            mx={1}
                            aria-label='emergency-withdraw'
                            size='sm'
                            onClick={() => handleOpenDeposit(VaultMethod.EMERGENCY_WITHDRAW, vault)}
                          >
                            <WarningTwoIcon />
                          </IconButton>
                        </Tooltip>}
                    </Table.Cell>
                  )}
                </Table.Row>
              ))}
          </Table.Body>}
        </Table.Root>
      ) : (
          <VStack>
          {createdVaults.map((vault: VaultData, i: number) => (
            <Box key={i} p={4} shadow="md" borderWidth="1px" borderRadius="lg" w="100%">
              <Text fontSize="lg" fontWeight="bold">{vault.name ? vault.name : shortenAddress(vault.address)}</Text>
              <Text >Address: {shortenAddress(vault.address)}</Text>
              <Text>Balance: ${vault.totalValues}</Text>
              <Text>APR: {vault.name?.includes('Blend USDC') ? '11.31' : '23.36'}%</Text>
              {address && (
                <Stack direction="row" mt={2}>
                  <Tooltip content={'Deposit'}>
                    <IconButton
                      colorScheme='blue'
                      aria-label={VaultMethod.DEPOSIT}
                      size='sm'
                      onClick={() => handleOpenDeposit(VaultMethod.DEPOSIT, vault)}
                    >
                      <ArrowLeftIcon __css={{ transform: 'rotate(90deg)' }} />
                    </IconButton>
                  </Tooltip>
                  <Tooltip content={'Withdraw'}>
                    <IconButton
                      colorScheme='orange'
                      aria-label={VaultMethod.WITHDRAW}
                      size='sm'
                      onClick={() => handleOpenDeposit(VaultMethod.WITHDRAW, vault)}
                    >
                      <ArrowLeftIcon __css={{ transform: 'rotate(-90deg)' }} />
                    </IconButton>
                  </Tooltip>
                  {(address == vault.manager) &&
                    <Tooltip content={'Rebalance'}>
                      <IconButton
                        colorScheme='teal'
                        aria-label='rebalance'
                        size='sm'
                        onClick={() => handleOpenDeployVault('edit_vault', true, vault)}
                      >
                        <SettingsIcon />
                      </IconButton>
                    </Tooltip>}
                  {(address == vault.emergencyManager || address == vault.manager) &&
                    <Tooltip content={'Emergency withdraw'}>
                      <IconButton
                        colorScheme='yellow'
                        aria-label={VaultMethod.EMERGENCY_WITHDRAW}
                        size='sm'
                        onClick={() => handleOpenDeposit(VaultMethod.EMERGENCY_WITHDRAW, vault)}
                      >
                        <WarningTwoIcon color={'white'} />
                      </IconButton>
                    </Tooltip>}
                </Stack>
              )}
            </Box>
          ))}
        </VStack>
      )}
    </Box>
  )
}

export default AllVaults
