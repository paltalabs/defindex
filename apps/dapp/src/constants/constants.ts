import axios from 'axios'


const contract_addresses = {
  testnet: {
    defindex_factory: "CBYZ5BCEFAIUZSJIDW3M5PJJ2YUVN6HUI372OZCPKJXYGGUJFJQOY7IJ",
    hodl_strategy: "CBCVWGWQEZZGMZZMYFGD262YX5MY4CU4ZSENPY4XIM4ICH3WRBPENFLC",
    fixed_apr_strategy: "CCATEYC2XAOYFQKIQLZLJLFMLQQ56FAA3IHCPUQ2COCMGSKMLEPOQVEH",
    blend_strategy: "CDKKD3T65UAHXXBXUQPKSF2JMPFYU5XCJP2R26HWH5ZHGSHINLZL3P72"
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