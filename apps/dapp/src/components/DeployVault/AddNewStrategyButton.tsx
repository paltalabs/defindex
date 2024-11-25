'use client'
import {
  DialogBackdrop,
  DialogBody,
  DialogContent,
  DialogFooter,
  DialogRoot,
  DialogTrigger,
} from '@/components/ui/dialog'
import { getTokenSymbol } from '@/helpers/getTokenInfo'
import { StrategyMethod, useStrategyCallback } from '@/hooks/useStrategy'
import { getDefaultStrategies, pushAsset, setAssetAmount } from '@/store/lib/features/vaultStore'
import { useAppDispatch, useAppSelector } from '@/store/lib/storeHooks'
import { Asset, Strategy } from '@/store/lib/types'
import {
  Button,
  For,
  Grid,
  GridItem,
  HStack,
  IconButton,
  Stack,
  Text,
} from '@chakra-ui/react'
import { useSorobanReact } from '@soroban-react/core'
import { scValToNative, xdr } from '@stellar/stellar-sdk'
import { useEffect, useState } from 'react'
import { MdAdd } from 'react-icons/md'
import { Checkbox } from '../ui/checkbox'
import { CheckboxCard } from '../ui/checkbox-card'
import { InputGroup } from '../ui/input-group'
import { NumberInputField, NumberInputRoot } from '../ui/number-input'

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
  const newVault = useAppSelector((state) => state.newVault)
  const [defaultStrategies, setDefaultStrategies] = useState<any[]>([])
  const [selectedAsset, setSelectedAsset] = useState<Asset>({ address: '', strategies: [], symbol: '', amount: 0 })
  const [assets, setAssets] = useState<Asset[]>([])
  const [amountInput, setAmountInput] = useState<AmountInputProps>({ amount: 0, enabled: false })

  const resetForm = () => {
    setSelectedAsset({ address: '', strategies: [], symbol: '' })
    setAmountInput({ amount: 0, enabled: false })
    setOpen(false)
  }

  const getSymbol = async (address: string) => {
    const symbol = await getTokenSymbol(address, sorobanContext)
    if (!symbol) return '';
    return symbol === 'native' ? 'XLM' : symbol
  }

  useEffect(() => {
    const fetchStrategies = async () => {
      const tempStrategies = await getDefaultStrategies(activeChain?.name?.toLowerCase() || 'testnet')
      setDefaultStrategies(tempStrategies)
    }
    fetchStrategies();
  }, [activeChain?.networkPassphrase])

  useEffect(() => {
    const fetchStrategies = async () => {
      const rawDefaultStrategies = await getDefaultStrategies(activeChain?.name?.toLowerCase() || 'testnet')
      const defaultStrategiesWithAssets = await Promise.all(rawDefaultStrategies.map(async (strategy) => {
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
        const asset = { address: assetAddress, strategies: [strategy], symbol: assetSymbol! }
        return asset
      }
      ))
      setAssets(defaultStrategiesWithAssets)
    }
    fetchStrategies();
  }, [activeChain?.networkPassphrase])


  const handleSelectStrategy = (value: boolean, strategy: Strategy) => {
    const selectedAsset = assets.find((asset) => asset.strategies.some((str) => str.address === strategy.address))
    if (selectedAsset) {
      setSelectedAsset(selectedAsset)
    }
  }

  const handleAmountInput = async (e: any) => {
    if (!e) {
      console.log('input is empty')
      setSelectedAsset({ ...selectedAsset, amount: 0 })
    }
    const decimalRegex = /^(\d+)?(\.\d{0,7})?$/
    if (!decimalRegex.test(e)) return
    setAmountInput({ amount: e, enabled: true });
  }
  const strategyExists = (strategy: Strategy) => {
    const exists = newVault.assets.some((asset) => asset.strategies.some((str) => str.address === strategy.address))
    return exists
  }
  const addAsset = async () => {
    const newAsset: Asset = {
      address: selectedAsset.address,
      strategies: selectedAsset.strategies,
      symbol: selectedAsset.symbol
    }
    const exists = strategyExists(selectedAsset.strategies[0]!)
    if (exists) {

    }
    await dispatch(pushAsset(newAsset))
    if (!exists && amountInput.enabled && amountInput.amount! > 0) {
      await dispatch(setAssetAmount({ address: newAsset.address, amount: amountInput.amount! }))
    }
    resetForm()
  }

  return (
    <DialogRoot open={open} onOpenChange={(e) => { setOpen(e.open) }} placement={'center'}>
      <DialogBackdrop backdropFilter='blur(1px)' />
      <DialogTrigger asChild>
        <Button
          size="md"
          textAlign={'end'}
          disabled={defaultStrategies.length === 0}>
          Add Strategy
        </Button>
      </DialogTrigger>
      <DialogContent>
        <DialogBody>
          <Text fontSize='lg'>Select strategies:</Text>
          <For each={defaultStrategies}>
            {(strategy, index) => (
              <Stack key={index} my={2}>
                <CheckboxCard
                  checked={strategyExists(strategy) || selectedAsset.strategies.some((str) => str.address === strategy.address)}
                  disabled={strategyExists(strategy)}
                  onCheckedChange={(e) => handleSelectStrategy(!!e.checked, strategy)}
                  label={strategy.name}
                />
                {selectedAsset.strategies.some((str) => str.address === strategy.address) &&
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
              </Stack>
            )}
          </For>
          {amountInput.enabled && (
            <Grid templateColumns={['1fr', null, 'repeat(12, 2fr)']}>
              <GridItem alignContent={'center'} colStart={1}>
                <Text fontSize={'sm'}>Amount:</Text>
              </GridItem>
              <GridItem colStart={8} colEnd={13}>
                <InputGroup
                  endElement={`${selectedAsset.symbol}`}
                >
                  <NumberInputRoot
                    onValueChange={(e) => handleAmountInput(Number(e.value))}
                  >
                    <NumberInputField />
                  </NumberInputRoot>
                </InputGroup>
              </GridItem>
            </Grid>
          )}
        </DialogBody>
        <DialogFooter>
          <Button variant='ghost' mr={3} onClick={() => setOpen(false)}>
            Close
          </Button>
          <IconButton
            disabled={selectedAsset.strategies.length === 0}
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