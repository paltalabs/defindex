"use client"
import { useState, useEffect, use } from "react"
import { Asset, AssetContext, AssetContextType, Strategy } from "@/contexts"
import useMounted from "@/hooks/useMounted"
import { extractStrategies, usePublicAddresses } from "@/hooks/usePublicAddresses"
import { useSorobanReact, WalletNetwork } from "stellar-react"
import { StrategyMethod, useStrategyCallback } from "@/hooks/useStrategy"
import { scValToNative, xdr } from "@stellar/stellar-sdk"


export const StrategiesProvider = ({ children }: { children: React.ReactNode }) => {
  const [assets, setAssets] = useState<Asset[]>([]);
  const [network, setNetwork] = useState<'testnet' | 'mainnet'>('testnet');
  const { data: publicAddresses, isLoading, error } = usePublicAddresses(network);
  const isMounted = useMounted();
  const sorobanContext = useSorobanReact();
  const useStrategy = useStrategyCallback();

  const fetchStrategies = async (network: string) => {
    if (!network || !publicAddresses) return
    const extractedStrategies = await extractStrategies(publicAddresses);
    try {
      const results = await Promise.allSettled(
        extractedStrategies.map(async (strategy) => {
          const assetAddress = await useStrategy(
            strategy.address,
            StrategyMethod.ASSET,
            undefined,
            false
          ).then((result) => {
            const scVal = result as xdr.ScVal;
            return scValToNative(scVal);
          });

          if (!assetAddress) {
            throw new Error('Asset address not found');
          }

          return {
            address: assetAddress as string,
            strategies: [strategy],
            symbol: strategy.assetSymbol!,
          } as Asset;
        })
      );

      const fulfilledResults = results.filter(
        (result) => result.status === 'fulfilled'
      ) as PromiseFulfilledResult<Asset>[];

      const parsedAssets = fulfilledResults
        .map((result) => result.value)
        .reduce((acc: Asset[], current) => {
          const existingAsset = acc.find(
            (asset) => asset.address === current.address
          );
          if (existingAsset) {
            existingAsset.strategies.push(...current.strategies);
          } else {
            acc.push(current);
          }
          return acc;
        }, []);

      return parsedAssets;
    } catch (error) {
      console.error('Error fetching strategies:', error);
    }
  }

  useEffect(() => {
    fetchStrategies(network).then((assets) => {
      if (assets) {
        setAssets(assets);
      }
    })
  }, [network, publicAddresses]);

  const AssetContextValue: AssetContextType = {
    assets,
    setAssets
  }

  useEffect(() => {
    if (sorobanContext.activeNetwork) {
      const network = sorobanContext.activeNetwork === WalletNetwork.TESTNET ? 'testnet' : 'mainnet';
      setNetwork(network);
    }
  }, [sorobanContext.activeNetwork]);


  if (!isMounted) return null;
  return (
    <AssetContext.Provider value={AssetContextValue}>
      {children}
    </AssetContext.Provider>
  )
}