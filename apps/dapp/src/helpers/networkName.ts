export type NetworkType = 'mainnet' | 'testnet';

export const getNetworkName = (network?: NetworkType): NetworkType => {
  if (!network) {
    return 'mainnet';
  }
  return network;
}