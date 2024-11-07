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
  Icon,
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
import { IoIosArrowDown, IoIosArrowUp } from 'react-icons/io'

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
const CustomAccordionTrigger = ({ title, type, accordionValue, setAccordionValue }: { title: string, type: AccordionItems, accordionValue: AccordionItems[], setAccordionValue: React.Dispatch<React.SetStateAction<AccordionItems[]>> }) => {
  return (
    <AccordionItemTrigger onClick={() => {
      if (accordionValue[0] === type) {
        setAccordionValue([])
      }
    }}>
      <Grid templateColumns={'repeat(12, 1fr)'} width={'100%'}>
        <GridItem colSpan={6} colStart={1}>
          <Text fontSize='lg' textAlign={'left'} fontWeight='bold' mb={2}>
            {title} settings
          </Text>
        </GridItem>
        <GridItem colSpan={1} colStart={12}>
          <Text fontSize='lg' fontWeight='bold' mb={2}>
            {accordionValue[0] === type ?
              <Icon>
                <IoIosArrowUp />
              </Icon>
              :
              <Icon>
                <IoIosArrowDown />
              </Icon>
            }
          </Text>
        </GridItem>
      </Grid>
    </AccordionItemTrigger>
  )
}

const CustomInputField = ({
  label,
  value,
  onChange,
  handleClick,
  placeholder,
  invalid
}: {
  label: string,
  value: string,
  onChange: (e: any) => void,
  handleClick: (address: string) => void,
  placeholder: string,
  invalid: boolean
}) => {
  const { address } = useSorobanReact()
  if (!address) return null
  return (
    <Fieldset.Root invalid={invalid}>
      <Fieldset.Legend>{label}</Fieldset.Legend>
      <Stack>
        <InputGroup endElement={
          <Tooltip content='Use connected wallet'>
            <IconButton
              aria-label='Connected address'
              size={'sm'}
              onClick={() => handleClick(address)}
            >
              <FaRegPaste />
            </IconButton>
          </Tooltip>
        }>
          <Input
            value={value}
            onChange={onChange}
            placeholder={placeholder}
          />
        </InputGroup>
      </Stack>
      <Fieldset.ErrorText>A valid Stellar / Soroban address is required.</Fieldset.ErrorText>
    </Fieldset.Root>
  )
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
    if (isNaN(input)) return
    if (input < 0 || input > 100) return
    const decimalRegex = /^(\d+)?(\.\d{0,2})?$/
    if (!decimalRegex.test(input)) return
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
          <CustomAccordionTrigger
            setAccordionValue={setAccordionValue}
            title='Strategies'
            type={AccordionItems.STRATEGY_DETAILS}
            accordionValue={accordionValue} />
          <AccordionItemContent>
            <Table.Root size={'lg'} w={'full'}>
              <Table.Header>
                <Table.Row >
                  <Table.Cell>Name</Table.Cell>
                  <Table.Cell textAlign={'center'}>Address</Table.Cell>
                  <Table.Cell textAlign={'center'} >Percentage</Table.Cell>
                </Table.Row>
              </Table.Header>
              <Table.Body>
                {data.map((strategy: ChartData, index: number) => (
                  <Table.Row key={index}>
                    <Table.Cell>{strategy.label}</Table.Cell>
                    <Table.Cell textAlign={'center'}>
                      {strategy.address ? shortenAddress(strategy.address) : '-'}
                    </Table.Cell>
                    <Table.Cell textAlign={'center'}>{strategy.value}%</Table.Cell>
                  </Table.Row>
                ))}
              </Table.Body>
            </Table.Root>
          </AccordionItemContent>
        </AccordionItem>
        <AccordionItem value={AccordionItems.MANAGER_CONFIGS}>
          <CustomAccordionTrigger
            setAccordionValue={setAccordionValue}
            title='Manager'
            type={AccordionItems.MANAGER_CONFIGS}
            accordionValue={accordionValue}
          />
          <AccordionItemContent>
            <CustomInputField
              label='Manager'
              value={formControl.manager.value || ''}
              onChange={(e) => handleManagerChange(e.target.value)}
              handleClick={(address: string) => handleManagerChange(address)}
              placeholder='GAFS3TLVM...'
              invalid={formControl.manager.isValid === false}
            />
            <CustomInputField
              label='Emergency manager'
              value={formControl.emergencyManager.value || ''}
              onChange={(e) => handleEmergencyManagerChange(e.target.value)}
              handleClick={(address: string) => handleEmergencyManagerChange(address)}
              placeholder='GAFS3TLVM...'
              invalid={formControl.emergencyManager.isValid === false}
            />
          </AccordionItemContent>
        </AccordionItem>
        <AccordionItem value={AccordionItems.FEES_CONFIGS}>
          <CustomAccordionTrigger
            setAccordionValue={setAccordionValue}
            title='Fees'
            type={AccordionItems.FEES_CONFIGS}
            accordionValue={accordionValue} />
          <AccordionItemContent>
            <CustomInputField
              label='Fee receiver'
              value={formControl.feeReceiver.value || ''}
              onChange={(e) => handleFeeReceiverChange(e.target.value)}
              handleClick={(address: string) => handleFeeReceiverChange(address)}
              placeholder='GAFS3TLVM...'
              invalid={formControl.feeReceiver.isValid === false}
            />
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