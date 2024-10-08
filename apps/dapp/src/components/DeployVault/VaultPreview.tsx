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
  FormControl,
  FormLabel,
  FormErrorMessage,
  InputGroup,
  IconButton,
  InputRightElement,
} from '@chakra-ui/react'
import { shortenAddress } from '@/helpers/shortenAddress'
import { PieChart } from '@mui/x-charts'
import { ChartData } from './ConfirmDelpoyModal'
import { setEmergencyManager, setFeeReceiver, setManager } from '@/store/lib/features/vaultStore'
import { useAppDispatch } from '@/store/lib/storeHooks'
import { StrKey } from '@stellar/stellar-sdk'
import { LinkIcon } from '@chakra-ui/icons'
import { useSorobanReact } from '@soroban-react/core'


interface FormControlInterface {
  manager: {
    isValid: boolean | undefined;
    value: string | undefined;
  },
  emergencyManager: {
    isValid: boolean | undefined;
    value: string | undefined;
  },
  feeReceiver: {
    isValid: boolean | undefined;
    value: string | undefined;
  },
}
export const VaultPreview = ({ data }: { data: ChartData[] }) => {

  const dispatch = useAppDispatch()
  const { address } = useSorobanReact()
  const [formControl, setFormControl] = useState<FormControlInterface>({
    manager: {
      isValid: undefined,
      value: undefined
    },
    emergencyManager: {
      isValid: undefined,
      value: undefined
    },
    feeReceiver: {
      isValid: undefined,
      value: undefined
    },
  })
  const isValidAddress = (address: string) => {
    if (StrKey.isValidEd25519PublicKey(address) || StrKey.isValidMed25519PublicKey(address) || StrKey.isValidContract(address)) {
      return true
    } else {
      return false
    }
  }
  const handleManagerChange = (input: string) => {
    const isValid = isValidAddress(input)
    while (!isValid) {
      setFormControl({
        ...formControl,
        manager: {
          value: input,
          isValid: false,
        }
      })
      dispatch(setManager(''))
      return
    }
    if (isValid) {
      setFormControl({
        ...formControl,
        manager: {
          value: input,
          isValid: true
        }
      })
      dispatch(setManager(input))
    }
    return;
  };

  const handleEmergencyManagerChange = (input: string) => {
    const isValid = isValidAddress(input)
    while (!isValid) {
      setFormControl({
        ...formControl,
        emergencyManager: {
          value: input,
          isValid: false,
        }
      })
      dispatch(setEmergencyManager(''))
      return
    }
    if (isValid) {
      setFormControl({
        ...formControl,
        emergencyManager: {
          value: input,
          isValid: true,
        }
      })
      dispatch(setEmergencyManager(input))
    }
    return;
  };
  const handleFeeReceiverChange = (input: string) => {
    const isValid = isValidAddress(input)
    while (!isValid) {
      setFormControl({
        ...formControl,
        feeReceiver: {
          value: input,
          isValid: false,
        }
      })
      dispatch(setFeeReceiver(''))
      return
    }
    setFormControl({
      ...formControl,
      feeReceiver: {
        value: input,
        isValid: true,
      }
    })
    dispatch(setFeeReceiver(input))
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
        w={'100%'}
        templateColumns={'repeat(4, 1fr)'}
        templateRows={'repeat(3, 1fr)'}
        alignSelf={'end'}
        gap={6}
      >
        <GridItem colSpan={4} colStart={1} rowStart={1}>
          <FormControl
            isInvalid={formControl.manager.isValid === false}
            isRequired
          >
            <FormLabel>Manager</FormLabel>
            <InputGroup>
              <Input
                onChange={(event) => handleManagerChange(event?.target.value)}
                value={formControl.manager.value}
                placeholder='GAFS3TLVM...'
                sx={{ pr: 8 }}
              />
              <Tooltip label='Use connected address.'>
                <InputRightElement>
                  <IconButton
                    aria-label='Connected address'
                    icon={<LinkIcon />}
                    bg={'whiteAlpha.500'}
                    size={'sm'}
                    backdropFilter={'blur(1px)'}
                    onClick={() => handleManagerChange(address!)}
                  />
                </InputRightElement>
              </Tooltip>
            </InputGroup>
            <FormErrorMessage>A valid Stellar / Soroban address is required.</FormErrorMessage>
          </FormControl>
        </GridItem>

        <GridItem colSpan={4} colStart={1} rowStart={2}>
          <FormControl
            isInvalid={formControl.emergencyManager.isValid === false}
            isRequired
          >
            <FormLabel>Emergency manager</FormLabel>
            <InputGroup>
              <Input
                onChange={(event) => handleEmergencyManagerChange(event?.target.value)}
                value={formControl.emergencyManager.value}
                placeholder='GAFS3TLVM...'
                sx={{ pr: 8 }}
              />
              <Tooltip label='Use connected address.'>
                <InputRightElement>
                  <IconButton
                    aria-label='Connected address'
                    icon={<LinkIcon />}
                    bg={'whiteAlpha.500'}
                    size={'sm'}
                    backdropFilter={'blur(1px)'}
                    onClick={() => handleEmergencyManagerChange(address!)}
                  />
                </InputRightElement>
              </Tooltip>
            </InputGroup>
            <FormErrorMessage>A valid Stellar / Soroban address is required.</FormErrorMessage>
          </FormControl>
        </GridItem>

        <GridItem colSpan={4} colStart={1} rowStart={3}>
          <FormControl
            isInvalid={formControl.feeReceiver.isValid === false}
            isRequired
          >
            <FormLabel>Fee reciever</FormLabel>
            <InputGroup>
              <Input
                onChange={(event) => handleFeeReceiverChange(event?.target.value)}
                value={formControl.feeReceiver.value}
                placeholder='GAFS3TLVM...'
                sx={{ pr: 8 }}
              />
              <Tooltip label='Use connected address.'>
                <InputRightElement>
                  <IconButton
                    aria-label='Connected address'
                    icon={<LinkIcon />}
                    bg={'whiteAlpha.500'}
                    size={'sm'}
                    backdropFilter={'blur(1px)'}
                    onClick={() => handleFeeReceiverChange(address!)}
                  />
                </InputRightElement>
              </Tooltip>
            </InputGroup>
            <FormErrorMessage>A valid Stellar / Soroban address is required.</FormErrorMessage>
          </FormControl>
        </GridItem>



      </Grid>
    </>
  )
}
