import { VaultMethod, useVaultCallback } from '@/hooks/useVault'
import { useAppSelector } from '@/store/lib/storeHooks'
import {
  Button,
  Card,
  Input,
  Textarea,
  Text,
  Grid,
  GridItem,
  InputGroup,
  InputRightAddon
} from '@chakra-ui/react'
import { useSorobanReact } from '@soroban-react/core'
import { Address, nativeToScVal, scValToNative, xdr } from '@stellar/stellar-sdk'
import React, { useEffect, useState } from 'react'

export const DepositToVault = () => {
  const [amount, set_amount] = useState<number>(0)
  const [isLoading, setIsLoading] = useState<boolean>(false)
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
    } else {
      if (!amount) throw new Error('Amount is required');
      args.unshift(nativeToScVal((amount * Math.pow(10, 7)), { type: "i128" }),)
    }
    // const result = await vault(
    //   vaultMethod!,
    //   selectedVault?.address!,
    //   args,
    //   true,
    // )
    const result = await vault(
      VaultMethod.GETEMERGENCYMANAGER,
      selectedVault?.address!,
      [],
      true,
    )
    return result
  }
  const depositToVault = async () => {
    if (!address || !amount) return;

    const depositParams: xdr.ScVal[] = [
      nativeToScVal((amount * Math.pow(10, 7)), { type: "i128" }),
      new Address(address).toScVal()
    ];

    const result = await vault(
      VaultMethod.DEPOSIT,
      selectedVault?.address!,
      depositParams,
      true,
    )
    setIsLoading(!isLoading)
    console.log('🚀 ~ deployDefindex ~ result:', result);
    return result;
  }

  const setAmount = (e: any) => {
    if (Number.isNaN(e)) return;
    set_amount(e)
  }

  return (
    <>
      <Card variant="outline" px={16} py={16} bgColor="whiteAlpha.100">
        <Grid templateColumns="repeat(12, 1fr)" gap={6}>
          <GridItem colSpan={12}>
            <Text fontSize='xl'>{selectedVault?.method === 'deposit' ? 'Deposit to' : 'Withdraw from'}:</Text>
          </GridItem>
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
                <InputGroup alignContent={'center'} alignItems={'center'}>
                  <Input my={4} type="text" onChange={(e) => setAmount(Number(e.target.value))} placeholder='Amount' value={amount} />
                  <InputRightAddon>$ USDC</InputRightAddon>
                </InputGroup>
              </GridItem>
            </>
          }
        </Grid>
        <Button isDisabled={vaultMethod != VaultMethod.EMERGENCY_WITHDRAW && amount < 0.0000001} my={4} colorScheme='green' onClick={() => vaultOperation()}>{selectedVault?.method.includes('withdraw') ? 'Withdraw' : 'Deposit'}</Button>
      </Card>
    </>
  )
}
