'use client'
import { Box, Flex, Link, Image, HStack } from '@chakra-ui/react'
import React, { useEffect } from 'react'
import ConnectButton from '../web3/ConnectWallet'
import './NavBar.css'
import { navBarHeight } from '../ui/Common'
import { useSorobanReact, WalletNetwork } from 'stellar-react'
import { usePublicAddresses } from '@/hooks/usePublicAddresses'
import { getNetworkName } from '@/helpers/networkName'


function NavBar() {
  const sorobanContext = useSorobanReact();
  const [vaultAddress, setVaultAddress] = React.useState<string | null>(null);
  const publicAddresses = usePublicAddresses(getNetworkName(sorobanContext.activeNetwork)).data;
  const links = [
    { name: 'Launch Vault', href: '/' },
    { name: 'Paltalabs Vault', href: `/vault/${vaultAddress}` },
  ]

  useEffect(() => {
    if (publicAddresses) {
      const vaultAddress = sorobanContext.activeNetwork === WalletNetwork.PUBLIC ? publicAddresses['usdc_palta_vault'] : publicAddresses['usdc_blend_vault'];
      if (vaultAddress) {
        setVaultAddress(vaultAddress);
      }
    }
  }, [publicAddresses])
  return (
    <Box>
      <Flex alignItems={'center'} className='nav-bar'>
        <Image src={'../defindex_logo.svg'} alt="Logo" width={'6rem'} height={navBarHeight} fit={'contain'} />
        <Box alignItems={'center'} display={'flex'}>
          {links.map((link) => (
            <Link key={link.name} as="a" href={link.href} mx={2} className='nav-link'>
              {link.name}
            </Link>
          ))}
          <ConnectButton />
        </Box>
      </Flex>
    </Box>
  )
}

export default NavBar
