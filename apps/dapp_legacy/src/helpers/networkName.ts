import { Networks } from "@stellar/stellar-sdk"

export const getNetworkName = (networkPassphrase: string): string => {
  switch (networkPassphrase) {
    case Networks.TESTNET:
      return 'testnet'
    case Networks.PUBLIC:
      return 'mainnet'
    case Networks.FUTURENET:
      return 'futurenet'
    case Networks.SANDBOX:
      return 'sandbox'
    case Networks.STANDALONE:
      return 'standalone'
    default:
      return ''
  }
}