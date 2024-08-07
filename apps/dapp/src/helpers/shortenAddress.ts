export function shortenAddress(address: string): string {
  if (address.length !== 56) {
    throw new Error('Invalid address length');
  }

  const firstThree = address.slice(0, 3);
  const lastThree = address.slice(-3);

  return `${firstThree}...${lastThree}`;
}