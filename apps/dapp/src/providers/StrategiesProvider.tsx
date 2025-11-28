"use client"
import { useState, useEffect, useMemo } from "react"
import { Asset, AssetContext, AssetContextType } from "@/contexts"
import useMounted from "@/hooks/useMounted"
import { extractStrategies, usePublicAddresses } from "@/hooks/usePublicAddresses"
import { useUser } from "@/contexts/UserContext"
import { StrategyMethod, useStrategyCallback } from "@/hooks/useStrategy"
import { scValToNative, xdr } from "@stellar/stellar-sdk"


export const StrategiesProvider = ({ children }: { children: React.ReactNode }) => {
  const [assets, setAssets] = useState<Asset[]>([]);

  const { activeNetwork } = useUser();
  const { data: publicAddresses } = usePublicAddresses(activeNetwork);
  const isMounted = useMounted();
  const strategyCallback = useStrategyCallback();

  useEffect(() => {
    const fetchStrategies = async () => {
      if (!publicAddresses) return
      const extractedStrategies = await extractStrategies(publicAddresses);
      try {
        const results = await Promise.allSettled(
          extractedStrategies.map(async (strategy) => {
            const assetAddress = await strategyCallback(
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

        setAssets(parsedAssets);
      } catch (error) {
        console.error('Error fetching strategies:', error);
      }
    }

    fetchStrategies();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [activeNetwork, publicAddresses]);

  const AssetContextValue: AssetContextType = useMemo(() => ({
    assets,
    setAssets
  }), [assets, setAssets])



  if (!isMounted) return null;
  return (
    <AssetContext.Provider value={AssetContextValue}>
      {children}
    </AssetContext.Provider>
  )
}