'use client'
import { Box, Flex, Link, Image } from '@chakra-ui/react'
import React, { useContext } from 'react'
import ConnectButton from '../web3/ConnectWallet'
import './NavBar.css'
import { navBarHeight } from '../ui/Common'
import { PublicAddressesContext } from '@/contexts'


function NavBar() {
  const publicAddresses = useContext(PublicAddressesContext);
  const links = [
    { name: 'Launch Vault', href: '/' },
    { name: 'Paltalabs Vault', href: `/vault/${publicAddresses?.vaults[0]?.address}` },
  ]

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
