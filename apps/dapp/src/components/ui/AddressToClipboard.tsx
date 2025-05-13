'use client'
import { HStack, IconButton, Text } from '@chakra-ui/react';
import React from 'react'
import { FaCopy, FaExternalLinkAlt } from "react-icons/fa";
import './AddressToClipboard.css'
import { useSorobanReact, WalletNetwork } from 'stellar-react';
import { toaster } from './toaster';

function AddressToClipboard({ label, vaultAddress }: { label: string, vaultAddress: string }) {
  const sorobanContext = useSorobanReact();

  const copyToClipboard = () => {
    navigator.clipboard.writeText(vaultAddress).then(() => {
      toaster.create({
        title: 'Copied to clipboard',
        description: `Address ${vaultAddress} copied to clipboard`,
        type: 'success',
        duration: 2000,
      });
    }).catch((error) => {
      toaster.error({
        title: 'Error copying to clipboard',
        description: `Failed to copy address ${vaultAddress} to clipboard: ${error.message}`,
        duration: 2000,
      });
    });
  };

  const openInExplorer = () => {
    const networkName = sorobanContext.activeNetwork === WalletNetwork.TESTNET ? 'testnet' : 'public';
    const url = `https://stellar.expert/explorer/${networkName}/contract/${vaultAddress}`;
    window.open(url, '_blank');
  }

  return (
    <HStack className='address-to-clipboard' alignItems={'baseline'} justifyContent={'start'} gap={3} w={'full'}>
      <Text lineClamp={1} fontSize={'sm'} className='label'>{label}:</Text>
      <Text truncate fontSize={'xs'} className='value'>{vaultAddress}</Text>
      <IconButton
        variant={'ghost'}
        aria-label="Copy address to clipboard"
        className='icon-button'
        onClick={copyToClipboard}
        size={'xs'}
      >
        <FaCopy />
      </IconButton>
      <IconButton
        variant={'ghost'}
        aria-label="Open address in explorer"
        className='icon-button'
        onClick={openInExplorer}
        size={'xs'}
      >
        <FaExternalLinkAlt />
      </IconButton>
    </HStack>
  )
}

export default AddressToClipboard
