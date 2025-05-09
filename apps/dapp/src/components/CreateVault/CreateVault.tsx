import React, { useContext, useEffect } from 'react'
import BackgroundCard from '../ui/BackgroundCard'
import { Fieldset, HStack, createListCollection, Stack, Button, Flex } from '@chakra-ui/react'
import { CustomSelect, FormField } from '../ui/CustomInputFields'
import { baseMargin } from '../ui/Common'
import { decimalRegex, parseNumericInput } from '@/helpers/input'
import { Asset, AssetContext, Strategy, Vault, VaultContext } from '@/contexts'
import { parsePascalCase } from '@/helpers/utils'

interface VaultConfigSectionProps {
  title: string;
  children: React.ReactNode;
}

function VaultConfigSection({ title, children }: VaultConfigSectionProps) {
  return (
    <BackgroundCard title={title} titleFontWeight='bold' titleFontSize='xl'>
      <Fieldset.Root mt={baseMargin}>
        <Fieldset.Content>
          <HStack gap={4} w={'full'} alignContent={'center'} justifyContent={'center'}>
            {children}
          </HStack>
        </Fieldset.Content>
      </Fieldset.Root>
    </BackgroundCard>
  );
}

function SelectAssets() {
  const assetContext = useContext(AssetContext);
  const vaultContext = useContext(VaultContext);
  const [selectedAssets, setSelectedAssets] = React.useState<Asset[]>([])
  const handleSelect = (e: any) => {
    console.log('Selected assets:', e)
    console.log('assets:', assetContext?.assets)
    const selected = assetContext?.assets.filter((asset) => e.includes(asset.address))
    setSelectedAssets(selected || [])
  }
  useEffect(() => {
    console.log('Selected assets:', selectedAssets)
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
  }, [selectedAssets])
  const assetsCollection = createListCollection({
    items: assetContext?.assets.map((asset) => ({
      label: asset.symbol?.toUpperCase()!,
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

function SelectStrategies({ asset }: { asset: Asset }) {
  const vaultContext = useContext(VaultContext);
  const [selectedStrategies, setSelectedStrategies] = React.useState<string[]>([])
  const strategiesCollection = createListCollection({
    items: asset.strategies.map((strategy) =>
    ({
      label: parsePascalCase(strategy.name),
      value: strategy.address,
    }))
  })

  const handleSelect = (e: any) => {
    console.log('Selected strategies:', e)
    console.log('strategies:', asset.strategies)
    setSelectedStrategies(e)
  }

  useEffect(() => {
    console.log('Selected strategies:', selectedStrategies)
    const newStrategies: Strategy[] = asset.strategies.filter((strategy) => selectedStrategies.includes(strategy.address))
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
    console.log('New vault:', vaultContext?.newVault)
  }, [selectedStrategies])
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

function AddStrategies() {
  const assetContext = useContext(AssetContext);
  const vaultContext = useContext(VaultContext);

  const handleDepositAmount = (e: any, i: number) => {
    if (!decimalRegex.test(e.target.value) && e.target.value != '') return
    const assetAllocation = vaultContext?.newVault.assetAllocation.map((item, index) => {
      if (item.address === assetContext?.assets[i].address) {
        return {
          ...item,
          amount: parseNumericInput(e.target.value, 7),
        }
      }
      return item
    });
    vaultContext?.setNewVault({
      ...vaultContext.newVault,
      assetAllocation: assetAllocation!,
    })
  }


  return (
    <BackgroundCard title='Add Strategies' titleFontWeight='bold' titleFontSize='xl'>
      <HStack>
        {vaultContext?.newVault.assetAllocation.map((item, index) => (
          <Stack key={index} w={'full'} alignContent={'center'} justifyContent={'center'} mt={baseMargin} gap={4}>
            <FormField
              label={item.symbol?.toUpperCase() || ''}
              placeholder="Initial deposit"
              type="number"
              min={0}
              value={vaultContext.newVault.assetAllocation[index].amount}
              onChange={(e) => handleDepositAmount(e, index)}
            />
            <SelectStrategies asset={assetContext!.assets[index]} />
          </Stack>
        ))}
      </HStack>
    </BackgroundCard>
  )
}

function VaultConfig() {
  const vaultContext = useContext(VaultContext);
  const [vaultConfig, setVaultConfig] = React.useState<Partial<Vault>>({
    name: '',
    symbol: '',
  })

  useEffect(() => {
    vaultContext?.setNewVault({
      ...vaultContext.newVault,
      name: vaultConfig.name || '',
      symbol: vaultConfig.symbol || '',
    })
  }, [vaultConfig])

  return (
    <VaultConfigSection title="Creating a Vault">
      <FormField
        label="Vault Name"
        placeholder="Vault name"
        value={vaultConfig.name}
        onChange={(e) => {
          setVaultConfig({ ...vaultConfig, name: e.target.value })
        }}
      />
      <FormField
        label="Tag for the vault"
        placeholder="Tag name"
        value={vaultConfig.symbol}
        onChange={(e) => {
          setVaultConfig({ ...vaultConfig, symbol: e.target.value })
        }}
      />
      <SelectAssets />
    </VaultConfigSection>
  );
}

function ManagerConfig() {
  const vaultContext = useContext(VaultContext);
  const [managerConfig, setManagerConfig] = React.useState<Partial<Vault>>({
    vaultManager: '',
    emergencyManager: '',
    rebalanceManager: '',
  })
  useEffect(() => {
    vaultContext?.setNewVault({
      ...vaultContext.newVault,
      vaultManager: managerConfig.vaultManager || '',
      emergencyManager: managerConfig.emergencyManager || '',
      rebalanceManager: managerConfig.rebalanceManager || '',
    })
  }, [managerConfig])

  return (
    <VaultConfigSection title="Manager Config">
      <FormField
        label="Manager"
        placeholder="Manager address"
        value={managerConfig.vaultManager}
        onChange={(e) => {
          setManagerConfig({ ...managerConfig, vaultManager: e.target.value })
        }}
      />
      <FormField
        label="Emergency Manager"
        placeholder="Emergency manager address"
        value={managerConfig.emergencyManager}
        onChange={(e) => {
          setManagerConfig({ ...managerConfig, emergencyManager: e.target.value })
        }}
      />
      <FormField
        label="Rebalance manager"
        placeholder="Rebalance manager address"
        value={managerConfig.rebalanceManager}
        onChange={(e) => {
          setManagerConfig({ ...managerConfig, rebalanceManager: e.target.value })
        }}
      />
    </VaultConfigSection>
  );
}

function FeeConfig() {
  const vaultContext = useContext(VaultContext);

  const handeInput = (e: any) => {
    if (!decimalRegex.test(e.target.value) && e.target.value != '') return
    if (e.target.value == '') {
      vaultContext?.setNewVault({
        ...vaultContext.newVault,
        feePercent: 0,
      })
      return
    }
    if (parseFloat(e.target.value) >= 100) {
      e.target.value = 100
    }
    vaultContext?.setNewVault({
      ...vaultContext.newVault,
      feePercent: parseNumericInput(e.target.value, 2) || vaultContext.newVault.feePercent,
    })
  }

  const handleFeeReceiver = (e: any) => {
    vaultContext?.setNewVault({
      ...vaultContext.newVault,
      feeReceiver: e.target.value,
    })
  }

  return (
    <VaultConfigSection title="Fee Config">
      <FormField
        label="Fee receiver"
        placeholder="Fee receiver address"
        value={vaultContext!.newVault.feeReceiver}
        onChange={handleFeeReceiver}
      />
      <FormField
        label="Fee percentage"
        placeholder="Percentage"
        type='number'
        min={0}
        max={100}
        value={vaultContext!.newVault.feePercent}
        onChange={handeInput}
      />
    </VaultConfigSection>
  );
}

function CreateVaultButton() {
  return (
    <Flex w={'full'} h={'full'} alignItems={'center'} justifyContent={'end'}>
      <Button
        px={4}
        rounded={15}
        variant={'outline'}
        size={'lg'}
        mb={baseMargin}
        colorPalette={'green'}>
        Launch Vault
      </Button>
    </Flex>
  )
}

function CreateVault() {
  return (
    <Stack h={'full'} w={'full'} alignContent={'center'} justifyContent={'center'} gap={6}>
      <VaultConfig />
      <AddStrategies />
      <ManagerConfig />
      <FeeConfig />
      <CreateVaultButton />
    </Stack>
  )
}

export default CreateVault
