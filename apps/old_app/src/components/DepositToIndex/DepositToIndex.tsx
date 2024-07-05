import { DefindexMethod, useDefindexCallback } from '@/hooks/useDefindex'
import { useAppSelector } from '@/store/lib/storeHooks'
import { Button, Card, Input, Radio, RadioGroup, Stack } from '@chakra-ui/react'
import { useSorobanReact } from '@soroban-react/core'
import { Address, nativeToScVal, scValToNative, xdr } from '@stellar/stellar-sdk'
import React, { useEffect, useState } from 'react'

function DepositToIndex() {
  const [defindex_address, set_defindex_address] = useState<string>(String)
  const [amount, set_amount] = useState<number>(0)
  const [balance, set_balance] = useState<number>(0)
  const [isLoading, setIsLoading] = useState<boolean>(false)

  const { address } = useSorobanReact();
  const defindex = useDefindexCallback()

  const createdIndexes = useAppSelector(state => state.wallet.createdIndexes)

  const depositDefindex = async () => {
    if (!address || !amount) return;

    const depositParams: xdr.ScVal[] = [
      nativeToScVal((amount * Math.pow(10, 7)), { type: "i128" }),
      new Address(address).toScVal()
    ];

    console.log('deploying Defindex')
    const result = await defindex(
      DefindexMethod.DEPOSIT,
      defindex_address,
      depositParams,
      true,
    )
    setIsLoading(!isLoading)
    console.log('🚀 ~ deployDefindex ~ result:', result);
    return result;
  }

  const withdrawDefindex = async () => {
    if (!address) return;

    const withdrawParams: xdr.ScVal[] = [
      new Address(address).toScVal()
    ];

    console.log('withdraw Defindex')
    const result = await defindex(
      DefindexMethod.WITHDRAW,
      defindex_address,
      withdrawParams,
      true,
    )
    setIsLoading(!isLoading)
    console.log('🚀 ~ deployDefindex ~ result:', result);
    return result;
  }

  useEffect(() => {
    getBalance()
  }, [defindex_address, isLoading])

  const getBalance = async () => {
    if (!address) return;
    if (defindex_address.length == 56) {
      const balanceParams: xdr.ScVal[] = [
        new Address(address).toScVal()
      ];

      console.log('Defindex balance')
      const result: any = await defindex(
        DefindexMethod.BALANCE,
        defindex_address,
        balanceParams,
        false,
      )
      const nativeResult = scValToNative(result)
      const sum = nativeResult.reduce((acc: number, val: number) => Number(acc) + Number(val), 0);
      const parsedResult = sum / Math.pow(10, 7)
      set_balance(parsedResult)
    }
  }

  return (
    <>
      <h2>DepositToIndex</h2>
      <Card variant="outline" px={16} py={16} bgColor="whiteAlpha.100">
        <RadioGroup defaultValue='0' onChange={(e) => set_defindex_address(e)}>
          <Stack>
            {createdIndexes.map((index, i) => (
              <Radio value={index} key={i}>{index}</Radio>
            ))}
          </Stack>
        </RadioGroup>
        <Input my={4} type="text" onChange={(e) => set_defindex_address(e.target.value)} placeholder='Defindex address' value={defindex_address} />
        <Input my={4} type="text" onChange={(e) => set_amount(Number(e.target.value))} placeholder='Amount' value={amount} />
        <Button my={4} colorScheme='green' onClick={depositDefindex}>Deposit</Button>
        <Button my={4} colorScheme='blue' onClick={withdrawDefindex}>Withdraw</Button>
        <h2>Balance: {balance}</h2>
      </Card>
    </>
  )
}

export default DepositToIndex