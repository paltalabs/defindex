import { useSorobanReact } from '@soroban-react/core'
import { Address, nativeToScVal, xdr } from '@stellar/stellar-sdk'
import { useContext, useState } from 'react'

import { updateVaultData } from '@/store/lib/features/walletStore'
import { useAppDispatch, useAppSelector } from '@/store/lib/storeHooks'
import { Strategy, VaultData } from '@/store/lib/types'

import { ModalContext } from '@/contexts'
import { VaultMethod, useVault, useVaultCallback } from '@/hooks/useVault'

import {
  Button,
  Grid,
  GridItem,
  HStack,
  Input,
  NativeSelectField,
  Stack,
  Text
} from '@chakra-ui/react'
import { ClipboardIconButton, ClipboardRoot } from '../ui/clipboard'
import { DialogBody, DialogContent, DialogHeader } from '../ui/dialog'
import { InputGroup } from '../ui/input-group'
import { NativeSelectRoot } from '../ui/native-select'

export const InteractWithVault = () => {
  const [amount, set_amount] = useState<number>(0)
  const selectedVault = useAppSelector(state => state.wallet.vaults.selectedVault)
  const vaultMethod = selectedVault?.method

  const { address } = useSorobanReact();
  const [selectedStrategy, setSelectedStrategy] = useState<string | undefined>(undefined)
  const vaultCB = useVaultCallback()
  const vault = useVault()
  const dispatch = useAppDispatch()
  const { transactionStatusModal: statusModal, interactWithVaultModal: interactModal, inspectVaultModal: inspectModal } = useContext(ModalContext)

  const vaultOperation = async () => {
    if (!address || !vaultMethod || !selectedVault.address) return;
    if (!amount && vaultMethod != VaultMethod.RESCUE) throw new Error('Amount is required');
    const parsedAmount = parseFloat(amount.toString())
    const convertedAmount = parsedAmount * Math.pow(10, 7)
    statusModal.initModal()
    let params: xdr.ScVal[] = []
    if (vaultMethod === VaultMethod.DEPOSIT) {
      const depositParams: xdr.ScVal[] = [
        xdr.ScVal.scvVec([nativeToScVal(parseFloat(convertedAmount.toString()), { type: "i128" })]),
        xdr.ScVal.scvVec([nativeToScVal((convertedAmount * 0.9), { type: "i128" })]),
        new Address(address).toScVal(),
        xdr.ScVal.scvBool(false)
      ]
      params = depositParams
    };
    if (vaultMethod === VaultMethod.WITHDRAW) {
      const withdrawAmount = ((amount * selectedVault.totalSupply) / selectedVault.TVL)
      const truncatedWithdrawAmount = Math.floor(withdrawAmount * 1e7) / 1e7;
      const convertedWithdrawAmount = Number(truncatedWithdrawAmount) * Math.pow(10, 7)
      const withdrawParams: xdr.ScVal[] = [
        nativeToScVal(Math.ceil(convertedWithdrawAmount), { type: "i128" }),
        new Address(address).toScVal(),
      ]
      params = withdrawParams
    };
    if (vaultMethod === VaultMethod.RESCUE) {
      if (!selectedStrategy) throw new Error('Strategy is required')
      console.log(selectedStrategy)
      const emergencyWithdrawParams: xdr.ScVal[] = [
        new Address(selectedStrategy!).toScVal(),
        new Address(address).toScVal(),
      ]
      params = emergencyWithdrawParams
    }
    try {
      const result = await vaultCB(
        vaultMethod!,
        selectedVault?.address!,
        params,
        true,
      ).then(async (res) => {
        await statusModal.handleSuccess(res.txHash)
      }
      ).finally(async () => {
        const newBalance = await vault.getUserBalance(selectedVault.address, address)
        const newIdleFunds = await vault.getIdleFunds(selectedVault.address!)
        const newInvestedFunds = await vault.getInvestedFunds(selectedVault.address)
        const newTVL = await vault.getTVL(selectedVault?.address!)
        const newVaultData: Partial<VaultData> = {
          address: selectedVault.address,
          userBalance: newBalance || 0,
          idleFunds: newIdleFunds,
          investedFunds: newInvestedFunds,
          TVL: newTVL
        }
        dispatch(updateVaultData(newVaultData))
      });
    }
    catch (error: any) {
      console.error('Error:', error)
      await statusModal.handleError(error.toString())
    } finally {
      set_amount(0)
      await setTimeout(() => {
        interactModal.setIsOpen(false)
        inspectModal.setIsOpen(false)
      }, 3000)
    }
  }

  const setAmount = (input: any) => {
    if (input < 0 || !selectedVault) return;
    if (vaultMethod === VaultMethod.WITHDRAW) {
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
              <ClipboardRoot value={selectedVault.address}>
                <HStack alignItems={'center'} justifyItems={'center'}>
                  <Text>
                    {selectedVault.address}
                  </Text>
                  <ClipboardIconButton />
                </HStack>
              </ClipboardRoot>
            </GridItem>
            <GridItem colSpan={5} colStart={1} textAlign={'start'}>
              <h2>Total value locked: ${selectedVault?.TVL} {selectedVault.assets[0]?.symbol}</h2>
            </GridItem>
            <GridItem colSpan={6} colStart={6} textAlign={'end'}>
              <h2>User balance in vault: ${`${selectedVault.userBalance ?? 0}`} {selectedVault.assets[0]?.symbol}</h2>
            </GridItem>
            {vaultMethod != VaultMethod.RESCUE &&
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
              vaultMethod === VaultMethod.RESCUE &&
              <>
                <GridItem colSpan={6} textAlign={'end'} alignContent={'center'}>
                  <Text fontSize='lg'>Emergency withdraw from {selectedVault?.name}:</Text>
                </GridItem>
                <GridItem colSpan={6} colEnd={13} textAlign={'end'} >
                  <NativeSelectRoot>
                    <NativeSelectField value={selectedStrategy} onChange={(e) => setSelectedStrategy(e.currentTarget.value)}>
                      <option value={undefined}>Select option</option>
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
            disabled={
              vaultMethod != VaultMethod.RESCUE && amount < 0.0000001 ||
              vaultMethod === VaultMethod.RESCUE && !selectedStrategy
            }
            my={4}
            w={'full'}
            colorScheme='green'
            onClick={() => vaultOperation()}>
            {selectedVault?.method === VaultMethod.DEPOSIT && 'Deposit' ||
              selectedVault?.method === VaultMethod.WITHDRAW && 'Withdraw' ||
              selectedVault?.method === VaultMethod.RESCUE && 'Emergency Withdraw'}
          </Button>
        </DialogBody>
      </DialogContent>
    </>
  )
}
