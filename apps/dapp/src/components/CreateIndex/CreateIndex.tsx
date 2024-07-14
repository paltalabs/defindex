import React from 'react'
import {
  Card,
  Button,
  Grid,
  GridItem,
} from '@chakra-ui/react'
import ItemSlider from './Slider'
import AddNewAdapterButton from './AddNewAdapterButton'
import { useAppDispatch, useAppSelector } from '@/store/lib/storeHooks'
import { useFactoryCallback, FactoryMethod } from '@/hooks/useFactory'
import {
  Address,
  nativeToScVal,
  scValToNative,
  xdr,
} from "@stellar/stellar-sdk";
import { pushIndex } from '@/store/lib/features/walletStore'
import { useSorobanReact } from '@soroban-react/core'

function CreateIndex() {
  const adapters = useAppSelector(state => state.adapters.adapters)
  const dispatch = useAppDispatch();
  const { activeChain } = useSorobanReact()
  const factory = useFactoryCallback()

  const deployDefindex = async () => {
    const adapterAddressPairScVal = adapters.map((adapter, index) => {
      return xdr.ScVal.scvMap([
        new xdr.ScMapEntry({
          key: xdr.ScVal.scvSymbol("address"),
          val: (new Address(adapter.address)).toScVal(),

        }),
        new xdr.ScMapEntry({
          key: xdr.ScVal.scvSymbol("index"),
          val: xdr.ScVal.scvU32(index),
        }),
        new xdr.ScMapEntry({
          key: xdr.ScVal.scvSymbol("share"),
          val: xdr.ScVal.scvU32(adapter.value),
        }),
      ]);
    });

    const adapterAddressesScVal = xdr.ScVal.scvVec(adapterAddressPairScVal);

    const createDefindexParams: xdr.ScVal[] = [adapterAddressesScVal];
    console.log('deploying Defindex')
    const result: any = await factory(
      FactoryMethod.CREATE_DEFINDEX,
      createDefindexParams,
      true,
    )
    const parsedResult = scValToNative(result.returnValue);
    dispatch(pushIndex(parsedResult))
    return result;
  }
  const totalValues = useAppSelector(state => state.adapters.totalValues)


  return (
    <>
      <h2>
        Create Index on {activeChain?.name} Chain:
      </h2>
      <Card variant="outline" px={16} py={16} width={'75vw'} bgColor="whiteAlpha.100">
        <Grid templateColumns={'repeat(12, 2fr)'} alignSelf={'end'}>
          <GridItem colStart={12}>
            <AddNewAdapterButton />
          </GridItem>
        </Grid>
        {adapters.map((adapter, index) => (
          <ItemSlider key={index} name={adapter.name} address={adapter.address} value={adapter.value} />
        ))}
        <Grid templateColumns={'repeat(8, 2fr)'} dir='reverse'>
          <GridItem colStart={8} textAlign={'end'}>
            <h2>Total: {totalValues}%</h2>
          </GridItem>
        </Grid>
        <Button isDisabled={totalValues! > 100} colorScheme="green" size="lg" mt={4} onClick={deployDefindex}>
          Deploy DeFindex
        </Button>
      </Card>
    </>
  )
}

export default CreateIndex