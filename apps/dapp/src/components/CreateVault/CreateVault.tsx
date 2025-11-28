import { Asset, AssetContext, Strategy, Vault, VaultContext } from '@/contexts'
import { useUser } from '@/contexts/UserContext'
import { isValidAddress } from '@/helpers/address'
import { decimalRegex, parseNumericInput } from '@/helpers/input'
import { parsePascalCase } from '@/helpers/utils'
import { useDefindexSDK } from '@/hooks/useDefindexSDK'
import { Box, Button, Checkbox, createListCollection, Fieldset, Flex, HStack, IconButton, Slider, Stack, Text } from '@chakra-ui/react'
import React, { useContext, useEffect, useState } from 'react'
import { HiExternalLink } from 'react-icons/hi'
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

interface StrategyAllocationSlidersProps {
  assetIndex: number;
  assetAmount: number;
  assetSymbol: string;
}

function StrategyAllocationSliders({ assetIndex, assetAmount, assetSymbol }: StrategyAllocationSlidersProps) {
  const vaultContext = useContext(VaultContext);
  const strategies = vaultContext?.newVault.assetAllocation[assetIndex]?.strategies || [];

  if (strategies.length === 0 || assetAmount <= 0) {
    return null;
  }

  const handlePercentageChange = (strategyIndex: number, newPercentage: number) => {
    if (!vaultContext) return;

    const currentStrategies = [...strategies];
    const oldPercentage = currentStrategies[strategyIndex].amount || 0;
    const diff = newPercentage - oldPercentage;

    // Update the changed strategy
    currentStrategies[strategyIndex] = {
      ...currentStrategies[strategyIndex],
      amount: newPercentage,
    };

    // Distribute the difference among other strategies proportionally
    const otherStrategies = currentStrategies.filter((_, i) => i !== strategyIndex);
    const otherTotal = otherStrategies.reduce((sum, s) => sum + (s.amount || 0), 0);

    if (otherTotal > 0 && diff !== 0) {
      const remaining = -diff;
      otherStrategies.forEach((_, i) => {
        const actualIndex = i >= strategyIndex ? i + 1 : i;
        const currentAmount = currentStrategies[actualIndex].amount || 0;
        const proportion = currentAmount / otherTotal;
        const adjustment = Math.round(remaining * proportion);
        const newAmount = Math.max(0, Math.min(100, currentAmount + adjustment));
        currentStrategies[actualIndex] = {
          ...currentStrategies[actualIndex],
          amount: newAmount,
        };
      });

      // Ensure total is exactly 100
      const total = currentStrategies.reduce((sum, s) => sum + (s.amount || 0), 0);
      if (total !== 100 && currentStrategies.length > 1) {
        const lastOtherIndex = strategyIndex === currentStrategies.length - 1 ? 0 : currentStrategies.length - 1;
        currentStrategies[lastOtherIndex] = {
          ...currentStrategies[lastOtherIndex],
          amount: (currentStrategies[lastOtherIndex].amount || 0) + (100 - total),
        };
      }
    }

    const newAssetAllocation = vaultContext.newVault.assetAllocation.map((item, i) => {
      if (i === assetIndex) {
        return { ...item, strategies: currentStrategies };
      }
      return item;
    });

    vaultContext.setNewVault({
      ...vaultContext.newVault,
      assetAllocation: newAssetAllocation,
    });
  };

  const formatAmount = (percentage: number) => {
    const amount = (assetAmount * percentage) / 100;
    return amount.toFixed(2);
  };

  return (
    <Box w="full" mt={4}>
      <Text fontSize="sm" fontWeight="medium" mb={2} color="gray.400">
        Strategy Allocation
      </Text>
      <Stack gap={3}>
        {strategies.map((strategy, idx) => (
          <Box key={strategy.address} p={3} borderRadius="md" bg="whiteAlpha.50">
            <Flex justify="space-between" align="center" mb={2}>
              <Text fontSize="sm" color="gray.300">
                {parsePascalCase(strategy.name)}
              </Text>
              <Text fontSize="sm" fontWeight="bold" color="green.400">
                {strategy.amount || 0}% = {formatAmount(strategy.amount || 0)} {assetSymbol}
              </Text>
            </Flex>
            <Slider.Root
              value={[strategy.amount || 0]}
              min={0}
              max={100}
              step={1}
              onValueChange={(details) => handlePercentageChange(idx, details.value[0])}
            >
              <Slider.Control>
                <Slider.Track>
                  <Slider.Range />
                </Slider.Track>
                <Slider.Thumb index={0} />
              </Slider.Control>
            </Slider.Root>
          </Box>
        ))}
        <Flex justify="flex-end">
          <Text fontSize="xs" color={strategies.reduce((sum, s) => sum + (s.amount || 0), 0) === 100 ? 'green.400' : 'red.400'}>
            Total: {strategies.reduce((sum, s) => sum + (s.amount || 0), 0)}%
          </Text>
        </Flex>
      </Stack>
    </Box>
  );
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

      <Flex direction="column" align="center" justifyContent="center" alignSelf="center" gap={1} w="110px">
        <Text fontSize="xs" color={upgradable ? 'green.400' : 'gray.400'} whiteSpace="nowrap">
          {upgradable ? 'Upgradable' : 'Non-Upgradable'}
        </Text>
        <Checkbox.Root
          checked={upgradable}
          onCheckedChange={(e) => {
            setUpgradable(!!e.checked)
            vaultContext?.setNewVault({
              ...vaultContext.newVault,
              upgradable: !!e.checked,
            })
          }}
          size="lg"
        >
          <Checkbox.HiddenInput />
          <Checkbox.Control>
            <Checkbox.Indicator strokeWidth={1} />
          </Checkbox.Control>
        </Checkbox.Root>
        
      </Flex>

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
  const { address, activeNetwork } = useUser();
  const { createVault, createVaultAutoInvest } = useDefindexSDK();
  const [loading, setLoading] = useState(false);
  const [disabled, setDisabled] = useState(false);

  useEffect(() => {
    if (
      !address
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
  }, [
    address,
    vaultContext?.newVault.name,
    vaultContext?.newVault.symbol,
    vaultContext?.newVault.vaultManager,
    vaultContext?.newVault.emergencyManager,
    vaultContext?.newVault.rebalanceManager,
    vaultContext?.newVault.feeReceiver,
    vaultContext?.newVault.feePercent,
    vaultContext?.newVault.assetAllocation.length,
    vaultContext
  ]);

  const handleCreateVault = async () => {
    if (!vaultContext || !address) return;
    setLoading(true);

    const newVault = vaultContext.newVault;
    if (!newVault) return;

    // Convert fee percentage to basis points (1% = 100 bps)
    const vaultFeeBps = Math.round(newVault.feePercent * 100);

    // Check if there's a deposit with strategy allocation
    const hasDeposit = newVault.assetAllocation.some((asset) => asset.amount > 0);
    const hasStrategyAllocation = newVault.assetAllocation.some((asset) =>
      asset.strategies.some((strategy) => (strategy.amount || 0) > 0)
    );

    try {
      let result;

      if (hasDeposit && hasStrategyAllocation) {
        // Use createVaultAutoInvest - creates vault, deposits, and invests in strategies
        const autoInvestConfig = {
          caller: address,
          roles: {
            emergencyManager: newVault.emergencyManager,
            rebalanceManager: newVault.rebalanceManager,
            feeReceiver: newVault.feeReceiver,
            manager: newVault.vaultManager,
          },
          name: newVault.name,
          symbol: newVault.symbol,
          vaultFee: vaultFeeBps,
          upgradable: newVault.upgradable,
          assets: newVault.assetAllocation.map((asset) => ({
            address: asset.address,
            symbol: asset.symbol || '',
            amount: Math.round(asset.amount * 10_000_000), // Convert to stroops
            strategies: asset.strategies.map((strategy) => ({
              address: strategy.address,
              name: strategy.name,
              // Calculate amount based on percentage
              amount: Math.round((asset.amount * (strategy.amount || 0) / 100) * 10_000_000),
            })),
          })),
        };

        result = await createVaultAutoInvest(autoInvestConfig);

        const txHash = result.txHash || result.hash;
        const vaultAddress = result.predictedVaultAddress;
        const network = activeNetwork === 'mainnet' ? 'public' : 'testnet';

        toaster.create({
          title: 'Vault created with auto-invest',
          description: vaultAddress ? (
            <Flex align="center" gap={2}>
              <Text
                as="span"
                cursor="pointer"
                _hover={{ textDecoration: 'underline' }}
                onClick={() => {
                  navigator.clipboard.writeText(vaultAddress);
                  toaster.create({
                    title: 'Address copied',
                    type: 'success',
                    duration: 2000,
                  });
                }}
              >
                {vaultAddress.slice(0, 8)}...{vaultAddress.slice(-8)}
              </Text>
              <IconButton
                aria-label="View on Stellar Expert"
                size="xs"
                variant="ghost"
                onClick={() => {
                  window.open(`https://stellar.expert/explorer/${network}/contract/${vaultAddress}`, '_blank');
                }}
              >
                <HiExternalLink />
              </IconButton>
            </Flex>
          ) : 'Vault created and funds invested successfully',
          type: 'success',
          duration: 10000,
          action: {
            label: 'View transaction',
            onClick: () => {
              window.open(`https://stellar.expert/explorer/${network}/search?term=${txHash}`, '_blank');
            }
          }
        });
      } else {
        // Use createVault - creates vault without deposit
        const vaultConfig = {
          roles: {
            0: newVault.emergencyManager,
            1: newVault.feeReceiver,
            2: newVault.vaultManager,
            3: newVault.rebalanceManager,
          },
          vault_fee_bps: vaultFeeBps,
          assets: newVault.assetAllocation.map((asset) => ({
            address: asset.address,
            strategies: asset.strategies.map((strategy) => ({
              address: strategy.address,
              name: strategy.name,
              paused: strategy.paused,
            })),
          })),
          name_symbol: {
            name: newVault.name,
            symbol: newVault.symbol,
          },
          upgradable: newVault.upgradable,
          caller: address,
        };

        result = await createVault(vaultConfig);

        const txHash = result.txHash || result.hash;
        const vaultAddr = result.predictedVaultAddress;
        const net = activeNetwork === 'mainnet' ? 'public' : 'testnet';

        toaster.create({
          title: 'Vault created',
          description: vaultAddr ? (
            <Flex align="center" gap={2}>
              <Text
                as="span"
                cursor="pointer"
                _hover={{ textDecoration: 'underline' }}
                onClick={() => {
                  navigator.clipboard.writeText(vaultAddr);
                  toaster.create({
                    title: 'Address copied',
                    type: 'success',
                    duration: 2000,
                  });
                }}
              >
                {vaultAddr.slice(0, 8)}...{vaultAddr.slice(-8)}
              </Text>
              <IconButton
                aria-label="View on Stellar Expert"
                size="xs"
                variant="ghost"
                onClick={() => {
                  window.open(`https://stellar.expert/explorer/${net}/contract/${vaultAddr}`, '_blank');
                }}
              >
                <HiExternalLink />
              </IconButton>
            </Flex>
          ) : 'Vault created successfully',
          type: 'success',
          duration: 10000,
          action: {
            label: 'View transaction',
            onClick: () => {
              window.open(`https://stellar.expert/explorer/${net}/search?term=${txHash}`, '_blank');
            }
          }
        });
      }
    } catch (error: unknown) {
      console.error('Error creating vault:', error);
      toaster.create({
        title: 'Error creating vault',
        description: error instanceof Error ? error.message : 'An error occurred',
        type: 'error',
        duration: 5000,
      });
    } finally {
      setLoading(false);
    }
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
        {address ? 'Launch Vault' : 'Connect Wallet'}
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
