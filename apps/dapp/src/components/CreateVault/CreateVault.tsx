import { Asset, AssetContext, Strategy, Vault, VaultContext } from '@/contexts'
import { isValidAddress } from '@/helpers/address'
import { decimalRegex, parseNumericInput } from '@/helpers/input'
import { parsePascalCase } from '@/helpers/utils'
import { getAssetParamsSCVal, getCreateDeFindexVaultDepositParams, getCreateDeFindexVaultParams } from '@/helpers/vault'
import { FactoryMethod, useFactoryCallback } from '@/hooks/useFactory'
import { soroswapRouterAddress } from '@/hooks/usePublicAddresses'
import { Button, createListCollection, Fieldset, Flex, HStack, Stack } from '@chakra-ui/react'
import { xdr } from '@stellar/stellar-sdk'
import React, { useContext, useEffect, useState } from 'react'
import { useSorobanReact, WalletNetwork } from 'stellar-react'
import BackgroundCard from '../ui/BackgroundCard'
import { baseMargin } from '../ui/Common'
import { CustomSelect, FormField } from '../ui/CustomInputFields'
import { toaster } from '../ui/toaster'

interface VaultConfigSectionProps {
  title: string;
  children: React.ReactNode;
}

function VaultConfigSection({ title, children }: VaultConfigSectionProps) {
  return (
    <BackgroundCard title={title} titleFontWeight='bold' titleFontSize='xl'>
      <Fieldset.Root mt={baseMargin}>
        <Fieldset.Content>
          <HStack gap={4} w={'full'} alignContent={'center'} justifyContent={'center'} alignItems={'center'} justifyItems={'center'}>
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

  const handleSelect = (e: string[]) => {
    setSelectedStrategies(e)
  }

  useEffect(() => {
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

function AddStrategies() {
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
      <HStack>
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
          </Stack>
        ))}
      </HStack>
    </BackgroundCard>
  )
}

function VaultConfig() {
  const vaultContext = useContext(VaultContext);
  const [upgradable, setUpgradable] = React.useState(true)
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
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [vaultConfig])

  const handleVaultNameChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const value = e.target.value;
    if (value.length <= 20) {
      setVaultConfig({ ...vaultConfig, name: value });
    }
  }

  return (
    <VaultConfigSection title="Creating a Vault">
      <FormField
        label="Vault Name"
        placeholder="Vault name"
        value={vaultConfig.name}
        onChange={handleVaultNameChange}
        invalid={vaultConfig.name !== undefined && vaultConfig.name.length == 20}
        errorMessage={vaultConfig.name && vaultConfig.name.length >= 20 ? 'Vault name must be 20 characters or less' : ''}
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

      <Button
        variant={upgradable ? 'solid' : 'outline'}
        size={'lg'}
        colorPalette={'green'}
        alignSelf={'end'}
        p={4}
        rounded={16}
        onClick={() => {
          setUpgradable(!upgradable)
          vaultContext?.setNewVault({
            ...vaultContext.newVault,
            upgradable: !upgradable,
          })
        }}
      >
        {upgradable ? 'Upgradable' : 'Non Upgradable'}
      </Button>

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
    // eslint-disable-next-line react-hooks/exhaustive-deps
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
        invalid={!isValidAddress(managerConfig.vaultManager!)}
        errorMessage={!managerConfig.vaultManager || !isValidAddress(managerConfig.vaultManager) ? 'Invalid address' : ''}
      />
      <FormField
        label="Emergency Manager"
        placeholder="Emergency manager address"
        value={managerConfig.emergencyManager}
        onChange={(e) => {
          setManagerConfig({ ...managerConfig, emergencyManager: e.target.value })
        }}
        invalid={!isValidAddress(managerConfig.emergencyManager!)}
        errorMessage={!managerConfig.emergencyManager || !isValidAddress(managerConfig.emergencyManager) ? 'Invalid address' : ''}
      />
      <FormField
        label="Rebalance manager"
        placeholder="Rebalance manager address"
        value={managerConfig.rebalanceManager}
        onChange={(e) => {
          setManagerConfig({ ...managerConfig, rebalanceManager: e.target.value })
        }}
        invalid={!isValidAddress(managerConfig.rebalanceManager!)}
        errorMessage={!managerConfig.rebalanceManager || !isValidAddress(managerConfig.rebalanceManager) ? 'Invalid address' : ''}
      />
    </VaultConfigSection>
  );
}

function FeeConfig() {
  const vaultContext = useContext(VaultContext);
  const [showWarning, setShowWarning] = useState(false)

  useEffect(() => {
    const handleWarning = () => {
      if (vaultContext && vaultContext.newVault.feePercent > 50) {
        console.log('show warning')
        setShowWarning(true)
      }
      else {
        setShowWarning(false)
      }
    }
    handleWarning()
  }, [vaultContext])
  const handleInput = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (!decimalRegex.test(e.target.value) && e.target.value != '') return
    if (e.target.value == '') {
      vaultContext?.setNewVault({
        ...vaultContext.newVault,
        feePercent: 0,
      })
      return
    }
    if (parseFloat(e.target.value) >= 100) {
      e.target.value = '100'
    }
    vaultContext?.setNewVault({
      ...vaultContext.newVault,
      feePercent: Number(parseNumericInput(e.target.value, 2) || vaultContext.newVault.feePercent),
    })
  }

  const handleFeeReceiver = (e: React.ChangeEvent<HTMLInputElement>) => {
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
        invalid={!isValidAddress(vaultContext!.newVault.feeReceiver!)}
        errorMessage={!vaultContext!.newVault.feeReceiver || !isValidAddress(vaultContext!.newVault.feeReceiver) ? 'Invalid address' : ''}
      />
      <FormField
        label="Fee percentage"
        placeholder="Percentage"
        type='number'
        min={0}
        max={100}
        value={parseNumericInput(vaultContext!.newVault.feePercent.toString(), 2) || 0}
        onChange={handleInput}
        invalid={showWarning}
        errorMessage={'Too high fees could lead to issues'}
      />
    </VaultConfigSection>
  );
}

function CreateVaultButton() {
  const vaultContext = useContext(VaultContext);
  const factoryCallback = useFactoryCallback();
  const sorobanContext = useSorobanReact();
  const [loading, setLoading] = useState(false);
  const [disabled, setDisabled] = useState(false);

  useEffect(() => {
    if (
      !sorobanContext.address
      || !vaultContext
      || !vaultContext.newVault.name
      || !vaultContext.newVault.symbol
      || !vaultContext.newVault.vaultManager
      || !vaultContext.newVault.emergencyManager
      || !vaultContext.newVault.rebalanceManager
      || !vaultContext.newVault.feeReceiver
      || !vaultContext.newVault.feePercent
      || vaultContext.newVault.assetAllocation.length === 0
    ) {
      setDisabled(true);
    } else {
      setDisabled(false);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [
    sorobanContext.address,
    vaultContext?.newVault.name,
    vaultContext?.newVault.symbol,
    vaultContext?.newVault.vaultManager,
    vaultContext?.newVault.emergencyManager,
    vaultContext?.newVault.rebalanceManager,
    vaultContext?.newVault.feeReceiver,
    vaultContext?.newVault.feePercent,
    vaultContext?.newVault.assetAllocation.length
  ]);

  const handleCreateVault = async () => {
    if (!vaultContext) return;
    setLoading(true);
    const soroswapRouter = await soroswapRouterAddress(sorobanContext.activeNetwork);

    const newVault = vaultContext.newVault;
    if (!newVault) return;

    const assetParams = getAssetParamsSCVal(
      newVault.assetAllocation
    );
    let params: xdr.ScVal[] = [];

    const isCreateAndDeposit = newVault.assetAllocation.some((asset) => asset.amount > 0);
    if (isCreateAndDeposit) {
      if (!sorobanContext.address) return;
      params = getCreateDeFindexVaultDepositParams(
        sorobanContext.address,
        newVault.emergencyManager,
        newVault.rebalanceManager,
        newVault.feeReceiver,
        newVault.vaultManager,
        newVault.feePercent,
        newVault.name,
        newVault.symbol,
        assetParams,
        soroswapRouter,
        newVault.upgradable,
        newVault.assetAllocation
      );
      try {
        const result = await factoryCallback(FactoryMethod.CREATE_DEFINDEX_VAULT_DEPOSIT, params, true);
        toaster.create({
          title: 'Vault created',
          description: 'Vault created successfully',
          type: 'success',
          duration: 5000,
          action: {
            label: 'View transaction',
            onClick: () => {
              window.open(`https://stellar.expert/explorer/${sorobanContext.activeNetwork == WalletNetwork.PUBLIC ? 'public' : 'testnet'}/search?term=${result.txHash}`, '_blank');
            }
          }
        });
        setLoading(false);
      } catch (error: unknown) {
        console.error('Error creating vault:', error);
        toaster.create({
          title: 'Error creating vault',
          description: error instanceof Error ? error.message : 'An error occurred',
          type: 'error',
          duration: 5000,
        });
        setLoading(false);
        return;
      }
    } else {
      params = getCreateDeFindexVaultParams(
        newVault.emergencyManager,
        newVault.rebalanceManager,
        newVault.feeReceiver,
        newVault.vaultManager,
        newVault.feePercent,
        newVault.name,
        newVault.symbol,
        assetParams,
        soroswapRouter,
        newVault.upgradable,
      );
      try {
        const result = await factoryCallback(FactoryMethod.CREATE_DEFINDEX_VAULT, params, true);
        toaster.create({
          title: 'Vault created',
          description: `Vault created successfully`,
          type: 'success',
          duration: 5000,
          action: {
            label: 'View transaction',
            onClick: () => {
              window.open(`https://stellar.expert/explorer/${sorobanContext.activeNetwork == WalletNetwork.PUBLIC ? 'public' : 'testnet'}/search?term=${result.txHash}`, '_blank');
            }
          }
        });
        setLoading(false);
      } catch (error: unknown) {
        console.error('Error creating vault:', error);
        toaster.create({
          title: 'Error creating vault',
          description: error instanceof Error ? error.message : 'An error occurred',
          type: 'error',
          duration: 5000,
        });
        setLoading(false);
        return;
      }
    }
    setLoading(false);
  }
  return (
    <Flex w={'full'} h={'full'} alignItems={'center'} justifyContent={'end'}>
      <Button
        px={4}
        rounded={15}
        variant={'outline'}
        size={'lg'}
        mb={baseMargin}
        colorPalette={'green'}
        loading={loading}
        onClick={handleCreateVault}
        disabled={disabled}
        className='custom-button'
      >
        {sorobanContext.address ? 'Launch Vault' : 'Connect Wallet'}
      </Button>
    </Flex>
  )
}

function CreateVault() {


  return (
    <Stack alignContent={'center'} justifyContent={'center'} gap={6} mt={'10dvh'}>
      <VaultConfig />
      <AddStrategies />
      <ManagerConfig />
      <FeeConfig />
      <CreateVaultButton />
    </Stack>
  )
}

export default CreateVault
