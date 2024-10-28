import React, { useState } from 'react'
import {
  Box,
  Table,
  Text,
  Grid,
  GridItem,
  Input,
  IconButton,
  Fieldset,
  Stack,
  Tooltip,
  InputAddon,
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
      <Table.Root>
        <Table.Header>
          <Table.Row>
            <Table.Header>Name</Table.Header>
            <Table.Header textAlign={'center'}>Address</Table.Header>
            <Table.Header textAlign={'end'}>Percentage</Table.Header>
          </Table.Row>
        </Table.Header>
        <Table.Body>
            {data.map((strategy: ChartData, index: number) => (
              <Table.Row key={index}>
                <Table.Cell>{strategy.label}</Table.Cell>
                <Table.Cell textAlign={'center'}>
                  {/* <Tooltip content={strategy.address}>
                    {strategy.address ? shortenAddress(strategy.address) : '-'}
                    </Tooltip> */}
                  {strategy.address ? shortenAddress(strategy.address) : '-'}
                </Table.Cell>
                <Table.Cell textAlign={'end'}>{strategy.value}%</Table.Cell>
              </Table.Row>
            ))}
        </Table.Body>
      </Table.Root>
      <Box height="20px" />
      <Grid
        w={'100%'}
        templateColumns={'repeat(4, 1fr)'}
        templateRows={'repeat(3, 1fr)'}
        alignSelf={'end'}
        gap={6}
      >
        <GridItem colSpan={4} colStart={1} rowStart={1}>
          <Fieldset.Root
            invalid={formControl.manager.isValid === false}
          >
            <Fieldset.Legend>Manager</Fieldset.Legend>
            <Stack>
              <Input
                onChange={(event) => handleManagerChange(event?.target.value)}
                value={formControl.manager.value}
                placeholder='GAFS3TLVM...'
              />
              {/* <Tooltip content='Use connected address.'>
                </Tooltip> */}
              <InputAddon>
                  <IconButton
                  aria-label='Connected address'
                    bg={'whiteAlpha.500'}
                    size={'sm'}
                    backdropFilter={'blur(1px)'}
                    onClick={() => handleManagerChange(address!)}
                >
                  <LinkIcon />
                </IconButton>
              </InputAddon>
            </Stack>
            <Fieldset.ErrorText>A valid Stellar / Soroban address is required.</Fieldset.ErrorText>
          </Fieldset.Root>
        </GridItem>

        <GridItem colSpan={4} colStart={1} rowStart={2}>
          <Fieldset.Root
            invalid={formControl.emergencyManager.isValid === false}
          >
            <Fieldset.Legend>Emergency manager</Fieldset.Legend>
            <Stack>
              <Input
                onChange={(event) => handleEmergencyManagerChange(event?.target.value)}
                value={formControl.emergencyManager.value}
                placeholder='GAFS3TLVM...'
              />
              {/*      <Tooltip label='Use connected address.'>
                </Tooltip> */}
              <InputAddon>
                  <IconButton
                  aria-label='Connected address'
                    bg={'whiteAlpha.500'}
                    size={'sm'}
                    backdropFilter={'blur(1px)'}
                    onClick={() => handleEmergencyManagerChange(address!)}
                >
                  <LinkIcon />
                </IconButton>
              </InputAddon>
            </Stack>
            <Fieldset.ErrorText>A valid Stellar / Soroban address is required.</Fieldset.ErrorText>
          </Fieldset.Root>
        </GridItem>

        <GridItem colSpan={4} colStart={1} rowStart={3}>
          <Fieldset.Root
            invalid={formControl.feeReceiver.isValid === false}
          >
            <Fieldset.Legend>Fee reciever</Fieldset.Legend>
            <Stack>
              <Input
                onChange={(event) => handleFeeReceiverChange(event?.target.value)}
                value={formControl.feeReceiver.value}
                placeholder='GAFS3TLVM...'
              />
              {/* <Tooltip label='Use connected address.'>
                </Tooltip> */}
              <InputAddon>
                  <IconButton
                  aria-label='Connected address'
                    bg={'whiteAlpha.500'}
                    size={'sm'}
                    backdropFilter={'blur(1px)'}
                    onClick={() => handleFeeReceiverChange(address!)}
                >
                  <LinkIcon />
                </IconButton>
              </InputAddon>
            </Stack>
            <Fieldset.ErrorText>A valid Stellar / Soroban address is required.</Fieldset.ErrorText>
          </Fieldset.Root>
        </GridItem>



      </Grid>
    </>
  )
}
