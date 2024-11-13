import { Vault, SorobanNetwork } from '../src/Vault';
import { SorobanRpc, Networks, Keypair } from '@stellar/stellar-sdk';

async function createSorobanContext() {
    const server = new SorobanRpc.Server("https://soroban-testnet.stellar.org", {
        allowHttp: true
    });

    const secretKey = 'SC352W6PEHWSHYKP5IYO3HWAEVGLTVLZW5WE3UXPWSGKBST5K6DKRT7F';
    const keypair = Keypair.fromSecret(secretKey);

    return {
        server,
        address: keypair.publicKey(),
        activeChain: {
            networkPassphrase: Networks.TESTNET,
            network: "TESTNET",
            id: "testnet",
            networkUrl: "https://soroban-testnet.stellar.org"
        },
        activeConnector: undefined,
        chains: [],
        connectors: [],
        connect: async () => { },
        disconnect: async () => { }
    };
}

async function vaultExample() {
    const sorobanContext = await createSorobanContext();
    const secretKey = 'SC352W6PEHWSHYKP5IYO3HWAEVGLTVLZW5WE3UXPWSGKBST5K6DKRT7F';

    // Initialize the Vault
    const vault = new Vault({
        network: SorobanNetwork.TESTNET,
        contractId: 'CAJVX4P2XDRVIGZ7GRHRSGBE6B23L4FQLT5SGUBHU7NT2IER7R2WIURG' // Replace with your contract ID
    });

    try {
        const accountAddress = sorobanContext.address;
        console.log('Using account:', accountAddress);

        // Check initial balance
        const initialBalance = await vault.balance(accountAddress, sorobanContext);
        console.log('Initial balance:', initialBalance);

        // Deposit 100 tokens
        const depositTx = await vault.deposit(
            accountAddress,
            100,
            true,
            sorobanContext,
            secretKey
        );
        console.log('Deposit transaction hash:', depositTx);

        // Wait a few seconds for the transaction to be processed
        await new Promise(resolve => setTimeout(resolve, 5000));

        // Check updated balance
        const updatedBalance = await vault.balance(accountAddress, sorobanContext);
        console.log('Updated balance:', updatedBalance);

        // Withdraw 50 tokens
        const withdrawTx = await vault.withdraw(
            accountAddress,
            50,
            true,
            sorobanContext,
            secretKey
        );
        console.log('Withdraw transaction hash:', withdrawTx);

        // Wait a few seconds for the transaction to be processed
        await new Promise(resolve => setTimeout(resolve, 5000));

        // Check final balance
        const finalBalance = await vault.balance(accountAddress, sorobanContext);
        console.log('Final balance:', finalBalance);

    } catch (error) {
        console.error('Error:', error);
    }
}

// Execute the example
vaultExample()
    .then(() => console.log('Example completed'))
    .catch(error => console.error('Example failed:', error)); 