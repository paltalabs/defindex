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
  const [balance, set_balance] = useState<number>(0)
  const [isLoading, setIsLoading] = useState<boolean>(false)

  const { address } = useSorobanReact();
  const vault = useVaultCallback()

  const selectedIndex = useAppSelector(state => state.wallet.vaults.selectedVault)

  const depositToVault = async () => {
    if (!address || !amount) return;

    const depositParams: xdr.ScVal[] = [
      nativeToScVal((amount * Math.pow(10, 7)), { type: "i128" }),
      new Address(address).toScVal()
    ];

    console.log('deploying Defindex')
    const result = await vault(
      VaultMethod.DEPOSIT,
      selectedIndex?.address!,
      depositParams,
      true,
    )
    setIsLoading(!isLoading)
    console.log('🚀 ~ deployDefindex ~ result:', result);
    return result;
  }

  const withdrawVault = async () => {
    if (!address) return;

    const withdrawParams: xdr.ScVal[] = [
      new Address(address).toScVal()
    ];

    console.log('withdraw Defindex')
    const result = await vault(
      VaultMethod.WITHDRAW,
      selectedIndex?.address!,
      withdrawParams,
      true,
    )
    setIsLoading(!isLoading)
    console.log('🚀 ~ deployDefindex ~ result:', result);
    return result;
  }

  useEffect(() => {
    //getBalance()
    console.log(selectedIndex)
  }, [selectedIndex, isLoading])

  /*   const getBalance = async () => {
      if (!address) return;
      if (defindex_address.length == 56) {
        const balanceParams: xdr.ScVal[] = [
          new Address(address).toScVal()
        ];
  
        console.log('Defindex balance')
        const result: any = await defindex(
          VaultMethod.BALANCE,
          defindex_address,
          balanceParams,
          false,
        )
        const nativeResult = scValToNative(result)
        const sum = nativeResult.reduce((acc: number, val: number) => Number(acc) + Number(val), 0);
        const parsedResult = sum / Math.pow(10, 7)
        set_balance(parsedResult)
      }
    } */

  const setAmount = (e: any) => {
    if (Number.isNaN(e)) return;
    set_amount(e)
  }

  return (
    <>
      <Card variant="outline" px={16} py={16} bgColor="whiteAlpha.100">
        <Grid templateColumns="repeat(12, 1fr)" gap={6}>
          <GridItem colSpan={12}>
            <Text fontSize='xl'>{selectedIndex?.method === 'deposit' ? 'Deposit to' : 'Withdraw from'}:</Text>
          </GridItem>
          <GridItem colSpan={12}>
            <Textarea
              defaultValue={selectedIndex?.address}
              rows={1}
              textAlign={'center'}
              readOnly
              resize={'none'} />
          </GridItem>
          <GridItem colSpan={4} colEnd={13} textAlign={'end'}>
            <h2>Current index balance: {selectedIndex?.totalValues}</h2>
          </GridItem>
          <GridItem colSpan={6} textAlign={'end'} alignContent={'center'}>
            <Text fontSize='lg'>Amount to {selectedIndex?.method && selectedIndex.method}:</Text>
          </GridItem>
          <GridItem colSpan={6} colEnd={13} textAlign={'end'} >
            <InputGroup alignContent={'center'} alignItems={'center'}>
              <Input my={4} type="text" onChange={(e) => setAmount(Number(e.target.value))} placeholder='Amount' value={amount} />
              <InputRightAddon>$ USDC</InputRightAddon>
            </InputGroup>
          </GridItem>
        </Grid>


        <Button isDisabled={amount < 0.0000001} my={4} colorScheme='green' onClick={depositToVault}>{selectedIndex?.method === 'deposit' ? 'Deposit' : 'Withdraw'}</Button>
        {/* <Button isDisabled={defindex_address.length < 56} my={4} colorScheme='green' onClick={depositDefindex}>Deposit</Button>
        <Button isDisabled={defindex_address.length < 56} my={4} colorScheme='blue' onClick={withdrawDefindex}>Withdraw</Button> */}
      </Card>
    </>
  )
}
