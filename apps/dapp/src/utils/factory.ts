
// // Usage
// fetchFactoryAddress()
//     .then((factoryAddress) => {
//         console.log('Factory Address:', factoryAddress);
//     })
//     .catch((error) => {
//         console.error('Error:', error);
//     });
export async function fetchFactoryAddress(): Promise<string> {
    const url = 'https://raw.githubusercontent.com/paltalabs/defindex/refs/heads/main/public/testnet.contracts.json';
    const response = await fetch(url);
    const data = await response.json();
    const factoryAddress = data.ids.defindex_factory;
    return factoryAddress;
}
