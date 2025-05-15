"use client"
import { useState, useContext, useEffect } from "react"
import { PublicAddressesContext, Vault, VaultContext, VaultContextType } from "@/contexts"
import useMounted from "@/hooks/useMounted"
import { useVault } from "@/hooks/useVault"


export const VaultProvider = ({ children }: { children: React.ReactNode }) => {
  const isMounted = useMounted();
  const publicAddresses = useContext(PublicAddressesContext);
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

  useEffect(() => {
    if (publicAddresses?.vaults) {
      console.log("publicAddresses.vaults", publicAddresses.vaults);
      setSelectedVault(publicAddresses.vaults[0]);
    }
  }, [publicAddresses]);

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