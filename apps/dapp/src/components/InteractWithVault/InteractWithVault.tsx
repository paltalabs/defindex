import React, { useContext, useState } from 'react'
import { Address, nativeToScVal, xdr } from '@stellar/stellar-sdk'
import { useSorobanReact } from '@soroban-react/core'

import { useAppDispatch, useAppSelector } from '@/store/lib/storeHooks'
import { setVaultTVL } from '@/store/lib/features/walletStore'
import { Strategy } from '@/store/lib/types'

import { VaultMethod, useVaultCallback, useVault } from '@/hooks/useVault'
import { ModalContext } from '@/contexts'

import { DialogBody, DialogContent, DialogHeader } from '../ui/dialog'
import { NativeSelectRoot } from '../ui/native-select'
import { InputGroup } from '../ui/input-group'
import {
  Button,
  Input,
  Textarea,
  Text,
  Grid,
  GridItem,
  Stack,
  NativeSelectField,
  HStack,
} from '@chakra-ui/react'

export const InteractWithVault = () => {
  const [amount, set_amount] = useState<number>(0)
  const selectedVault = useAppSelector(state => state.wallet.vaults.selectedVault)
  const vaultMethod = selectedVault?.method

  const { address } = useSorobanReact();
  const vaultCB = useVaultCallback()
  const vault = useVault()
  const dispatch = useAppDispatch()
  const { transactionStatusModal: statusModal, interactWithVaultModal: interactModal, inspectVaultModal: inspectModal } = useContext(ModalContext)

  const vaultOperation = async () => {
    if (!address || !vaultMethod) return;
    if (!amount) throw new Error('Amount is required');
    const parsedAmount = parseFloat(amount.toString())
    const convertedAmount = parsedAmount * Math.pow(10, 7)
    statusModal.initModal()
    let params: xdr.ScVal[] = []
    if (vaultMethod === VaultMethod.DEPOSIT) {
      const depositParams: xdr.ScVal[] = [
        xdr.ScVal.scvVec([nativeToScVal(parseFloat(convertedAmount.toString()), { type: "i128" })]),
        xdr.ScVal.scvVec([nativeToScVal((convertedAmount * 0.9), { type: "i128" })]),
        new Address(address).toScVal(),
      ]
      params = depositParams
    };
    if (vaultMethod === VaultMethod.WITHDRAW) {
      const withdrawAmount = ((amount * selectedVault.totalSupply) / selectedVault.TVL)
      const convertedWithdrawAmount = withdrawAmount * Math.pow(10, 7)
      const withdrawParams: xdr.ScVal[] = [
        nativeToScVal(convertedWithdrawAmount, { type: "i128" }),
        new Address(address).toScVal(),
      ]
      params = withdrawParams
    };
    console.log('Vault method:', vaultMethod)
    try {
      const result = await vaultCB(
        vaultMethod!,
        selectedVault?.address!,
        params,
        true,
      ).then((res) =>
        statusModal.handleSuccess(res.txHash)
      ).finally(async () => {
        const newTVL = await vault.getTVL(selectedVault?.address!)
        const parsedNewTVL = Number(newTVL) / 10 ** 7
        dispatch(setVaultTVL(parsedNewTVL))
      });
    }
    catch (error: any) {
      console.error('Error:', error)
      statusModal.handleError(error.toString())
    }
  }

  const setAmount = (input: any) => {
    if (input < 0 || !selectedVault) return;
    if (vaultMethod === VaultMethod.WITHDRAW) {
      console.log(input, selectedVault?.userBalance)
      if (input > selectedVault.userBalance!) return;
    }
    const decimalRegex = /^(\d+)?(\.\d{0,7})?$/;
    if (!decimalRegex.test(input)) return;
    if (input.startsWith('.')) {
      set_amount(0 + input)
      return
    }
    set_amount(input)
  }
  if (!selectedVault) return null

  return (
    <>
      <DialogContent zIndex={'docked'}>
        <DialogHeader>
          <Text fontSize='xl'>{selectedVault.method === 'deposit' ? 'Deposit to' : 'Withdraw from'} {selectedVault.name}</Text>
        </DialogHeader>
        <DialogBody zIndex={'docked'}>
          <Grid templateColumns="repeat(11, 1fr)" gap={1}>
            <GridItem colSpan={12}>
              <h2>Vault address:</h2>
              <Textarea
                defaultValue={selectedVault.address}
                rows={1}
                w={'full'}
                textAlign={'center'}
                readOnly
                resize={'none'} />
            </GridItem>
            <GridItem colSpan={5} colStart={1} textAlign={'start'}>
              <h2>Total value locked: ${selectedVault?.TVL} {selectedVault.assets[0]?.symbol}</h2>
            </GridItem>
            <GridItem colSpan={6} colStart={6} textAlign={'end'}>
              <h2>User balance in vault: ${selectedVault?.userBalance} {selectedVault.assets[0]?.symbol}</h2>
            </GridItem>
            {vaultMethod != VaultMethod.EMERGENCY_WITHDRAW &&
              <GridItem colSpan={12} pt={6}>
                <HStack justify={'end'}>
                  <Text fontSize='sm'>Amount to {vaultMethod}:</Text>
                  <Stack alignContent={'center'} alignItems={'end'}>
                  <InputGroup
                      endElement={selectedVault?.assets[0]?.symbol}
                  >
                      <Input my={4} type="text" onChange={(e) => setAmount(e.target.value)} placeholder='Amount' value={amount} />
                  </InputGroup>
                  </Stack>
                </HStack>

              </GridItem>
            }
            {
              vaultMethod === VaultMethod.EMERGENCY_WITHDRAW &&
              <>
                <GridItem colSpan={6} textAlign={'end'} alignContent={'center'}>
                  <Text fontSize='lg'>Emergency withdraw from {selectedVault?.name}:</Text>
                </GridItem>
                <GridItem colSpan={6} colEnd={13} textAlign={'end'} >
                  <NativeSelectRoot>
                    <NativeSelectField>
                      {selectedVault?.assets[0]?.strategies.map((strategy: Strategy) => {
                        return (
                          <option key={strategy.address} value={strategy.address}>{strategy.name}</option>
                        )
                      })}
                    </NativeSelectField>
                  </NativeSelectRoot>
                </GridItem>
              </>
            }
          </Grid>
          <Button
            disabled={vaultMethod != VaultMethod.EMERGENCY_WITHDRAW && amount < 0.0000001}
            my={4}
            w={'full'}
            colorScheme='green'
            onClick={() => vaultOperation()}>
            {selectedVault?.method === VaultMethod.DEPOSIT && 'Deposit' ||
              selectedVault?.method === VaultMethod.WITHDRAW && 'Withdraw' ||
              selectedVault?.method === VaultMethod.EMERGENCY_WITHDRAW && 'Emergency Withdraw'}
          </Button>
        </DialogBody>
      </DialogContent>
    </>
  )
}
