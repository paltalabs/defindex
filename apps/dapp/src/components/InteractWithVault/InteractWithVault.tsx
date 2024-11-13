import { VaultMethod, useVaultCallback } from '@/hooks/useVault'
import { useAppDispatch, useAppSelector } from '@/store/lib/storeHooks'
import {
  Button,
  Input,
  Textarea,
  Text,
  Grid,
  GridItem,
  Stack,
  InputAddon,
  NativeSelectField,
} from '@chakra-ui/react'
import { useSorobanReact } from '@soroban-react/core'
import { Address, nativeToScVal, scValToNative, xdr } from '@stellar/stellar-sdk'
import React, { useEffect, useState } from 'react'
import { DialogBody, DialogContent, DialogHeader } from '../ui/dialog'
import { NativeSelectRoot } from '../ui/native-select'
import { InputGroup } from '../ui/input-group'
import { useVault } from '@/hooks/useVault'
import { setVaultTVL } from '@/store/lib/features/walletStore'
import { Strategy } from '@/store/lib/types'

export const InteractWithVault = () => {
  const [amount, set_amount] = useState<number>(0)
  const selectedVault = useAppSelector(state => state.wallet.vaults.selectedVault)
  const vaultMethod = selectedVault?.method

  const { address } = useSorobanReact();
  const vaultCB = useVaultCallback()
  const vault = useVault()
  const dispatch = useAppDispatch()

  const vaultOperation = async () => {
    if (!address || !vaultMethod) return;
    if (!amount) throw new Error('Amount is required');
    if (vaultMethod != VaultMethod.DEPOSIT) return;
    const depositParams: xdr.ScVal[] = [
      xdr.ScVal.scvVec([nativeToScVal((amount * Math.pow(10, 7)), { type: "i128" })]),
      xdr.ScVal.scvVec([nativeToScVal(((amount * 0.9) * Math.pow(10, 7)), { type: "i128" })]),
      new Address(address).toScVal(),
    ]
    console.log('Vault method:', vaultMethod)
    try {
      const result = await vaultCB(
        vaultMethod!,
        selectedVault?.address!,
        depositParams,
        true,
      )
    }
    catch (error) {
      console.error('Error:', error)
    } finally {
      const newTVL = await vault.getVaultTotalValues(selectedVault?.address!)
      const parsedNewTVL = Number(newTVL) / 10 ** 7
      dispatch(setVaultTVL(parsedNewTVL))
    }
  }

  const setAmount = (e: any) => {
    if (Number.isNaN(e)) return;
    set_amount(e)
  }

  return (
    <>
      <DialogContent zIndex={'docked'}>
        <DialogHeader>
          <Text fontSize='xl'>{selectedVault?.method === 'deposit' ? 'Deposit to' : 'Withdraw from'} {selectedVault?.name}</Text>
        </DialogHeader>
        <DialogBody zIndex={'docked'}>
          <Grid templateColumns="repeat(12, 1fr)" gap={6}>
            <GridItem colSpan={12}>
              <h2>Vault address:</h2>
              <Textarea
                defaultValue={selectedVault?.address}
                rows={1}
                textAlign={'center'}
                readOnly
                resize={'none'} />
            </GridItem>
            <GridItem colSpan={6} colEnd={13} textAlign={'end'}>
              <h2>TVL: {selectedVault?.totalValues}</h2>
            </GridItem>
            {vaultMethod != VaultMethod.EMERGENCY_WITHDRAW &&
              <>
                <GridItem colSpan={6} textAlign={'end'} alignContent={'center'}>
                  <Text fontSize='lg'>Amount to {vaultMethod}:</Text>
                </GridItem>

                <GridItem colSpan={6} colEnd={13} textAlign={'end'} >
                  <Stack alignContent={'center'} alignItems={'center'}>
                  <InputGroup
                    endElement={'$'}
                  >
                    <Input my={4} type="text" onChange={(e) => setAmount(Number(e.target.value))} placeholder='Amount' value={amount} />
                  </InputGroup>
                  </Stack>
                </GridItem>
              </>
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
                      {selectedVault?.assets.strategies[0].map((strategy: Strategy) => {
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
