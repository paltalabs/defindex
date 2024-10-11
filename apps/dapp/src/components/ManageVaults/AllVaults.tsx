import { shortenAddress } from '@/helpers/shortenAddress'
import { VaultMethod, useVaultCallback } from '@/hooks/useVault'
import { fetchDefaultAddresses, setIsVaultsLoading, setVaultRoles, VaultData } from '@/store/lib/features/walletStore'
import { useAppDispatch, useAppSelector } from '@/store/lib/storeHooks'
import { ArrowLeftIcon, SettingsIcon, WarningTwoIcon } from '@chakra-ui/icons'
import {
  Table,
  Thead,
  Tbody,
  Tr,
  Th,
  Td,
  TableContainer,
  Tooltip,
  Skeleton,
  Stat,
  StatHelpText,
  StatArrow,
  IconButton,
  Box,
  Stack,
  Text,
  VStack,
  useBreakpointValue
} from '@chakra-ui/react'
import { useSorobanReact } from '@soroban-react/core'
import { scValToNative } from '@stellar/stellar-sdk'
import { useEffect, useState } from 'react'

const SkeletonRow = () => {
  return (
    <Tr>
      <Td>
        <Skeleton height='20px' />
      </Td>
      <Td>
        <Skeleton height='20px' />
      </Td>
      <Td>
        <Skeleton height='20px' />
      </Td>
      <Td>
        <Skeleton height='20px' />
      </Td>
      <Td>
        <Skeleton height='20px' />
      </Td>
    </Tr>
  )
}
export const AllVaults = ({
  handleOpenDeployVault,
  handleOpenDeposit
}: {
  handleOpenDeployVault: (method: string, args?: any) => any,
  handleOpenDeposit: (method: string, args?: any) => any
}) => {
  const vault = useVaultCallback()
  const { activeChain, address } = useSorobanReact()
  // const { address } = useSorobanReact()
  // const activeChain = { id: "testnet", name: "testnet", networkPassphrase: "Test SDF Network ; September 2015" } // REMOVE_THIS

  const dispatch = useAppDispatch();
  const vaults = useAppSelector(state => state.wallet.vaults)
  const isLoading = vaults.isLoading
  const createdVaults = vaults.createdVaults

  const isMobile = useBreakpointValue({ base: true, md: false });

  const getRoles = async (selectedVault: string) => {
    setIsVaultsLoading(true)
    try {
      const manager: any = await vault(
        VaultMethod.GETMANAGER,
        selectedVault,
        undefined,
        false,
      )
      const emergencyManager: any = await vault(
        VaultMethod.GETEMERGENCYMANAGER,
        selectedVault,
        undefined,
        false,
      )
      const feeReceiver: any = await vault(
        VaultMethod.GETFEERECEIVER,
        selectedVault,
        undefined,
        false,
      )
      const parsedManager = scValToNative(manager)
      const parsedEmergencyManager = scValToNative(emergencyManager)
      const parsedFeeReceiver = scValToNative(feeReceiver)
      return {
        address: selectedVault,
        manager: parsedManager,
        emergencyManager: parsedEmergencyManager,
        feeReceiver: parsedFeeReceiver
      }
    } catch (e: any) {
      if (e.toString().includes('MissingValue')) {
        console.warn(`The vault ${shortenAddress(selectedVault)} is missing some values, some features may not work as expected`)
      } else {
        console.error(e)
      }
      return {
        address: selectedVault,
        manager: undefined,
        emergencyManager: undefined,
        feeReceiver: undefined
      }
    } finally {
      setIsVaultsLoading(false)
    }
  }

  const fetchVaultRoles = async () => {
    for (const vault of createdVaults) {
      const roles = await getRoles(vault.address)
      await dispatch(setVaultRoles(roles))
    }
  }

  useEffect(() => {
    dispatch(fetchDefaultAddresses(activeChain?.networkPassphrase!))
  }, [activeChain?.networkPassphrase, address]);

  useEffect(() => {
    if (createdVaults?.length > 0) {
      fetchVaultRoles()
    }
  }, [createdVaults.length, address])

  return (
    <Box mx={'auto'} minW={'100%'} p={4}>
      {!isMobile ? (
        <TableContainer>
          <Table variant="simple">
            <Thead>
              <Tr>
                <Th>Name</Th>
                <Th textAlign={'center'}>Address</Th>
                <Th textAlign={'center'}>Balance</Th>
                <Th textAlign={'center'}>Status</Th>
                <Th textAlign={'center'}>% APR</Th>
                {address && (
                  <Th textAlign={'center'}>Options</Th>
                )}
              </Tr>
            </Thead>
            {isLoading && <Tbody>
              <SkeletonRow />
              <SkeletonRow />
              <SkeletonRow />
            </Tbody>}
            {(!isLoading && createdVaults?.length != undefined) && <Tbody>
              {createdVaults.map((vault: VaultData, i: number) => (
                <Tr key={i}>
                  <Td>{vault.name ? vault.name : vault.address}</Td>
                  <Td sx={{ cursor: 'pointer' }} textAlign={'center'}>
                    <Tooltip
                      placement='bottom'
                      label={vault.address}
                      textAlign={'center'}
                      rounded={'lg'}>
                      {vault.address ? shortenAddress(vault.address) : '-'}
                    </Tooltip>
                  </Td>
                  <Td textAlign={'center'}>${vault.totalValues}</Td>
                  <Td textAlign={'center'}>{vault.name?.includes('Blend USDC') ? '200' : '400'}</Td>
                  <Td textAlign={'center'}>
                    <Stat>
                      <StatHelpText>
                        <StatArrow type='increase' />
                        {vault.name?.includes('Blend USDC') ? '11.31' : '23.36'}%
                      </StatHelpText>
                    </Stat>
                  </Td>
                  {address && (
                    <Td textAlign={'center'}>
                      <Tooltip hasArrow label={'Deposit'} rounded={'lg'}>
                        <IconButton
                          mx={1}
                          colorScheme='blue'
                          aria-label='deposit'
                          size='sm'
                          icon={<ArrowLeftIcon __css={{ transform: 'rotate(90deg)' }} />}
                          onClick={() => handleOpenDeposit('deposit', vault)}
                        />
                      </Tooltip>
                      <Tooltip hasArrow label={'Withdraw'} rounded={'lg'}>
                        <IconButton
                          mx={1}
                          colorScheme='orange'
                          aria-label='withdraw'
                          size='sm'
                          icon={<ArrowLeftIcon __css={{ transform: 'rotate(-90deg)' }} />}
                          onClick={() => handleOpenDeposit('withdraw', vault)}
                        />
                      </Tooltip>
                      {(address == vault.manager) &&
                        <Tooltip hasArrow label={'Rebalance'} rounded={'lg'}>
                          <IconButton
                            mx={1}
                            colorScheme='teal'
                            aria-label='rebalance'
                            size='sm'
                            icon={<SettingsIcon />}
                            onClick={() => handleOpenDeployVault('edit_vault', vault)}
                          />
                        </Tooltip>}
                      {(address == vault.emergencyManager || address == vault.manager) &&
                        <Tooltip hasArrow label={'Emergency withdraw'} rounded={'lg'}>
                          <IconButton
                            mx={1}
                            colorScheme='yellow'
                            aria-label='emergency-withdraw'
                            size='sm'
                            icon={<WarningTwoIcon color={'white'} />}
                            onClick={() => handleOpenDeposit('emergency_withdraw', vault)}
                          />
                        </Tooltip>}
                    </Td>
                  )}
                </Tr>
              ))}
            </Tbody>}
          </Table>
        </TableContainer>
      ) : (
        <VStack spacing={4}>
          {createdVaults.map((vault: VaultData, i: number) => (
            <Box key={i} p={4} shadow="md" borderWidth="1px" borderRadius="lg" w="100%">
              <Text fontSize="lg" fontWeight="bold">{vault.name ? vault.name : vault.address}</Text>
              <Text >Address: {shortenAddress(vault.address)}</Text>
              <Text>Balance: ${vault.totalValues}</Text>
              <Text>Status: {vault.name?.includes('Blend USDC') ? '200' : '400'}</Text>
              <Text>APR: {vault.name?.includes('Blend USDC') ? '11.31' : '23.36'}%</Text>
              {address && (
                <Stack direction="row" spacing={4} mt={2}>
                  <Tooltip hasArrow label={'Deposit'} rounded={'lg'}>
                    <IconButton
                      colorScheme='blue'
                      aria-label='deposit'
                      size='sm'
                      icon={<ArrowLeftIcon __css={{ transform: 'rotate(90deg)' }} />}
                      onClick={() => handleOpenDeposit('deposit', vault)}
                    />
                  </Tooltip>
                  <Tooltip hasArrow label={'Withdraw'} rounded={'lg'}>
                    <IconButton
                      colorScheme='orange'
                      aria-label='withdraw'
                      size='sm'
                      icon={<ArrowLeftIcon __css={{ transform: 'rotate(-90deg)' }} />}
                      onClick={() => handleOpenDeposit('withdraw', vault)}
                    />
                  </Tooltip>
                  {(address == vault.manager) &&
                    <Tooltip hasArrow label={'Rebalance'} rounded={'lg'}>
                      <IconButton
                        colorScheme='teal'
                        aria-label='rebalance'
                        size='sm'
                        icon={<SettingsIcon />}
                        onClick={() => handleOpenDeployVault('edit_vault', vault)}
                      />
                    </Tooltip>}
                  {(address == vault.emergencyManager || address == vault.manager) &&
                    <Tooltip hasArrow label={'Emergency withdraw'} rounded={'lg'}>
                      <IconButton
                        colorScheme='yellow'
                        aria-label='emergency-withdraw'
                        size='sm'
                        icon={<WarningTwoIcon color={'white'} />}
                        onClick={() => handleOpenDeposit('emergency_withdraw', vault)}
                      />
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
