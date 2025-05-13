'use client'
import { Box, Container, Stack } from '@chakra-ui/react'
import React, { useContext, useEffect } from 'react'
import { VaultDetailsBanner } from './VaultDetails'
import VaultInteraction from './VaultInteraction'
import { VaultContext } from '@/contexts'
import { useVault } from '@/hooks/useVault'
import { useSorobanReact } from 'stellar-react'

function VaultDetailsPage({ vaultAddress }: { vaultAddress: string }) {
  const sorobanContext = useSorobanReact();
  const vaultContext = useContext(VaultContext)
  const vault = useVault();

  const getVaultInfo = async () => {
    const vaultInfo = await vault.getVaultInfo(vaultAddress);
    return vaultInfo;
  }

  useEffect(() => {
    const vaultInfo = getVaultInfo()
    vaultInfo.then((vaultInfo) => {
      if (vaultContext && vaultInfo) {
        vaultContext.setSelectedVault(vaultInfo);
      }
    })
  }, [vaultAddress, sorobanContext.address, sorobanContext.activeNetwork])
  return (
    <Stack gap={4}>
      <VaultDetailsBanner vaultAddress={vaultAddress} />
      <VaultInteraction vaultAddress={vaultAddress} />
    </Stack>
  )
}

export default VaultDetailsPage
