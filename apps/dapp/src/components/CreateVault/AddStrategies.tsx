import { AssetContext, VaultContext } from '@/contexts'
import { decimalRegex, parseNumericInput } from '@/helpers/input'
import { HStack, Stack } from '@chakra-ui/react'
import React, { useContext } from 'react'
import BackgroundCard from '../ui/BackgroundCard'
import { baseMargin } from '../ui/Common'
import { FormField } from '../ui/CustomInputFields'
import { SelectStrategies } from './SelectStrategies'
import { StrategyAllocationSliders } from './StrategyAllocationSliders'

export function AddStrategies() {
  const assetContext = useContext(AssetContext);
  const vaultContext = useContext(VaultContext);

  const handleDepositAmount = (e: React.ChangeEvent<HTMLInputElement>, i: number) => {
    if (!decimalRegex.test(e.target.value) && e.target.value != '') return
    const assetAllocation = vaultContext?.newVault.assetAllocation.map((item) => {
      let newItem = item
      const amount = parseNumericInput(e.target.value, 7);
      if (item.address === vaultContext?.newVault.assetAllocation[i].address) {
        newItem = {
          ...item,
          amount: Number(amount),
        }
      }
      return newItem
    });
    vaultContext?.setNewVault({
      ...vaultContext.newVault,
      assetAllocation: assetAllocation!,
    })
  }

  return (
    <BackgroundCard title='Add Strategies' titleFontWeight='bold' titleFontSize='xl'>
      <HStack alignItems="flex-start">
        {vaultContext?.newVault.assetAllocation.map((item, index) => (
          <Stack key={index} w={'full'} alignContent={'center'} justifyContent={'center'} mt={baseMargin} gap={4}>
            <FormField
              label={item.symbol?.toUpperCase() || ''}
              placeholder="Initial deposit"
              type="number"
              min={0}
              value={parseNumericInput(vaultContext.newVault.assetAllocation[index]?.amount.toString(), 7) || 0}
              onChange={(e) => handleDepositAmount(e, index)}
            />
            <SelectStrategies asset={assetContext!.assets.find((a) => a.address === item.address)!} />
            <StrategyAllocationSliders
              assetIndex={index}
              assetAmount={item.amount}
              assetSymbol={item.symbol?.toUpperCase() || ''}
            />
          </Stack>
        ))}
      </HStack>
    </BackgroundCard>
  )
}
