import { Vault, VaultContext } from '@/contexts'
import { Checkbox, Flex, Text } from '@chakra-ui/react'
import React, { useContext, useEffect } from 'react'
import { FormField } from '../ui/CustomInputFields'
import { SelectAssets } from './SelectAssets'
import { VaultConfigSection } from './VaultConfigSection'

export function VaultConfig() {
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

      <Flex direction="column" align="center" justifyContent="center" alignSelf="center" gap={1} w="120px" flexShrink={0}>
        <Text fontSize="xs" color={upgradable ? '#D3FFB4' : 'gray.400'} whiteSpace="nowrap">
          {upgradable ? 'Upgradable' : 'Non-Upgradable'}
        </Text>
        <Checkbox.Root
          checked={upgradable}
          colorScheme={'green'}
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
            <Checkbox.Indicator className='checkbox' strokeWidth={1} />
          </Checkbox.Control>
        </Checkbox.Root>
      </Flex>
    </VaultConfigSection>
  );
}
