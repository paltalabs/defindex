import React, { useState } from 'react'
import {
  Box,
  Table,
  Thead,
  Tbody,
  Tr,
  Th,
  Td,
  TableContainer,
  Tooltip,
  Text,
  Grid,
  GridItem,
  Input,
} from '@chakra-ui/react'
import { shortenAddress } from '@/helpers/shortenAddress'
import { PieChart } from '@mui/x-charts'
import { ChartData } from './ConfirmDelpoyModal'
import { setEmergencyManager, setFeeReceiver, setManager } from '@/store/lib/features/vaultStore'
import { useAppDispatch } from '@/store/lib/storeHooks'
import { StrKey } from '@stellar/stellar-sdk'


interface FormControlInterface {
  manager: {
    isValid: boolean | undefined;
  },
  emergencyManager: {
    isValid: boolean | undefined;
  },
  feeReceiver: {
    isValid: boolean | undefined;
  },
}
export const VaultPreview = ({ data }: { data: ChartData[] }) => {

  const dispatch = useAppDispatch()
  const [formControl, setFormControl] = useState<FormControlInterface>({
    manager: {
      isValid: undefined,
    },
    emergencyManager: {
      isValid: undefined,
    },
    feeReceiver: {
      isValid: undefined,
    },
  })
  const isValidAddress = (address: string) => {
    if (StrKey.isValidEd25519PublicKey(address) || StrKey.isValidMed25519PublicKey(address) || StrKey.isValidContract(address)) {
      return true
    } else {
      return false
    }
  }
  const handleManagerChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const isValid = isValidAddress(event.target.value)
    while (!isValid) {
      setFormControl({
        ...formControl,
        manager: {
          isValid: false,
        }
      })
      dispatch(setManager(''))
      return
    }
    setFormControl({
      ...formControl,
      manager: {
        isValid: true,
      }
    })
    dispatch(setManager(event.target.value))
  };

  const handleEmergencyManagerChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const isValid = isValidAddress(event.target.value)
    while (!isValid) {
      setFormControl({
        ...formControl,
        emergencyManager: {
          isValid: false,
        }
      })
      dispatch(setEmergencyManager(''))
      return
    }
    setFormControl({
      ...formControl,
      emergencyManager: {
        isValid: true,
      }
    })
    dispatch(setEmergencyManager(event.target.value))
  };
  const handleFeeReceiverChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const isValid = isValidAddress(event.target.value)
    while (!isValid) {
      setFormControl({
        ...formControl,
        feeReceiver: {
          isValid: false,
        }
      })
      dispatch(setFeeReceiver(''))
      return
    }
    setFormControl({
      ...formControl,
      feeReceiver: {
        isValid: true,
      }
    })
    dispatch(setFeeReceiver(event.target.value))
  };

  return (
    <>
      <Box display='flex' my={4}>
        <PieChart
          series={[
            {
              data: data,
            },
          ]}
          width={500}
          height={200}
        />
      </Box>
      <Text fontSize='lg' fontWeight='bold' mb={2}>
        Strategies
      </Text>
      <TableContainer>
        <Table variant={'simple'}>
          <Thead>
            <Tr>
              <Th>Name</Th>
              <Th textAlign={'center'}>Address</Th>
              <Th textAlign={'end'}>Percentage</Th>
            </Tr>
          </Thead>
          <Tbody>
            {data.map((strategy: ChartData, index: number) => (
              <Tr key={index} sx={{ cursor: 'default' }}>
                <Td>{strategy.label}</Td>
                <Td sx={{ cursor: 'pointer' }} textAlign={'center'}>
                  <Tooltip label={strategy.address}>
                    {strategy.address ? shortenAddress(strategy.address) : '-'}
                  </Tooltip>
                </Td>
                <Td textAlign={'end'}>{strategy.value}%</Td>
              </Tr>
            ))}
          </Tbody>
        </Table>
      </TableContainer>
      <Box height="20px" />
      <Grid
        templateColumns={'repeat(8, 2fr)'}
        templateRows={'repeat(5, 1fr)'}
        alignSelf={'end'}
        alignContent={'center'}
        mb={4}
      >
        <GridItem colStart={2} colSpan={3} rowStart={1}>
          <Text mt={4}>Manager</Text>
        </GridItem>
        <GridItem colStart={5} colSpan={3} rowStart={1}>
          <Input
            onChange={handleManagerChange}
            placeholder='GAFS3TLVM...'
            isRequired
            isInvalid={formControl.manager.isValid === false}
          />
        </GridItem>
        <GridItem colStart={2} colSpan={3} rowStart={3}>
          <Text mt={4}>Emergency Manager</Text>
        </GridItem>
        <GridItem colStart={5} colSpan={3} rowStart={3}>
          <Input
            onChange={handleEmergencyManagerChange}
            placeholder='GAFS3TLVM...'
            isRequired
            isInvalid={formControl.emergencyManager.isValid === false}
          />
        </GridItem>
        <GridItem colStart={2} colSpan={3} rowStart={5}>
          <Text mt={4}>Fee Receiver</Text>
        </GridItem>
        <GridItem colStart={5} colSpan={3} rowStart={5}>
          <Input
            onChange={handleFeeReceiverChange}
            placeholder='GAFS3TLVM...'
            isRequired
            isInvalid={formControl.feeReceiver.isValid === false}
          />
        </GridItem>
      </Grid>
    </>
  )
}
