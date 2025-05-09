import { useSorobanReact } from 'stellar-react'
import { Address, xdr } from '@stellar/stellar-sdk'
import { useContext, useEffect, useState } from 'react'

import { setFeeReceiver } from '@/store/lib/features/vaultStore'
import { setVaultFeeReceiver } from '@/store/lib/features/walletStore'
import { useAppDispatch, useAppSelector } from '@/store/lib/storeHooks'

import { ModalContext } from '@/contexts'
import { isValidAddress } from '@/helpers/address'
import { VaultMethod, useVault, useVaultCallback } from '@/hooks/useVault'

import {
  Fieldset,
  HStack,
  IconButton,
  Input,
  Link,
  Stack,
  Text,
} from '@chakra-ui/react'
import { FaRegPaste } from 'react-icons/fa6'
import { LuSettings2 } from "react-icons/lu"
import { dropdownData } from '../DeployVault/VaultPreview'
import { Button } from '../ui/button'
import { DialogBody, DialogContent, DialogHeader } from '../ui/dialog'
import { InputGroup } from '../ui/input-group'
import { InfoTip } from '../ui/toggle-tip'
import { Tooltip } from '../ui/tooltip'

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
              css={{ "--bg": "{colors.red.400/40}" }}
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

export const EditVaultModal = () => {
  const selectedVault = useAppSelector(state => state.wallet.vaults.selectedVault)
  const vaultMethod = selectedVault?.method

  const { address } = useSorobanReact();
  const vaultCB = useVaultCallback()
  const vault = useVault()
  const dispatch = useAppDispatch()
  const {
    transactionStatusModal: statusModal,
    rebalanceVaultModal: rebalanceModal
  } = useContext(ModalContext)
  const [formControl, setFormControl] = useState({
    feeReceiver: {
      value: selectedVault?.feeReceiver ?? '',
      isValid: false,
      needsUpdate: false,
      isLoading: false,
    }
  })




  const handleFeeReceiverChange = (input: string) => {
    const isValid = isValidAddress(input)
    while (!isValid) {
      setFormControl({
        ...formControl,
        feeReceiver: {
          ...formControl.feeReceiver,
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
        ...formControl.feeReceiver,
        value: input,
        isValid: true,
      }
    })
  };

  useEffect(() => {
    setFormControl({
      ...formControl,
      feeReceiver: {
        isValid: isValidAddress(selectedVault?.feeReceiver ?? ''),
        value: selectedVault?.feeReceiver ?? '',
        needsUpdate: false,
        isLoading: false,
      }
    })
  }, [selectedVault])
  enum Values {
    FEERECIEVER = 'feeReceiver',
    MANAGER = 'manager',
    EMERGENCYMANAGER = 'emergencyManager'
  }

  const updateValue = async (value: Values) => {
    if (!address || !selectedVault) return;
    let result: any;
    if (value === Values.FEERECIEVER) {
      setFormControl({ feeReceiver: { ...formControl.feeReceiver, isLoading: true } })
      statusModal.initModal()
      console.log('Updating fee receiver')
      const caller = new Address(address);
      const feeReceiver = new Address(formControl.feeReceiver.value);
      const createDefindexParams: xdr.ScVal[] = [
        caller.toScVal(),
        feeReceiver.toScVal(),
      ];
      try {
        result = await vaultCB(VaultMethod.SET_FEE_RECEIVER, selectedVault.address, createDefindexParams, true).then((res) => {
          console.log(res)
          statusModal.handleSuccess(res.txHash)
          dispatch(setVaultFeeReceiver(formControl.feeReceiver.value))
        })
      } catch (error: any) {
        console.error('Error:', error)
        statusModal.handleError(error.toString())
      } finally {
        setFormControl({ feeReceiver: { ...formControl.feeReceiver, isLoading: false } })
      }

    };
  }

  useEffect(() => {
    if (!selectedVault?.feeReceiver) return
    if (formControl.feeReceiver.value !== selectedVault.feeReceiver && formControl.feeReceiver.isValid) {
      setFormControl({
        ...formControl,
        feeReceiver: {
          ...formControl.feeReceiver,
          needsUpdate: true,
        }
      })
    } else if (formControl.feeReceiver.value === selectedVault.feeReceiver && formControl.feeReceiver.isValid) {
      setFormControl({
        ...formControl,
        feeReceiver: {
          ...formControl.feeReceiver,
          needsUpdate: false,
        }
      })
    }
  }, [formControl.feeReceiver.value, formControl.feeReceiver.isValid])

  if (!selectedVault) return null
  return (
    <>
      <DialogContent zIndex={'docked'}>
        <DialogHeader>
          <HStack justifyContent={'space-between'}>
            <Text fontSize='xl'>Manage {selectedVault.name}</Text>
            <IconButton variant={'ghost'}
              onClick={() => rebalanceModal.setIsOpen(true)}
              size={'sm'}
            >
              <LuSettings2 />
            </IconButton>
          </HStack>
        </DialogHeader>
        <DialogBody zIndex={'docked'}>
          <HStack align={'baseline'}>
            <CustomInputField
              label={dropdownData.feeReceiver.title}
              value={formControl.feeReceiver.value}
              href={dropdownData.feeReceiver.href}
              onChange={(e) => handleFeeReceiverChange(e.target.value)}
              handleClick={(address) => setFormControl({ feeReceiver: { ...formControl.feeReceiver, isValid: true, value: address } })}
              placeholder='GAFS3TLVM...'
              invalid={!formControl.feeReceiver.isValid}
              description={dropdownData.feeReceiver.description}
            />
          </HStack>
          <HStack justifyContent={'end'} mt={4}>
            {formControl.feeReceiver.needsUpdate &&
              <Button
                loading={formControl.feeReceiver.isLoading}
                onClick={() => updateValue(Values.FEERECIEVER)}
                disabled={!formControl.feeReceiver.isValid}
              >
                Update fee receiver
              </Button>
            }
          </HStack>
        </DialogBody>
      </DialogContent>
    </>
  )
}
