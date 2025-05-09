'use client'
import { Box, Flex, Link, Image, HStack } from '@chakra-ui/react'
import React from 'react'
import ConnectButton from '../web3/ConnectWallet'
import './NavBar.css'


const links = [
  { name: 'Defindex Vaults', href: '/' },
  { name: 'Portfolio', href: '/' },
  { name: 'Defindex home', href: '/' },
]
function NavBar() {
  return (
    <Box>
      <Flex alignItems={'center'} className='nav-bar'>
        <Image src={'../defindex_logo.svg'} alt="Logo" width={'6rem'} height={'2rem'} fit={'contain'} />
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
