import { Asset, Strategy, VaultContext } from '@/contexts'
import { parsePascalCase } from '@/helpers/utils'
import { createListCollection } from '@chakra-ui/react'
import React, { useContext, useEffect } from 'react'
import { CustomSelect } from '../ui/CustomInputFields'

interface SelectStrategiesProps {
  asset: Asset;
}

export function SelectStrategies({ asset }: SelectStrategiesProps) {
  const vaultContext = useContext(VaultContext);
  const [selectedStrategies, setSelectedStrategies] = React.useState<string[]>([])

  const strategiesCollection = createListCollection({
    items: asset.strategies.map((strategy) => ({
      label: parsePascalCase(strategy.name),
      value: strategy.address,
    }))
  })

  const handleSelect = (e: string[]) => {
    setSelectedStrategies(e)
  }

  useEffect(() => {
    const newStrategies: Strategy[] = asset.strategies
      .filter((strategy) => selectedStrategies.includes(strategy.address))
      .map((strategy, _, arr) => ({
        ...strategy,
        // Initialize with equal distribution
        amount: arr.length > 0 ? Math.floor(100 / arr.length) : 0,
      }));

    // Adjust last strategy to ensure sum is exactly 100
    if (newStrategies.length > 0) {
      const sum = newStrategies.reduce((acc, s) => acc + (s.amount || 0), 0);
      newStrategies[newStrategies.length - 1].amount = (newStrategies[newStrategies.length - 1].amount || 0) + (100 - sum);
    }

    const assetAllocation = vaultContext?.newVault.assetAllocation.map((item) => {
      if (item.address === asset.address) {
        return {
          ...item,
          strategies: newStrategies,
        }
      }
      return item
    });
    vaultContext?.setNewVault({
      ...vaultContext.newVault,
      assetAllocation: assetAllocation!,
    })
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [selectedStrategies, asset.address, asset.strategies])

  return (
    <CustomSelect
      collection={strategiesCollection}
      label="Strategies"
      placeholder="Select strategies"
      value={selectedStrategies}
      onSelect={handleSelect}
    />
  )
}
