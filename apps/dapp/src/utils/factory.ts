
// // Usage
// fetchFactoryAddress()
//     .then((factoryAddress) => {
//         console.log('Factory Address:', factoryAddress);
//     })
//     .catch((error) => {
//         console.error('Error:', error);
//     });

export async function fetchFactoryAddress(network: string): Promise<string> {
    if (network !== "testnet" && network !== "mainnet") {
        throw new Error(`Invalid network: ${network}. It should be testnet or mainnet`);
    }
  
    const url = `https://raw.githubusercontent.com/paltalabs/defindex/refs/heads/main/public/${network}.contracts.json`;
    try {
        const response = await fetch(url);
        if (response.status === 404) {
            throw new Error(`Deployment not found for network: ${network}`);
        }
        const data = await response.json();
        const factoryAddress = data.ids.defindex_factory;
        return factoryAddress;

    } catch (error) {
        if (error instanceof Error && error.message === `Deployment not found for network: ${network}`) {
            throw error;
        }
        else {

            throw new Error(`Failed to fetch factory address: ${error}`);
        }

    }
}
