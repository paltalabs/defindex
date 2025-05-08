"use client"
import { useState, useEffect } from "react"
import { StrategiesContext, StrategiesContextType, Strategy } from "@/contexts"
import useMounted from "@/hooks/useMounted"
import { extractStrategies, usePublicAddresses } from "@/hooks/usePublicAddresses"
import { useSorobanReact, WalletNetwork } from "stellar-react"


export const StrategiesProvider = ({ children }: { children: React.ReactNode }) => {
  const [strategies, setStrategies] = useState<Strategy[]>([]);
  const [network, setNetwork] = useState<'testnet' | 'mainnet'>('testnet');
  const { data: publicAddresses, isLoading, error } = usePublicAddresses(network);
  const isMounted = useMounted();
  const sorobanContext = useSorobanReact();

  const StrategiesContextValue: StrategiesContextType = {
    strategies,
    setStrategies
  }

  useEffect(() => {
    if (sorobanContext.activeNetwork) {
      const network = sorobanContext.activeNetwork === WalletNetwork.TESTNET ? 'testnet' : 'mainnet';
      setNetwork(network);
    }
  }, [sorobanContext.activeNetwork]);

  useEffect(() => {
    if (publicAddresses && !isLoading) {
      extractStrategies(publicAddresses).then((strategies) => {
        setStrategies(strategies);
      }).catch((error) => {
        console.error('Error fetching strategies:', error);
      });
    }
  }, [publicAddresses, isLoading]);

  if (!isMounted) return null;
  return (
    <StrategiesContext.Provider value={StrategiesContextValue}>
      {strategies.map((strategy) => (
        <div key={strategy.address}>
          <h2>{strategy.assetSymbol}_{strategy.name}</h2>
        </div>
      ))}
      {children}
    </StrategiesContext.Provider>
  )
}