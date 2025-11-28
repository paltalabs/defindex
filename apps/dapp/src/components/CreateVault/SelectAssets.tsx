import { Asset, AssetContext, VaultContext } from '@/contexts'
import { createListCollection } from '@chakra-ui/react'
import React, { useContext, useEffect } from 'react'
import { CustomSelect } from '../ui/CustomInputFields'

export function SelectAssets() {
  const assetContext = useContext(AssetContext);
  const vaultContext = useContext(VaultContext);
  const [selectedAssets, setSelectedAssets] = React.useState<Asset[]>([])

  const handleSelect = (e: string[]) => {
    const selected = assetContext?.assets.filter((asset) => e.includes(asset.address))
    setSelectedAssets(selected || [])
  }

  useEffect(() => {
    const newAssets: Asset[] = selectedAssets.map((asset) => ({
      address: asset.address,
      strategies: [],
      symbol: asset.symbol,
      amount: 0,
    }));
    vaultContext?.setNewVault({
      ...vaultContext.newVault,
      assetAllocation: newAssets,
    })
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [selectedAssets])

  const assetsCollection = createListCollection({
    items: assetContext?.assets.map((asset) => ({
      label: asset.symbol?.toUpperCase() || '',
      value: asset.address,
    })) || []
  })

  return (
    <CustomSelect
      collection={assetsCollection}
      label="Assets"
      placeholder="Select assets"
      value={selectedAssets.map((asset) => asset.address)}
      onSelect={handleSelect}
    />
  )
}
