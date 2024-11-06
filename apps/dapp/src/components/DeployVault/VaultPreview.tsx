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
} from '@chakra-ui/react'
import { shortenAddress } from '@/helpers/shortenAddress'

import { ChartData } from './ConfirmDelpoyModal'
import { setEmergencyManager, setFeeReceiver, setManager, setVaultShare } from '@/store/lib/features/vaultStore'
import { useAppDispatch, useAppSelector } from '@/store/lib/storeHooks'
import { StrKey } from '@stellar/stellar-sdk'
import { FaRegPaste } from "react-icons/fa6";
import { useSorobanReact } from '@soroban-react/core'
import { InputGroup } from '../ui/input-group'
import { Tooltip } from '../ui/tooltip'
import {
  AccordionItem,
  AccordionItemContent,
  AccordionItemTrigger,
  AccordionRoot,
} from "@chakra-ui/react"

enum AccordionItems {
  STRATEGY_DETAILS = 'strategy-details',
  MANAGER_CONFIGS = 'manager-configs',
  FEES_CONFIGS = 'fees-configs',
}
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
  vaultShare: number
}
export const VaultPreview = ({ data }: { data: ChartData[] }) => {

  const dispatch = useAppDispatch()
  const { address } = useSorobanReact()
  const vaultShare = useAppSelector(state => state.newVault.vaultShare)
  const [accordionValue, setAccordionValue] = useState<AccordionItems[]>([AccordionItems.STRATEGY_DETAILS])
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
    vaultShare: 0
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
  const handleVaultShareChange = (input: any) => {
    console.log(input)
    if (isNaN(input)) return
    if (input < 0 || input > 100) return
    const decimalRegex = /^(\d+)?(\.\d{0,2})?$/
    if (!decimalRegex.test(input)) return
    console.log(input * 100)
    setFormControl({
      ...formControl,
      vaultShare: input
    })
    dispatch(setVaultShare(input * 100))
  }

  return (
    <>
      <Box display='flex' my={4}>
        {/* <PieChart
          series={[
            {
              data: data,
            },
          ]}
          width={500}
          height={200}
        /> */}
      </Box>
      <AccordionRoot value={accordionValue} onValueChange={(e: any) => setAccordionValue(e.value)}>
        <AccordionItem value={AccordionItems.STRATEGY_DETAILS}>
          <AccordionItemTrigger>
            <Text fontSize='lg' fontWeight='bold' mb={2}>
              Strategies
            </Text>
          </AccordionItemTrigger>
          <AccordionItemContent>
            <Table.Root>
              <Table.Header>
                <Table.Row>
                  <Table.Cell>Name</Table.Cell>
                  <Table.Cell>Address</Table.Cell>
                  <Table.Cell >Percentage</Table.Cell>
                </Table.Row>
              </Table.Header>
              <Table.Body>
                {data.map((strategy: ChartData, index: number) => (
                  <Table.Row key={index}>
                    <Table.Cell>{strategy.label}</Table.Cell>
                    <Tooltip content={strategy.address}>
                      <Table.Cell w={1}>
                        {strategy.address ? shortenAddress(strategy.address) : '-'}
                      </Table.Cell>
                    </Tooltip>
                    <Table.Cell>{strategy.value}%</Table.Cell>
                  </Table.Row>
                ))}
              </Table.Body>
            </Table.Root>
          </AccordionItemContent>
        </AccordionItem>
        <AccordionItem value={AccordionItems.MANAGER_CONFIGS}>
          <AccordionItemTrigger>
            <Text fontSize='lg' fontWeight='bold' mb={2}>
              Manager settings
            </Text>
          </AccordionItemTrigger>
          <AccordionItemContent>
          <Fieldset.Root
            invalid={formControl.manager.isValid === false}
          >
            <Fieldset.Legend>Manager</Fieldset.Legend>
              <Stack mb={6}>
              <InputGroup endElement={
                  <Tooltip content='Use connected wallet'>
                  <IconButton
                    aria-label='Connected address'
                    size={'sm'}
                    onClick={() => handleManagerChange(address!)}
                    >
                    <FaRegPaste />
                    </IconButton>
                </Tooltip>
              }>
                <Input
                  onChange={(event) => handleManagerChange(event?.target.value)}
                  value={formControl.manager.value}
                  placeholder='GAFS3TLVM...'
                />
              </InputGroup>
            </Stack>
            <Fieldset.ErrorText>A valid Stellar / Soroban address is required.</Fieldset.ErrorText>
            </Fieldset.Root>
          <Fieldset.Root
            invalid={formControl.emergencyManager.isValid === false}
          >
            <Fieldset.Legend>Emergency manager</Fieldset.Legend>
              <Stack mb={6}>
              <InputGroup endElement={
                <Tooltip content='Use connected wallet'>
                  <IconButton
                    aria-label='Connected address'
                    size={'sm'}
                    onClick={() => handleEmergencyManagerChange(address!)}
                  >
                    <FaRegPaste />
                  </IconButton>
                </Tooltip>
              }>
                <Input
                  onChange={(event) => handleEmergencyManagerChange(event?.target.value)}
                  value={formControl.emergencyManager.value}
                  placeholder='GAFS3TLVM...'
                />
              </InputGroup>
            </Stack>
            <Fieldset.ErrorText>A valid Stellar / Soroban address is required.</Fieldset.ErrorText>
          </Fieldset.Root>
          </AccordionItemContent>
        </AccordionItem>
        <AccordionItem value={AccordionItems.FEES_CONFIGS}>
          <AccordionItemTrigger>
            <Text fontSize='lg' fontWeight='bold' mb={2}>
              Fees settings
            </Text>
          </AccordionItemTrigger>
          <AccordionItemContent>
          <Fieldset.Root
            invalid={formControl.feeReceiver.isValid === false}
          >
            <Fieldset.Legend>Fee reciever</Fieldset.Legend>
            <Stack>
              <InputGroup endElement={
                <Tooltip content='Use connected wallet'>
                  <IconButton
                    aria-label='Connected address'
                    size={'sm'}
                    onClick={() => handleFeeReceiverChange(address!)}
                  >
                    <FaRegPaste />
                  </IconButton>
                </Tooltip>
              }>
                <Input
                  onChange={(event) => handleFeeReceiverChange(event?.target.value)}
                  value={formControl.feeReceiver.value}
                  placeholder='GAFS3TLVM...'
                />
              </InputGroup>
            </Stack>
            <Fieldset.ErrorText>A valid Stellar / Soroban address is required.</Fieldset.ErrorText>
          </Fieldset.Root>
            <InputGroup
              endElement={'%'}
            >
              <Input
                value={formControl.vaultShare}
                onChange={(e) => { handleVaultShareChange(e.target.value) }}
              />
            </InputGroup>
          </AccordionItemContent>
        </AccordionItem>
      </AccordionRoot >
    </>
  )
}
