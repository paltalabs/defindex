import { Button } from '@chakra-ui/react'
import { useSorobanReact } from '@soroban-react/core'
import React from 'react'

export const ConnectButton = () => {
  const { address, disconnect, setActiveConnectorAndConnect, connect, connectors } = useSorobanReact()
  const handleClick = () => {
    if (address) {
      disconnect()
    } else {
      connect()
    }
  }
  return (
    <>
      <Button colorScheme='green' onClick={handleClick}>
        {address ? 'Disconnect' : 'Connect'}
      </Button>
    </>
  )
}

export default ConnectButton
