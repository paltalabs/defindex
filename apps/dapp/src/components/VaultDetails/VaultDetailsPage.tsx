'use client'
import { Stack } from '@chakra-ui/react'
import React, { useContext } from 'react'
import { VaultDetailsBanner } from './VaultDetails'
import VaultInteraction from './VaultInteraction'
import { VaultContext } from '@/contexts'


function VaultDetailsPage({ }: {}) {
  const vaultContext = useContext(VaultContext);
  const selectedVault = vaultContext?.selectedVault;
  if (!selectedVault) return null;
  return (
    <Stack gap={4}>
      <VaultDetailsBanner vault={selectedVault} />
      <VaultInteraction vault={selectedVault} />
    </Stack>
  )
}

export default VaultDetailsPage
