import { isValidAddress, shortenAddress } from '@/helpers/address'
import { setEmergencyManager, setFeeReceiver, setManager, setRebalanceManager, setUpgradable, setVaultShare } from '@/store/lib/features/vaultStore'
import { useAppDispatch, useAppSelector } from '@/store/lib/storeHooks'
import { Asset } from '@/store/lib/types'
import {
  AccordionItem,
  AccordionItemContent,
  AccordionItemTrigger,
  AccordionRoot,
  Checkbox,
  Fieldset,
  Grid,
  GridItem,
  Icon,
  IconButton,
  Input,
  Link,
  Stack,
  Table,
  Text,
} from '@chakra-ui/react'
import { useSorobanReact } from '@soroban-react/core'
import React from 'react'
import { FaRegPaste } from "react-icons/fa6"
import { IoIosArrowDown, IoIosArrowUp } from 'react-icons/io'
import { InputGroup } from '../ui/input-group'
import { NumberInputField, NumberInputRoot } from '../ui/number-input'
import { InfoTip } from '../ui/toggle-tip'
import { Tooltip } from '../ui/tooltip'


export enum AccordionItems {
  STRATEGY_DETAILS = 'strategy-details',
  MANAGER_CONFIGS = 'manager-configs',
  FEES_CONFIGS = 'fees-configs',
  UPGRADABLE_CONTRACT = 'upgradable-contract'
}
export interface FormControlInterface {
  manager: {
    isValid: boolean | undefined;
    value: string | undefined;
  },
  emergencyManager: {
    isValid: boolean | undefined;
    value: string | undefined;
  },
  rebalanceManager: {
    isValid: boolean | undefined;
    value: string | undefined;
  },
  feeReceiver: {
    isValid: boolean | undefined;
    value: string | undefined;
  },
  vaultShare: number,
  upgradable: boolean
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
            {title === 'Strategies' ? 'Strategies' : title + ' settings'}
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
  invalid,
  description,
  href,
}: {
  label: string,
  value: string,
  onChange: (e: any) => void,
  handleClick: (address: string) => void,
  placeholder: string,
  invalid: boolean,
  description?: string,
  href?: string,
}) => {
  const { address } = useSorobanReact()
  return (
    <Fieldset.Root invalid={invalid}>
      <Fieldset.Legend>{label}
        <InfoTip content={
          <>
            <Text fontSize={'xs'} maxW={'25vw'}>{description}</Text>
            <Link
              href={href ?? ''}
              colorPalette={'blue'}
              target='_blank'
            >
              Learn more.
            </Link>

          </>
        } />
      </Fieldset.Legend>
      <Stack>
        <InputGroup endElement={
          <Tooltip content='Use connected wallet'>
            <IconButton
              aria-label='Connected address'
              size={'sm'}
              variant={'ghost'}
              onClick={() => handleClick(address!)}
            >
              <FaRegPaste />
            </IconButton>
          </Tooltip>
        }
        >
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

interface VaultPreviewProps {

  data: Asset[];

  accordionValue: AccordionItems[];

  setAccordionValue: React.Dispatch<React.SetStateAction<AccordionItems[]>>;

  formControl: FormControlInterface;

  setFormControl: (args: FormControlInterface) => any;

}

export const dropdownData = {
  strategies: {
    title: 'Strategies',
    description: 'A strategy is a set of steps to be followed to execute an investment in one or several protocols.',
    href: 'https://docs.defindex.io/whitepaper/10-whitepaper/01-introduction#core-concepts'
  },
  manager: {
    title: 'Manager',
    description: 'The Manager can rebalance the Vault, emergency withdraw and invest IDLE funds in strategies.',
    href: 'https://docs.defindex.io/whitepaper/10-whitepaper/03-the-defindex-approach/02-contracts/01-vault-contract#management'
  },
  emergencyManager: {
    title: 'Emergency manager',
    description: 'The Emergency Manager has the authority to withdraw assets from the DeFindex in case of an emergency.',
    href: 'https://docs.defindex.io/whitepaper/10-whitepaper/03-the-defindex-approach/02-contracts/01-vault-contract#emergency-management'
  },
  rebalanceManager: {
    title: 'Rebalance manager',
    description: 'The Rebalance Manager can rebalance the Vault',
    href: 'https://docs.defindex.io/whitepaper/10-whitepaper/03-the-defindex-approach/02-contracts/01-vault-contract#rebalance-management'
  },
  feeReceiver: {
    title: 'Fee receiver',
    description: 'Fee Receiver could be the manager using the same address, or it could be a different entity such as a streaming contract, a DAO, or another party.',
    href: 'https://docs.defindex.io/whitepaper/10-whitepaper/03-the-defindex-approach/02-contracts/01-vault-contract#fee-collection'
  },
  upgradableContract: {
    title: 'Upgradable',
    description: 'The contract can be upgraded by the manager.',
    href: 'https://docs.defindex.io/whitepaper/10-whitepaper/03-the-defindex-approach/02-contracts/01-vault-contract#upgradability'
  }
}
export const VaultPreview: React.FC<VaultPreviewProps> = ({ data, accordionValue, setAccordionValue, formControl, setFormControl }) => {

  const dispatch = useAppDispatch()
  const newVault = useAppSelector(state => state.newVault)
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

  const handleRebalanceManagerChange = (input: string) => {
    const isValid = isValidAddress(input)
    while (!isValid) {
      setFormControl({
        ...formControl,
        rebalanceManager: {
          value: input,
          isValid: false,
        }
      })
      dispatch(setRebalanceManager(''))
      return
    }
    if (isValid) {
      setFormControl({
        ...formControl,
        rebalanceManager: {
          value: input,
          isValid: true,
        }
      })
      dispatch(setRebalanceManager(input))
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
    if (input < 0 || input > 100) return
    const decimalRegex = /^(\d+)?(\.\d{0,2})?$/
    if (!decimalRegex.test(input)) return
    setFormControl({
      ...formControl,
      vaultShare: input
    })
    dispatch(setVaultShare(input * 100))
  }

  const handleUpgradableChange = (input: boolean) => {
    setFormControl({
      ...formControl,
      upgradable: input
    })
    dispatch(setUpgradable(input))
  }


  return (
    <>
      <AccordionRoot value={accordionValue} onValueChange={(e: any) => setAccordionValue(e.value)}>
        <AccordionItem value={AccordionItems.STRATEGY_DETAILS}>
          <CustomAccordionTrigger
            setAccordionValue={setAccordionValue}
            title={dropdownData.strategies.title}
            type={AccordionItems.STRATEGY_DETAILS}
            accordionValue={accordionValue}
          />
          <AccordionItemContent>
            <Table.Root size={'lg'} w={'full'}>
              <Table.Header>
                <Table.Row >
                  <Table.Cell>Name</Table.Cell>
                  <Table.Cell textAlign={'center'}>Address</Table.Cell>
                  <Table.Cell textAlign={'center'} >Asset</Table.Cell>
                  {newVault.assets.some((asset) => asset.amount) && (
                    <Table.Cell textAlign={'center'}>Initial deposit</Table.Cell>
                  )}
                </Table.Row>
              </Table.Header>
              <Table.Body>
                {data.map((asset: Asset, index: number) => (
                  asset.strategies.map((strategy, j) => (
                    <Table.Row key={strategy.address}>
                      <Table.Cell>{strategy.name}</Table.Cell>
                      <Table.Cell textAlign={'center'}>
                        {strategy.address ? shortenAddress(strategy.address) : '-'}
                      </Table.Cell>
                      <Table.Cell textAlign={'center'}>{asset.symbol}</Table.Cell>
                      <Table.Cell textAlign={'center'}>${strategy.tempAmount} {asset.symbol}</Table.Cell>
                    </Table.Row>

                  ))
                ))}
              </Table.Body>
            </Table.Root>
          </AccordionItemContent>
        </AccordionItem>
        <AccordionItem value={AccordionItems.MANAGER_CONFIGS}>
          <CustomAccordionTrigger
            setAccordionValue={setAccordionValue}
            title={dropdownData.manager.title}
            type={AccordionItems.MANAGER_CONFIGS}
            accordionValue={accordionValue}
          />
          <AccordionItemContent>
            <CustomInputField
              label={dropdownData.manager.title}
              value={formControl.manager.value || ''}
              onChange={(e) => handleManagerChange(e.target.value)}
              handleClick={(address: string) => handleManagerChange(address)}
              placeholder='GAFS3TLVM...'
              invalid={formControl.manager.isValid === false}
              description={dropdownData.manager.description}
              href={dropdownData.manager.href}
            />
            <CustomInputField
              label={dropdownData.emergencyManager.title}
              value={formControl.emergencyManager.value || ''}
              onChange={(e) => handleEmergencyManagerChange(e.target.value)}
              handleClick={(address: string) => handleEmergencyManagerChange(address)}
              placeholder='GAFS3TLVM...'
              invalid={formControl.emergencyManager.isValid === false}
              description={dropdownData.emergencyManager.description}
              href={dropdownData.emergencyManager.href}
            />
            <CustomInputField
              label={dropdownData.rebalanceManager.title}
              value={formControl.rebalanceManager.value || ''}
              onChange={(e) => handleRebalanceManagerChange(e.target.value)}
              handleClick={(address: string) => handleRebalanceManagerChange(address)}
              placeholder='GAFS3TLVM...'
              invalid={formControl.rebalanceManager.isValid === false}
              description={dropdownData.rebalanceManager.description}
              href={dropdownData.rebalanceManager.href}
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
              description={dropdownData.feeReceiver.description}
              href={dropdownData.feeReceiver.href}
            />
            <Fieldset.Root invalid={formControl.vaultShare == 0} mt={4}>
              <Fieldset.Legend>Fee percentage <InfoTip content={
                <>
                  <Text fontSize={'xs'} maxW={'25vw'}>The recommended initial setup suggests a fee of 0.5% - 2% APR on these shares.</Text>
                  <Link
                    href={dropdownData.feeReceiver.href}
                    colorPalette={'blue'}
                    target='_blank'
                  >
                    Learn more.
                  </Link>

                </>
              } /></Fieldset.Legend>
              <Stack w={100}>
                <InputGroup endElement={'%'}>
                  <NumberInputRoot
                    onValueChange={(e) => { handleVaultShareChange(Number(e.value)) }}
                    required
                  >
                    <NumberInputField />
                  </NumberInputRoot>
                </InputGroup>
              </Stack>
              <Fieldset.ErrorText>This field is required.</Fieldset.ErrorText>
            </Fieldset.Root>
          </AccordionItemContent>
        </AccordionItem>
        <AccordionItem value={AccordionItems.UPGRADABLE_CONTRACT}>
          <CustomAccordionTrigger
            setAccordionValue={setAccordionValue}
            title='Upgrade'
            type={AccordionItems.UPGRADABLE_CONTRACT}
            accordionValue={accordionValue} />
          <AccordionItemContent
            px={4}
            mb={4}
          >
            <Fieldset.Root>
              <Checkbox.Root
                alignItems="center"
                checked={formControl.upgradable}
                gap="4"
                onCheckedChange={(e) => handleUpgradableChange(!!e.checked)}
              >
                <Checkbox.HiddenInput />
                <Checkbox.Label>
                  Upgradable <InfoTip content={
                    <>
                      <Text fontSize={'xs'} maxW={'25vw'}>{dropdownData.upgradableContract.description}</Text>
                      <Link
                        href={dropdownData.upgradableContract.href}
                        colorPalette={'blue'}
                        target='_blank'
                      >
                        Learn more.
                      </Link>
                    </>
                  } />
                </Checkbox.Label>
                <Checkbox.Control>
                  <Checkbox.Indicator />
                </Checkbox.Control>

              </Checkbox.Root>
            </Fieldset.Root>
          </AccordionItemContent>
        </AccordionItem>
      </AccordionRoot >
    </>
  )
}