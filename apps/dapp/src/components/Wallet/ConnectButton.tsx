import { Button } from '@chakra-ui/react'
import { useSorobanReact } from '@soroban-react/core'
import { ConnectWalletModal } from './ConnectWalletModal'
import { useState } from 'react'

export const ConnectButton = () => {
  const { address } = useSorobanReact()
  const [isModalOpen, setIsModalOpen] = useState<boolean>(false)
  const handleClick = () => {
    setIsModalOpen(true)
  }
  const handleClose = () => {
    setIsModalOpen(false)
  }
  return (
    <>

      <ConnectWalletModal isOpen={isModalOpen} onClose={handleClose} />
      <Button sx={{ mx: 4, px: 6 }} colorScheme='green' onClick={handleClick} rounded={18}>
        {address ? 'Disconnect' : 'Connect'}
      </Button>
    </>
  )
}

export default ConnectButton
