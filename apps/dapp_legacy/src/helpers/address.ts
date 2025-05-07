import { StrKey } from '@stellar/stellar-sdk';
export function shortenAddress(address: string): string {
  if (address.length !== 56) {
    throw new Error('Invalid address length');
  }

  const firstThree = address.slice(0, 3);
  const lastThree = address.slice(-3);

  return `${firstThree}...${lastThree}`;
}
export const isValidAddress = (address: string) => {
  if (StrKey.isValidEd25519PublicKey(address) || StrKey.isValidMed25519PublicKey(address) || StrKey.isValidContract(address)) {
    return true
  } else {
    return false
  }
}