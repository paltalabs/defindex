import { shortenAddress } from '@/helpers/shortenAddress'
import { DefindexMethod, useDefindexCallback } from '@/hooks/useDefindex'
import { fetchDefaultAddresses, IndexData } from '@/store/lib/features/walletStore'
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
export const AllIndexes = ({
  handleOpenDeployIndex,
  handleOpenDeposit
}: {
  handleOpenDeployIndex: (method: string, args?: any) => any,
  handleOpenDeposit: (method: string, args?: any) => any
}) => {
  const defindex = useDefindexCallback()
  const { activeChain, address } = useSorobanReact()
  const dispatch = useAppDispatch();
  const Indexes = useAppSelector(state => state.wallet.indexes)
  const isLoading = Indexes.isLoading
  const createdIndexes = Indexes.createdIndexes

  const getRoles = async () => {
    const selectedIndex = 'CD5NL55J4JYMALHCPIF3YADCWDRJNLM3XOFMZ6IOH5K4AOGINB4VB3BP'
    const manager: any = await defindex(
      DefindexMethod.GETMANAGER,
      selectedIndex,
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
        {(!isLoading && createdIndexes?.length != undefined) && <Tbody>
          {createdIndexes.map((index: IndexData, i: number) => (
            <Tr key={i}>
              <Td>{index.name ? index.name : index.address}</Td>
              <Td sx={{ cursor: 'pointer' }} textAlign={'center'}>
                <Tooltip
                  placement='bottom'
                  label={index.address}
                  textAlign={'center'}
                  rounded={'lg'}>
                  {index.address ? shortenAddress(index.address) : '-'}
                </Tooltip>
              </Td>
              <Td textAlign={'center'}>${index.balance}</Td>
              <Td textAlign={'center'}>{index.name?.includes('Blend USDC') ? '200' : '400'}</Td>
              <Td textAlign={'center'}>
                <Stat>
                  <StatHelpText>
                    <StatArrow type='increase' />
                    {index.name?.includes('Blend USDC') ? '11.31' : '23.36'}%
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
                    onClick={() => handleOpenDeposit('deposit', index)}
                  />
                </Tooltip>
                <Tooltip hasArrow label={'Withdraw'} rounded={'lg'}>
                  <IconButton
                    mx={1}
                    colorScheme='orange'
                    aria-label='withdraw'
                    size='sm'
                    icon={<ArrowLeftIcon __css={{ transform: 'rotate(-90deg)' }} />}
                    onClick={() => handleOpenDeposit('withdraw', index)}
                  />
                </Tooltip>
                <Tooltip hasArrow label={'Rebalance'} rounded={'lg'}>
                  <IconButton
                    mx={1}
                    colorScheme='teal'
                    aria-label='rebalance'
                    size='sm'
                    icon={<SettingsIcon />}
                    onClick={() => handleOpenDeployIndex('edit_index', index)}
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


export default AllIndexes