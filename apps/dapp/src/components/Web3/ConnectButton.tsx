import React from "react"
import { Button } from '@chakra-ui/react'
import { useSorobanReact } from 'stellar-react'

import { shortenAddress } from "@/helpers/address"



export const ConnectButton = () => {
  const { address, kit, disconnect, connect } = useSorobanReact()

  const handleConnect = async () => {
    if (address) {
      await disconnect()
    } else {
      await connect()
    }
  }
  return (
    <Button
      onClick={handleConnect}
      px={4}
      rounded={18}
    >
      {address ? `Disconnect ${shortenAddress(address)}` : 'Connect Wallet'}
    </Button>
  )
}

export default ConnectButton
