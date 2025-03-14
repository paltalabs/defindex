import axios from 'axios'


const contract_addresses = {
  testnet: {
    defindex_factory: "CAL6NQYPPUU424WHNJ255IKYSBXI4CPYCKJGYHNKDUM4T37P3PHKWM4I",
    hodl_strategy: "CC5P6K3LJ7RSL3BVYQQNUBT2S4GK7IOKI5DF6FDGZCOWRSDXJUYVNOH3",
    fixed_apr_strategy: "CCSOD3FEEA4FYRZP3HFICIVXKH4URJ5TAELHB6A76QJ6ZW73YC6V6RCL",
    blend_strategy: "CAXHJUBBHOFF3H2HB6OBIMMIXTQRCG5COPNZNB7LIG354OKDQGU5PYDC"
  },
  mainnet: {
    defindex_factory: undefined,
    hodl_strategy: undefined,
    fixed_apr_strategy: undefined,
    blend_strategy: undefined
  }
}
export const configFile = async (network: string) => {
  /* if(network != 'testnet' && network != 'mainnet' && network!= 'standalone') throw new Error(`Invalid network: ${network}. It should be testnet, mainnet or standalone`)
  const url = `https://raw.githubusercontent.com/paltalabs/defindex/refs/heads/main/public/${network}.contracts.json`
  const data = await axios.get(url)
  if(data.status === 200) return data.data
  if(data.status === 404) throw new Error(`Deployment not found for network: ${network}`) */
  if (network != 'testnet' && network != 'mainnet' && network != 'standalone') throw new Error(`Invalid network: ${network}.`)
  if (network === 'testnet') return contract_addresses.testnet
  if (network === 'mainnet') return contract_addresses.mainnet
  if (network === 'standalone') return contract_addresses.testnet
  return {}
}