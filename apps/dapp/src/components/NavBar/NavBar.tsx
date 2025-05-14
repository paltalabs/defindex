'use client'
import { Box, Flex, Image, Link } from '@chakra-ui/react'
import { navBarHeight } from '../ui/Common'
import ConnectButton from '../web3/ConnectWallet'
import './NavBar.css'


const links = [
  { name: 'Docs', href: 'https://docs.defindex.io/' },
  { name: 'Defindex Home', href: 'https://defindex.io/' },
]
function NavBar() {
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
