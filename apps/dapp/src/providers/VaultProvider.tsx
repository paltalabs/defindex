"use client"
import { useState, useMemo } from "react"
import { Vault, VaultContext, VaultContextType } from "@/contexts"
import useMounted from "@/hooks/useMounted"


export const VaultProvider = ({ children }: { children: React.ReactNode }) => {
  const isMounted = useMounted();

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
    upgradable: true,
  });
  const [vaults, setVaults] = useState<Vault[]>([]);
  const [selectedVault, setSelectedVault] = useState<Vault | null>(null);

  const vaultContextValue: VaultContextType = useMemo(() => ({
    newVault,
    setNewVault,
    vaults,
    setVaults,
    selectedVault,
    setSelectedVault,
  }), [newVault, setNewVault, vaults, setVaults, selectedVault, setSelectedVault])

  if (!isMounted) return null;
  return (
    <VaultContext.Provider value={vaultContextValue}>
      {children}
    </VaultContext.Provider>
  )
}