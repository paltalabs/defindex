'use client'
import { useUser } from '@/contexts/UserContext';
import { Box, Button, Flex, Image, Link, MenuContent, MenuItem, MenuPositioner, MenuRoot, MenuTrigger, Portal } from '@chakra-ui/react';
import { RiArrowDropDownLine } from "react-icons/ri";
import { navBarHeight } from '../ui/Common';
import ConnectButton from '../web3/ConnectWallet';
import './NavBar.css';

const links = [
  { name: 'Docs', href: 'https://docs.defindex.io/' },
  { name: 'Defindex Home', href: 'https://defindex.io/' },
]

function NetworkSelector() {
  const { activeNetwork, setActiveNetwork } = useUser();

  const networks = [
    { id: 'mainnet', name: 'Mainnet' },
    { id: 'testnet', name: 'Testnet' },
  ] as const;

  return (
    <MenuRoot positioning={{ placement: 'bottom' }}>
      <MenuTrigger asChild>
        <Button
          mx={2}
          px={4}
          rounded={15}
          className="network-selector-btn flex"
        >
          {activeNetwork === 'mainnet' ? 'Mainnet' : 'Testnet'}
          <span style={{ marginLeft: '0px' }}>
            <RiArrowDropDownLine/>
          </span>
        </Button>
      </MenuTrigger>
      <Portal>
        <MenuPositioner>
          <MenuContent className="network-menu-content">
            {networks.map((network) => (
              <MenuItem
                key={network.id}
                value={network.id}
                onClick={() => setActiveNetwork(network.id)}
                className={`network-item ${activeNetwork === network.id ? 'network-item-active' : ''} `}
              >
                {network.name}
              </MenuItem>
            ))}
          </MenuContent>
        </MenuPositioner>
      </Portal>
    </MenuRoot>
  );
}

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
          <NetworkSelector />
          <ConnectButton />
        </Box>
      </Flex>
    </Box>
  )
}

export default NavBar
