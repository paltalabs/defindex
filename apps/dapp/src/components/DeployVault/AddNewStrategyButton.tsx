'use client'
import React from 'react'
import { useEffect, useState } from 'react'
import {
  Button,
  For,
  Grid,
  GridItem,
  IconButton,
  Input,
  Skeleton,
  Stack,
  Text,
} from '@chakra-ui/react'
import { MdAdd } from 'react-icons/md'
import {  
  DialogBackdrop,
  DialogBody,
  DialogContent,
  DialogFooter,
  DialogRoot,
  DialogTrigger,
} from '@/components/ui/dialog'
import { useAppDispatch, useAppSelector } from '@/store/lib/storeHooks'
import { getDefaultStrategies, pushAmount, pushAsset } from '@/store/lib/features/vaultStore'
import { useSorobanReact } from '@soroban-react/core'
import { Asset, Strategy } from '@/store/lib/types'
import { CheckboxCard } from '../ui/checkbox-card'
import { getTokenSymbol } from '@/helpers/getTokenInfo'
import { StrategyMethod, useStrategyCallback } from '@/hooks/useStrategy'
import { scValToNative, xdr } from '@stellar/stellar-sdk'
import { InputGroup } from '../ui/input-group'
import { Checkbox } from '../ui/checkbox'

interface AmountInputProps {
  amount: number
  enabled: boolean
}

function AddNewStrategyButton() {
  const dispatch = useAppDispatch();
  const sorobanContext = useSorobanReact()
  const { activeChain } = useSorobanReact()
  const strategyCallback = useStrategyCallback();
  const [open, setOpen] = useState<boolean>(false)
  const [isLoading, setIsLoading] = useState<boolean>(false)
  const [defaultStrategies, setDefaultStrategies] = useState<Strategy[]>([])
  const [asset, setAsset] = useState<Asset>({ address: '', strategies: [] })
  const [amountInput, setAmountInput] = useState<AmountInputProps>({ amount: 0, enabled: false })

  useEffect(() => {
    const fetchStrategies = async () => {
      const tempStrategies = await getDefaultStrategies(activeChain?.name?.toLowerCase() || 'testnet')
      for (const strategy of tempStrategies) {
        const assetAddress = await strategyCallback(
          strategy.address,
          StrategyMethod.ASSET,
          undefined,
          false
        ).then((result) => {
          const resultScval = result as xdr.ScVal;
          const asset = scValToNative(resultScval);
          return asset;
        })
        const assetSymbol = await getSymbol(assetAddress)
        setAsset({ ...asset, address: assetAddress, symbol: assetSymbol! })
      }
      setDefaultStrategies(tempStrategies)
    }
    fetchStrategies();
  }, [activeChain?.networkPassphrase])


  const resetForm = () => {
    setAsset({ address: '', strategies: [] })
    setAmountInput({ amount: 0, enabled: false })
    setOpen(false)
  }

  const getSymbol = async (address: string) => {
    const symbol = await getTokenSymbol(address, sorobanContext)
    if (!symbol) return '';
    return symbol === 'native' ? 'XLM' : symbol
  }

  const handleSelectStrategy = (value: boolean, strategy: Strategy) => {
    setIsLoading(true)
    switch (value) {
      case true:
        const fetchAssets = async () => {
          try {
            const asset = await strategyCallback(
              strategy.address,
              StrategyMethod.ASSET,
              undefined,
              false
            ).then((result) => {
              const resultScval = result as xdr.ScVal;
              const asset = scValToNative(resultScval);
              return asset;
            });
            const symbol = await getSymbol(asset);
            const newAsset = { address: asset, symbol: symbol!, strategies: [strategy] }
            console.log(newAsset)
            setAsset({ address: asset, symbol: symbol!, strategies: [strategy] })
          } catch (error) {
            console.error(error);
          } finally {
            setIsLoading(false)
          }
        };
        fetchAssets();
        break
      case false:
        setAsset({ ...asset, strategies: asset.strategies.filter(str => str.address !== strategy.address) })
        setIsLoading(false)
        break
    }
  }

  const addAsset = async () => {
    const newAsset: Asset = {
      address: asset.address,
      strategies: asset.strategies,
      symbol: asset.symbol
    }
    await dispatch(pushAsset(newAsset))
    if (amountInput.enabled && amountInput.amount! > 0) {
      await dispatch(pushAmount(amountInput.amount!))
    }
    resetForm()
  }



  const handleAmountInput = async (e: any) => {
    const input = e.target.value
    const decimalRegex = /^(\d+)?(\.\d{0,7})?$/
    if (!decimalRegex.test(input)) return
    if (input.startsWith('.')) {
      setAmountInput({ amount: 0 + input, enabled: true });
      return
    }
    setAmountInput({ amount: input, enabled: true });
  }
  return (
    <DialogRoot open={open} onOpenChange={(e) => { setOpen(e.open) }} placement={'center'}>
      <DialogBackdrop backdropFilter='blur(1px)' />
      <DialogTrigger asChild>
        <Button
          size="md"
          textAlign={'end'}
          disabled={defaultStrategies.length === 0}>
          Add new assets
        </Button>
      </DialogTrigger>
      <DialogContent>
        <DialogBody>
          <Text fontSize='lg'>Select strategies:</Text>
          <For each={defaultStrategies}>
            {(strategy, index) => (
              <Stack key={index} my={2}>
                {isLoading && <Skeleton height={12} />}
                {!isLoading && <CheckboxCard
                  checked={asset.strategies.some((str) => str.address === strategy.address)}
                  onCheckedChange={(e) => handleSelectStrategy(!!e.checked, strategy)}
                  label={strategy.name}
                />}
                {asset.strategies.some((str) => str.address === strategy.address) &&
                  <Grid templateColumns={['1fr', null, 'repeat(12, 2fr)']} alignItems={'center'} >
                    <GridItem colSpan={2} colEnd={12}>
                      <Text fontSize={'xs'}>Initial deposit:</Text>
                    </GridItem>
                    <GridItem colStart={12} mt={1} ml={1}>
                      <Checkbox
                        size={'sm'}
                        checked={amountInput.enabled}
                        onCheckedChange={(e) => setAmountInput({ ...amountInput, enabled: !!e.checked })}
                      />
                    </GridItem>

                  </Grid>
                }
                {amountInput.enabled && (
                  <Grid templateColumns={['1fr', null, 'repeat(12, 2fr)']}>
                    <GridItem alignContent={'center'} colStart={1}>
                      <Text fontSize={'sm'}>Amount:</Text>
                    </GridItem>
                    <GridItem colStart={8} colEnd={13}>
                      <InputGroup
                        endElement={`${asset.symbol}`}
                      >
                        <Input onChange={handleAmountInput} value={amountInput.amount} />
                      </InputGroup>
                    </GridItem>
                  </Grid>
                )}
              </Stack>
            )}
          </For>

        </DialogBody>
        <DialogFooter>
          <Button variant='ghost' mr={3} onClick={() => setOpen(false)}>
            Close
          </Button>
          <IconButton
            disabled={asset.strategies.length === 0}
            aria-label='add_strategy'
            colorScheme='green'
            onClick={addAsset}
          >
            <MdAdd />
          </IconButton>
        </DialogFooter>
      </DialogContent>
    </DialogRoot>
  )
}

export default AddNewStrategyButton