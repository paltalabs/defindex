import React from "react"
import { Button } from '@chakra-ui/react'
import { useUser } from "@/contexts/UserContext"
import { shortenAddress } from "@/helpers/address"
import './ConnectWallet.css'

export const ConnectButton = () => {
  const { address, disconnect, connectWallet } = useUser();

  const handleConnect = async () => {
    if (address) {
      disconnect();
    } else {
      await connectWallet();
    }
  }
  return (
    <Button
      onClick={handleConnect}
      px={4}
      rounded={15}
      className="custom-button"
    >
      {address ? `Disconnect ${shortenAddress(address)}` : 'Connect Wallet'}
    </Button>
  )
}

export default ConnectButton;