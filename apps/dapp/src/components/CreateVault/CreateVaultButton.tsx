import { VaultContext } from '@/contexts'
import { useUser } from '@/contexts/UserContext'
import { useDefindexSDK } from '@/hooks/useDefindexSDK'
import { Button, Flex, IconButton, Text } from '@chakra-ui/react'
import { useContext, useEffect, useState } from 'react'
import { HiExternalLink } from 'react-icons/hi'
import { baseMargin } from '../ui/Common'
import { toaster } from '../ui/toaster'

export function CreateVaultButton() {
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
