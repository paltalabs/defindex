import axios from 'axios'


const contract_addresses = {
  testnet: {
    defindex_factory: "CAJJAIJT6E7GMJKFA66RRI7KUNNR22TPNLNUFCNANGCFXJ54RYPVPPJT",
    hodl_strategy: "CDTSVTAI4BXYIEZ66F2TLZ337OLW5R5P4ONWMHQY5XTOTBZURZEDZ64N",
    fixed_apr_strategy: "CDR6K2L2UN3SZLBOUCJCVNKKKGC4F5DHGQ6QIPVBXU3UTXTTGWWEZ2H3",
    blend_strategy: "CCFT4EVTUSYUNO7CYJHC3R2F5ZBTXW7CMJW5Z2PGVYCHWNDG4RS35YZ5"
  },
  mainnet: {
    defindex_factory: undefined,
    hodl_strategy: undefined,
    fixed_apr_strategy: undefined,
    blend_strategy: undefined
  }
}
export const configFile = async (network:string)=>{
  /* if(network != 'testnet' && network != 'mainnet' && network!= 'standalone') throw new Error(`Invalid network: ${network}. It should be testnet, mainnet or standalone`)
  const url = `https://raw.githubusercontent.com/paltalabs/defindex/refs/heads/main/public/${network}.contracts.json`
  const data = await axios.get(url)
  if(data.status === 200) return data.data
  if(data.status === 404) throw new Error(`Deployment not found for network: ${network}`) */
  if(network != 'testnet' && network != 'mainnet' && network != 'standalone') throw new Error(`Invalid network: ${network}.`)
  if(network === 'testnet') return contract_addresses.testnet
  if(network === 'mainnet') return contract_addresses.mainnet
  if(network === 'standalone') return contract_addresses.testnet
  return {}
}