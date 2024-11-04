import { VaultMethod, useVaultCallback } from '@/hooks/useVault'
import { useAppSelector } from '@/store/lib/storeHooks'
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

export const InteractWithVault = () => {
  const [amount, set_amount] = useState<number>(0)
  const selectedVault = useAppSelector(state => state.wallet.vaults.selectedVault)
  const vaultMethod = selectedVault?.method

  const { address } = useSorobanReact();
  const vault = useVaultCallback()

  const vaultOperation = async () => {
    if (!address || !vaultMethod) return;
    if (vaultMethod != VaultMethod.EMERGENCY_WITHDRAW) return;
    console.log('Vault method:', vaultMethod)
    const args: xdr.ScVal[] = [
      new Address(selectedVault.address).toScVal()
    ];
    if (vaultMethod === VaultMethod.EMERGENCY_WITHDRAW) {
      if (!selectedVault?.totalValues) throw new Error('Total values is required');
      args.unshift(nativeToScVal((0), { type: "i128" }),)
      const result = await vault(
        VaultMethod.EMERGENCY_WITHDRAW,
        selectedVault?.address!,
        [
          new Address(selectedVault.address).toScVal()
        ],
        true,
      )
      return result
    } else {
      if (!amount) throw new Error('Amount is required');
      args.unshift(nativeToScVal((amount * Math.pow(10, 7)), { type: "i128" }),)
    }
    const result = await vault(
      vaultMethod!,
      selectedVault?.address!,
      args,
      true,
    )
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
              <Textarea
                defaultValue={selectedVault?.address}
                rows={1}
                textAlign={'center'}
                readOnly
                resize={'none'} />
            </GridItem>
            <GridItem colSpan={6} colEnd={13} textAlign={'end'}>
              <h2>Current index balance: {selectedVault?.totalValues}</h2>
            </GridItem>
            {vaultMethod != VaultMethod.EMERGENCY_WITHDRAW &&
              <>
                <GridItem colSpan={6} textAlign={'end'} alignContent={'center'}>
                  <Text fontSize='lg'>Amount to {vaultMethod}:</Text>
                </GridItem>

                <GridItem colSpan={6} colEnd={13} textAlign={'end'} >
                  <Stack alignContent={'center'} alignItems={'center'}>
                    <Input my={4} type="text" onChange={(e) => setAmount(Number(e.target.value))} placeholder='Amount' value={amount} />
                    <InputAddon>$ USDC</InputAddon>
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
                      {selectedVault?.strategies.map((strategy) => {
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
