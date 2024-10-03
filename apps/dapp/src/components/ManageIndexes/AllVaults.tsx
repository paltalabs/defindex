import { shortenAddress } from '@/helpers/shortenAddress'
import { VaultMethod, useVaultCallback } from '@/hooks/useVault'
import { fetchDefaultAddresses, VaultData } from '@/store/lib/features/walletStore'
import { useAppDispatch, useAppSelector } from '@/store/lib/storeHooks'
import { ArrowLeftIcon, SettingsIcon } from '@chakra-ui/icons'
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
  const dispatch = useAppDispatch();
  const vaults = useAppSelector(state => state.wallet.vaults)
  const isLoading = vaults.isLoading
  const createdVaults = vaults.createdVaults

  const getRoles = async () => {
    const selectedVault = 'CD5NL55J4JYMALHCPIF3YADCWDRJNLM3XOFMZ6IOH5K4AOGINB4VB3BP'
    const manager: any = await vault(
      VaultMethod.GETMANAGER,
      selectedVault,
      undefined,
      false,
    )
    console.log('✨Manager', manager)
    const parsedManager = scValToNative(manager)
    console.log('✨Manager', parsedManager)
    /*  const emergencyManager: any = await defindex(
       DefindexMethod.GETEMERGENCYMANAGER,
       selectedIndex,
       undefined,
       false,
     )
     const parsedEmergencyManager = scValToNative(emergencyManager.returnValue)
     console.log('✨Emergency Manager', parsedEmergencyManager)
     const feeReceiver: any = await defindex(
       DefindexMethod.GETFEERECEIVER,
       selectedIndex,
       undefined,
       false,
     )
     const parsedFeeReceiver = scValToNative(feeReceiver.returnValue)
     console.log('✨Fee reciever', parsedFeeReceiver) */
  }
  useEffect(() => {
    dispatch(fetchDefaultAddresses(activeChain?.networkPassphrase!))
  }, [activeChain?.networkPassphrase]);
  return (
    <TableContainer
      mx={'auto'}
      minW={'100%'}
      p={4}>
      <Table variant="simple">
        <Thead>
          <Tr>
            <Th>Name</Th>
            <Th textAlign={'center'}>Address</Th>
            <Th textAlign={'center'}>Balance</Th>
            <Th textAlign={'center'}>Status</Th>
            <Th textAlign={'center'}>% APR</Th>
            <Th textAlign={'center'}>Options</Th>
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
              <Td textAlign={'center'}>${vault.balance}</Td>
              <Td textAlign={'center'}>{vault.name?.includes('Blend USDC') ? '200' : '400'}</Td>
              <Td textAlign={'center'}>
                <Stat>
                  <StatHelpText>
                    <StatArrow type='increase' />
                    {vault.name?.includes('Blend USDC') ? '11.31' : '23.36'}%
                  </StatHelpText>
                </Stat>
              </Td>
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
                <Tooltip hasArrow label={'Rebalance'} rounded={'lg'}>
                  <IconButton
                    mx={1}
                    colorScheme='teal'
                    aria-label='rebalance'
                    size='sm'
                    icon={<SettingsIcon />}
                    onClick={() => handleOpenDeployVault('edit_vault', vault)}
                  />
                </Tooltip>
                <Tooltip hasArrow label={'GetRole'} rounded={'lg'}>
                  <IconButton
                    mx={1}
                    colorScheme='teal'
                    aria-label='rebalance'
                    size='sm'
                    icon={<SettingsIcon />}
                    onClick={() => getRoles()}
                  />
                </Tooltip>
              </Td>
            </Tr>
          ))}
        </Tbody>}
      </Table>
    </TableContainer>
  )
}


export default AllVaults