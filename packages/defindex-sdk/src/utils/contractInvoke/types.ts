import * as StellarSdk from '@stellar/stellar-sdk'
import { SorobanRpc } from '@stellar/stellar-sdk'


export interface NetworkDetails {
    network: string
    networkUrl: string
    networkPassphrase: string
    sorobanRpcUrl?: string
}

export type Connector = {
    id: string
    name: string
    shortName?: string
    iconUrl: string | (() => Promise<string>)
    iconBackground: string
    installed?: boolean
    downloadUrls?: {
      android?: string
      ios?: string
      browserExtension?: string
      qrCode?: string
    }
    isConnected: () => boolean
    getNetworkDetails: () => Promise<NetworkDetails>
    getPublicKey: () => Promise<string>
    signTransaction: (
      xdr: string,
      opts?: {
        network?: string
        networkPassphrase?: string
        accountToSign?: string
      }
    ) => Promise<string>
}
  
export interface WalletChain {
    id: string
    name?: string
    networkPassphrase: string
    iconBackground?: string
    iconUrl?: string | null
    unsupported?: boolean
    network: string
    networkUrl: string
    sorobanRpcUrl?: string
}

// Type for top level contract registry
export type ContractDeploymentInfo = {
    contractId: string,
    networkPassphrase: string,
    contractAddress: string
  }

/**
 * Interface for the Soroban context.
 */
export interface SorobanContextType {
    // Indicates whether autoconnect is enabled
    autoconnect?: boolean
    // Name of the Soroban application
    appName?: string
    // List of chains
    chains: WalletChain[]
    // List of connectors
    connectors: Connector[]
    // Active chain
    activeChain?: WalletChain
    // Connected wallet address
    address?: string
    // Active connector
    activeConnector?: Connector
    // Soroban RPC server
    server?: SorobanRpc.Server
    // Stellar Horizon server
    serverHorizon?: StellarSdk.Horizon.Server
    // Function to connect to a wallet
    connect: () => Promise<void>
    // Function to disconnect from a wallet
    disconnect: () => Promise<void>
    // Function to set the active chain
    setActiveChain?: (chain: WalletChain) => void
    // Function to set the active connector and connect
    setActiveConnectorAndConnect?: (connector: Connector) => void
    // List of contract deployments
    deployments?: ContractDeploymentInfo[]
  }
  