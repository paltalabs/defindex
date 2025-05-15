"use client"

import { allowedAssets, Asset, PublicAddressesContext, PublicAddressesContextType, Strategy } from '@/contexts';
import useMounted from '@/hooks/useMounted'
import { extractStrategies, publicAddresses, soroswapRouterAddress } from '@/hooks/usePublicAddresses';
import { StrategyMethod, useStrategyCallback } from '@/hooks/useStrategy';
import { useVault } from '@/hooks/useVault';
import { scValToNative, xdr } from '@stellar/stellar-sdk';
import React, { useEffect } from 'react'
import { useSorobanReact, WalletNetwork } from 'stellar-react';

function PublicAddressesProvider({ children }: { children: React.ReactNode }) {
  const isMounted = useMounted();
  const sorobanContext = useSorobanReact();
  const useStrategyCB = useStrategyCallback();
  const vault = useVault();
  const [addressesFile, setAddressesFile] = React.useState<Record<string, string>>();
  const [networkName, setNetworkName] = React.useState<string>('mainnet');
  const [factoryAddress, setFactoryAddress] = React.useState<string>('');
  const [assets, setAssets] = React.useState<Asset[]>([]);
  const [vaults, setVaults] = React.useState<any[]>([]);
  const [soroswapRouter, setSoroswapRouter] = React.useState<string>('');


  useEffect(() => {
    const name = sorobanContext.activeNetwork === WalletNetwork.PUBLIC ? 'mainnet' : 'testnet';
    setNetworkName(name);
  }, [sorobanContext.activeNetwork]);

  useEffect(() => {
    soroswapRouterAddress(sorobanContext.activeNetwork).then((address) => {
      console.log('Soroswap Router Address:', address);
      setSoroswapRouter(address);
    });
    publicAddresses(sorobanContext.activeNetwork).then(async (addresses: Record<string, string>) => {
      const strategiesList: Strategy[] = await extractStrategies(addresses);
      const assetsList: Asset[] = [];
      for (const asset of allowedAssets) {
        const assetStrategies = strategiesList.filter(strategy => strategy.assetSymbol === asset);
        const assetStrategy = strategiesList.findLast(strategy => strategy.assetSymbol === asset);
        if (!assetStrategy) return;
        const assetAddress = await useStrategyCB(assetStrategy.address, StrategyMethod.ASSET, [], false);
        const parseAssetAddress = scValToNative(assetAddress as xdr.ScVal);
        const tempAsset = {
          address: parseAssetAddress,
          assetSymbol: asset,
          total_amount: 0,
          idle_amount: 0,
          invested_amount: 0,
          strategies: assetStrategies,
          amount: 0

        }
        assetsList.push(tempAsset);
      }
      setAssets(assetsList);
      let factoryAddress = addresses.factory;
      for (const key in addresses) {
        if (key.includes('defindex_factory')) {
          factoryAddress = addresses[key];
          break;
        }
      }
      setAddressesFile(addresses);
      setFactoryAddress(factoryAddress);
    }).catch((error) => {
      console.error('Error fetching public addresses:', error);
    });
  }, [sorobanContext.activeNetwork]);

  useEffect(() => {
    if (!addressesFile) return;
    const vaultsList: any[] = [];
    for (const key in addressesFile) {
      if (key.includes('_vault')) {
        const vaultAddress = addressesFile[key];
        vault.getVaultInfo(vaultAddress).then((vaultInfo) => {
          if (!vaultInfo) return;
          vaultsList.push(vaultInfo);
          setVaults(vaultsList);
        });
      }
    }
  }, [addressesFile]);

  const contextValue: PublicAddressesContextType = {
    networkName,
    setNetworkName,
    factoryAddress,
    setFactoryAddress,
    assets,
    setAssets,
    vaults,
    setVaults,
    soroswapRouterAddress: soroswapRouter,
    setSoroswapRouterAddress: setSoroswapRouter,
  };

  if (!isMounted) return null;
  return (
    <PublicAddressesContext.Provider value={contextValue}>
      {children}
    </PublicAddressesContext.Provider>
  );
}

export default PublicAddressesProvider
