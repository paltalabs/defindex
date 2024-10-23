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
  Stat,
  StatArrow,
  StatHelpText,
  Table, Thead, Tbody, Tr, Th, Td, TableContainer,
  Text,
  Tooltip,
  useBreakpointValue,
  VStack,
} from '@chakra-ui/react'
import { useSorobanReact } from '@soroban-react/core'
import { scValToNative } from '@stellar/stellar-sdk'
import { useEffect } from 'react'

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
  const dispatch = useAppDispatch();
  const vaults = useAppSelector(state => state.wallet.vaults)
  const isLoading = vaults.isLoading
  const createdVaults = vaults.createdVaults
  const factory = useFactoryCallback()

  const isMobile = useBreakpointValue({ base: true, md: false });

  const getVaultInfo = async (selectedVault: string) => {
    try {
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
    }
  }

  const getDefindexVaults = async () => {
    const defindexVaults: any = await factory(FactoryMethod.DEPLOYED_DEFINDEXES)
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
    console.log(defindexVaultsArray, '🟡 defindexVaultsArray')
  }


  useEffect(() => {
    getDefindexVaults()
  }, [activeChain?.networkPassphrase, address])

  return (
    <Box mx={'auto'} minW={'100%'} p={4}>
      {!isMobile ? (
        <TableContainer >
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
                          onClick={() => handleOpenDeposit(VaultMethod.DEPOSIT, vault)}
                        />
                      </Tooltip>
                      <Tooltip hasArrow label={'Withdraw'} rounded={'lg'}>
                        <IconButton
                          mx={1}
                          colorScheme='orange'
                          aria-label='withdraw'
                          size='sm'
                          icon={<ArrowLeftIcon __css={{ transform: 'rotate(-90deg)' }} />}
                          onClick={() => handleOpenDeposit(VaultMethod.WITHDRAW, vault)}
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
                            onClick={() => handleOpenDeposit(VaultMethod.EMERGENCY_WITHDRAW, vault)}
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
              <Text fontSize="lg" fontWeight="bold">{vault.name ? vault.name : shortenAddress(vault.address)}</Text>
              <Text >Address: {shortenAddress(vault.address)}</Text>
              <Text>Balance: ${vault.totalValues}</Text>
              <Text>Status: {vault.name?.includes('Blend USDC') ? '200' : '400'}</Text>
              <Text>APR: {vault.name?.includes('Blend USDC') ? '11.31' : '23.36'}%</Text>
              {address && (
                <Stack direction="row" spacing={4} mt={2}>
                  <Tooltip hasArrow label={'Deposit'} rounded={'lg'}>
                    <IconButton
                      colorScheme='blue'
                      aria-label={VaultMethod.DEPOSIT}
                      size='sm'
                      icon={<ArrowLeftIcon __css={{ transform: 'rotate(90deg)' }} />}
                      onClick={() => handleOpenDeposit(VaultMethod.DEPOSIT, vault)}
                    />
                  </Tooltip>
                  <Tooltip hasArrow label={'Withdraw'} rounded={'lg'}>
                    <IconButton
                      colorScheme='orange'
                      aria-label={VaultMethod.WITHDRAW}
                      size='sm'
                      icon={<ArrowLeftIcon __css={{ transform: 'rotate(-90deg)' }} />}
                      onClick={() => handleOpenDeposit(VaultMethod.WITHDRAW, vault)}
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
                        aria-label={VaultMethod.EMERGENCY_WITHDRAW}
                        size='sm'
                        icon={<WarningTwoIcon color={'white'} />}
                        onClick={() => handleOpenDeposit(VaultMethod.EMERGENCY_WITHDRAW, vault)}
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
