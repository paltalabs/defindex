import { Keypair } from '@stellar/stellar-sdk';

/**
 * Creates n Stellar keypairs and logs them to the console
 * @param n - Number of keypairs to generate
 */
function createAccounts(n: number): void {
  if (n <= 0) {
    console.error('Number of accounts must be greater than 0');
    return;
  }

  console.log(`Generating ${n} Stellar keypairs...\n`);

  for (let i = 0; i < n; i++) {
    const keypair = Keypair.random();
    console.log(`Account #${i + 1}:`);
    console.log(`  Public Key: ${keypair.publicKey()}`);
    console.log(`  Secret Key: ${keypair.secret()}`);
    console.log('');
  }

  console.log('All keypairs generated successfully.');
}

// Example usage: Generate 5 keypairs
const numberOfAccounts = 4;
createAccounts(numberOfAccounts);

