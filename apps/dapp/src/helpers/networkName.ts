import { WalletNetwork } from "stellar-react"

export const getNetworkName = (networkPassphrase?: WalletNetwork): string => {
  if(!networkPassphrase) {
    return 'testnet'
  }
  switch (networkPassphrase) {
    case WalletNetwork.TESTNET:
      return 'testnet'
    case WalletNetwork.PUBLIC:
      return 'mainnet'
    case WalletNetwork.FUTURENET:
      return 'futurenet'
    case WalletNetwork.SANDBOX:
      return 'sandbox'
    case WalletNetwork.STANDALONE:
      return 'standalone'
    default:
      return 'testnet'
  }
}