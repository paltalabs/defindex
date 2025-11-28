import { Vault, VaultContext } from '@/contexts'
import { isValidAddress } from '@/helpers/address'
import React, { useContext, useEffect } from 'react'
import { FormField } from '../ui/CustomInputFields'
import { VaultConfigSection } from './VaultConfigSection'

export function ManagerConfig() {
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
