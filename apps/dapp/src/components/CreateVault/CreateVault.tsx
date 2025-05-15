import React, { useContext, useEffect, useState } from 'react'
import BackgroundCard from '../ui/BackgroundCard'
import { Fieldset, HStack, createListCollection, Stack, Button, Flex, Box, Switch } from '@chakra-ui/react'
import { CustomSelect, FormField } from '../ui/CustomInputFields'
import { baseMargin } from '../ui/Common'
import { decimalRegex, parseNumericInput } from '@/helpers/input'
import { Asset, AssetContext, PublicAddressesContext, Strategy, Vault, VaultContext } from '@/contexts'
import { parsePascalCase } from '@/helpers/utils'
import { FactoryMethod, useFactoryCallback } from '@/hooks/useFactory'
import { getAssetParamsSCVal, getCreateDeFindexVaultDepositParams, getCreateDeFindexVaultParams } from '@/helpers/vault'
import { useSorobanReact, WalletNetwork } from 'stellar-react'
import { xdr } from '@stellar/stellar-sdk'
import { soroswapRouterAddress } from '@/hooks/usePublicAddresses'
import { toaster } from '../ui/toaster'
import { isValidAddress } from '@/helpers/address'
import { HiCheck, HiX } from 'react-icons/hi'

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
  const publicAddressesContext = useContext(PublicAddressesContext);
  const vaultContext = useContext(VaultContext);
  const [selectedAssets, setSelectedAssets] = React.useState<Asset[]>([])
  const [assetsCollection, setAssetsCollection] = React.useState<any>([])
  const handleSelect = (e: any) => {
    const selected = publicAddressesContext?.assets.filter((asset) => e.includes(asset.address))
    console.log('Selected assets:', selected)
    setSelectedAssets(selected || [])
  }
  useEffect(() => {
    const updateAssets = async () => {
      let newAssets: Asset[] = await Promise.all(selectedAssets.map(async (asset) => ({
        address: asset.address,
        strategies: [],
        assetSymbol: asset.assetSymbol.toUpperCase(),
        total_amount: asset.total_amount,
        invested_amount: asset.invested_amount,
        idle_amount: asset.idle_amount,
        amount: 0,
      })));

      vaultContext?.setNewVault({
        ...vaultContext.newVault,
        assetAllocation: newAssets,
      });
    };
    updateAssets();
  }, [selectedAssets])


  useEffect(() => {
    if (!publicAddressesContext) return
    const collection = createListCollection({
      items: publicAddressesContext.assets.map((asset) => ({
        label: asset.assetSymbol.toUpperCase(),
        value: asset.address,
      }))
    })
    setAssetsCollection(collection)
  }, [publicAddressesContext && publicAddressesContext.assets])
  if (!assetsCollection || assetsCollection.length === 0) return null
  return (
    <CustomSelect
      collection={assetsCollection}
      label="Assets"
      placeholder="Select assets"
      value={selectedAssets.map((asset) => asset.address)}
      onSelect={handleSelect}
      multiple={true}
    />
  )
}

function SelectStrategies({ asset }: { asset: Asset }) {
  const vaultContext = useContext(VaultContext);
  const [selectedStrategies, setSelectedStrategies] = React.useState<string[]>([])
  const [strategiesCollection, setStrategiesCollection] = React.useState<any>([])

  useEffect(() => {
    const collection = createListCollection({
      items: asset.strategies.map((strategy) => ({
        label: parsePascalCase(strategy.name),
        value: strategy.address,
      }))
    })
    setStrategiesCollection(collection)
  }, [asset && asset.strategies])

  const handleSelect = (e: any) => {
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
  }, [selectedStrategies])

  if (!asset || !asset.strategies || asset.strategies.length === 0) return null
  return (
    <CustomSelect
      collection={strategiesCollection}
      label="Strategies"
      placeholder="Select strategies"
      value={selectedStrategies}
      onSelect={handleSelect}
      multiple={true}
    />
  )
}

function AddStrategies() {
  const publicAddressesContext = useContext(PublicAddressesContext);
  const vaultContext = useContext(VaultContext);

  const handleDepositAmount = (e: any, i: number) => {
    if (!decimalRegex.test(e.target.value) && e.target.value != '') return
    const assetAllocation = vaultContext?.newVault.assetAllocation.map((item, index) => {
      let newItem = item
      if (item.address === vaultContext?.newVault.assetAllocation[i].address) {
        newItem = {
          ...item,
          amount: parseNumericInput(e.target.value, 2),
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
              label={item.assetSymbol?.toUpperCase() || ''}
              placeholder="Initial deposit"
              type="number"
              min={0}
              value={vaultContext.newVault.assetAllocation[index].amount}
              onChange={(e) => handleDepositAmount(e, index)}
            />
            <SelectStrategies asset={publicAddressesContext?.assets[index]!} />
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

      <Switch.Root
        onCheckedChange={(e) => {
          vaultContext?.setNewVault({
            ...vaultContext.newVault,
            upgradable: e.checked,
          })
        }}
        colorPalette={'green'}
        alignContent={'center'}
      >
        <Switch.HiddenInput />
        <Switch.Control>
          <Switch.Thumb>
            <Switch.ThumbIndicator fallback={<HiX color={'white'} />}>
              <HiCheck />
            </Switch.ThumbIndicator>
          </Switch.Thumb>
        </Switch.Control>
        <Switch.Label>
          Upgradable
        </Switch.Label>
      </Switch.Root>

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

  const handleWarning = () => {
    if (vaultContext?.newVault.feePercent! > 50) {
      console.log('show warning')
      setShowWarning(true)
    }
    else {

      setShowWarning(false)
    }
  }
  useEffect(() => {
    handleWarning()
  }, [vaultContext?.newVault.feePercent])
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
        invalid={!isValidAddress(vaultContext!.newVault.feeReceiver!)}
        errorMessage={!vaultContext!.newVault.feeReceiver || !isValidAddress(vaultContext!.newVault.feeReceiver) ? 'Invalid address' : ''}
      />
      <FormField
        label="Fee percentage"
        placeholder="Percentage"
        type='number'
        min={0}
        max={100}
        value={vaultContext!.newVault.feePercent}
        onChange={handeInput}
        invalid={showWarning}
        errorMessage={'Too high fees could lead to issues'}
      />
    </VaultConfigSection>
  );
}

function CreateVaultButton() {
  const vaultContext = useContext(VaultContext);
  const publicAddresses = useContext(PublicAddressesContext);
  const useFactory = useFactoryCallback();
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
  }, [sorobanContext, vaultContext, vaultContext!.newVault]);

  const handleCreateVault = async () => {
    if (!vaultContext || !publicAddresses) return;
    setLoading(true);
    const soroswapRouter = publicAddresses.soroswapRouterAddress;

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
        const result = await useFactory(FactoryMethod.CREATE_DEFINDEX_VAULT_DEPOSIT, params, true);
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
      } catch (error: any) {
        console.error('Error creating vault:', error);
        toaster.create({
          title: 'Error creating vault',
          description: error.message,
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
        const result = await useFactory(FactoryMethod.CREATE_DEFINDEX_VAULT, params, true);
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
      } catch (error: any) {
        console.error('Error creating vault:', error);
        toaster.create({
          title: 'Error creating vault',
          description: error.message,
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
