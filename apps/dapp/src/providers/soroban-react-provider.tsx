import useMounted from '@/hooks/useMounted';
import { NetworkDetails, SorobanReactProvider, WalletNetwork } from 'stellar-react';

const mainnetNetworkDetails: NetworkDetails = {
  network: WalletNetwork.PUBLIC,
  sorobanRpcUrl: 'https://soroban-rpc.creit.tech/',
  horizonRpcUrl: 'https://horizon.stellar.org'
}

const testnetNetworkDetails: NetworkDetails = {
  network: WalletNetwork.TESTNET,
  sorobanRpcUrl: 'https://soroban-testnet.stellar.org/',
  horizonRpcUrl: 'https://horizon-testnet.stellar.org'
}

export default function MySorobanReactProvider({ children }: { children: React.ReactNode }) {
  const mounted = useMounted();
  if (!mounted) return null;
  return (
    <SorobanReactProvider
      appName={"Example Stellar App"}
      allowedNetworkDetails={[mainnetNetworkDetails, testnetNetworkDetails]}
      activeNetwork={WalletNetwork.TESTNET}
    >
      {children}
    </SorobanReactProvider>
  )
}