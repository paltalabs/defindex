"use client"
import { useState, useContext } from "react"
import { Vault, VaultContext, VaultContextType } from "@/contexts"
import useMounted from "@/hooks/useMounted"
import { useVault } from "@/hooks/useVault"


export const VaultProvider = ({ children }: { children: React.ReactNode }) => {
  const isMounted = useMounted();
  const vault = useVault();
  const [newVault, setNewVault] = useState<Vault>({
    name: '',
    symbol: '',
    address: '',
    assetAllocation: [],
    vaultManager: '',
    emergencyManager: '',
    rebalanceManager: '',
    feeReceiver: '',
    feePercent: 0,
    totalSupply: 0,
    upgradable: true,
  });
  const [vaults, setVaults] = useState<Vault[]>([]);
  const [selectedVault, setSelectedVault] = useState<Vault | null>(null);

  const vaultContextValue: VaultContextType = {
    newVault,
    setNewVault,
    vaults,
    setVaults,
    selectedVault,
    setSelectedVault,
  }

  if (!isMounted) return null;
  return (
    <VaultContext.Provider value={vaultContextValue}>
      {children}
    </VaultContext.Provider>
  )
}