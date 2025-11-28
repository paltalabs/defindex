"use client";

import {
  allowAllModules,
  FREIGHTER_ID,
  ISupportedWallet,
  StellarWalletsKit,
  WalletNetwork,
} from "@creit.tech/stellar-wallets-kit";
import { LedgerModule } from "@creit.tech/stellar-wallets-kit/modules/ledger.module";
import {
  WalletConnectAllowedMethods,
  WalletConnectModule,
} from "@creit.tech/stellar-wallets-kit/modules/walletconnect.module";
import {
  createContext,
  ReactNode,
  useContext,
  useEffect,
  useRef,
  useState,
} from "react";

// Network configuration
const NETWORKS = {
  mainnet: {
    network: WalletNetwork.PUBLIC,
    networkPassphrase: "Public Global Stellar Network ; September 2015",
    sorobanRpcUrl: "https://soroban-rpc.creit.tech/",
    horizonRpcUrl: "https://horizon.stellar.org",
  },
  testnet: {
    network: WalletNetwork.TESTNET,
    networkPassphrase: "Test SDF Network ; September 2015",
    sorobanRpcUrl: "https://soroban-testnet.stellar.org/",
    horizonRpcUrl: "https://horizon-testnet.stellar.org",
  },
} as const;

type NetworkType = keyof typeof NETWORKS;

type NetworkConfig = typeof NETWORKS[NetworkType];

interface UserContextProps {
  address: string | null;
  setAddress: (address: string | null) => void;
  kit: StellarWalletsKit | null;
  connectWallet: () => Promise<void>;
  disconnect: () => void;
  signTransaction: (xdr: string, userAddress: string) => Promise<string>;
  selectedWallet: ISupportedWallet | null;
  activeNetwork: NetworkType;
  setActiveNetwork: (network: NetworkType) => void;
  networkConfig: NetworkConfig;
}

interface UserProviderProps {
  children: ReactNode;
}

export const UserContext = createContext<UserContextProps>({
  address: null,
  setAddress: () => {},
  kit: null,
  connectWallet: async () => {},
  disconnect: () => {},
  signTransaction: async () => "",
  selectedWallet: null,
  activeNetwork: "mainnet",
  setActiveNetwork: () => {},
  networkConfig: NETWORKS.mainnet,
});

export const UserProvider = ({ children }: UserProviderProps) => {
  const [address, setAddress] = useState<string | null>(null);
  const [kit, setKit] = useState<StellarWalletsKit | null>(null);
  const [selectedWallet, setSelectedWallet] = useState<ISupportedWallet | null>(null);
  const [activeNetwork, setActiveNetwork] = useState<NetworkType>("mainnet");
  const kitRef = useRef<StellarWalletsKit | null>(null);

  const networkConfig = NETWORKS[activeNetwork];

  useEffect(() => {
    if (typeof window !== "undefined") {
      try {
        const walletKit = new StellarWalletsKit({
          network: networkConfig.network,
          selectedWalletId: FREIGHTER_ID,
          modules: [
            ...allowAllModules(),
            new LedgerModule(),
            new WalletConnectModule({
              url: typeof window !== "undefined" ? window.location.origin : "https://app.defindex.io",
              projectId: "4ee1d28f1fe3c70aa8ebc4677e623e1d",
              method: WalletConnectAllowedMethods.SIGN,
              description: "DeFindex - DeFi Yield Infrastructure",
              name: "DeFindex",
              icons: ["/favicon.ico"],
              network: networkConfig.network,
            }),
          ],
        });
        kitRef.current = walletKit;
        setKit(walletKit);
      } catch (error) {
        console.error("Failed to initialize wallet kit:", error);
      }
    }
  }, [networkConfig.network]);

  const connectWallet = async () => {
    if (!kit) return;

    await kit.openModal({
      onWalletSelected: async (option: ISupportedWallet) => {
        kit.setWallet(option.id);
        setSelectedWallet(option);
        const { address } = await kit.getAddress();
        setAddress(address);
      },
    });
  };

  const disconnect = () => {
    if (kit) {
      kit.disconnect();
      setAddress(null);
      setSelectedWallet(null);
    }
  };

  const signTransaction = async (xdr: string, userAddress: string): Promise<string> => {
    if (!kit) throw new Error("Wallet kit not initialized");

    const { signedTxXdr } = await kit.signTransaction(xdr, {
      address: userAddress,
      networkPassphrase: networkConfig.networkPassphrase,
    });

    if (!signedTxXdr) throw new Error("Failed to sign transaction");
    return signedTxXdr;
  };

  const handleSetActiveNetwork = (network: NetworkType) => {
    setActiveNetwork(network);
    // Disconnect wallet when switching networks
    if (address) {
      disconnect();
    }
  };

  return (
    <UserContext.Provider
      value={{
        setAddress,
        address,
        kit,
        connectWallet,
        disconnect,
        signTransaction,
        selectedWallet,
        activeNetwork,
        setActiveNetwork: handleSetActiveNetwork,
        networkConfig,
      }}
    >
      {children}
    </UserContext.Provider>
  );
};

export const useUser = () => useContext(UserContext);
