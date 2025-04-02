

const contract_addresses = {
  testnet: {
    defindex_factory: "CADQLLHP7JIR45CHSANRURA2H2QQAT7ROLPS5HV7XIS6KL5QKIZ3JCB5",
    fixed_apr_strategy: "CDP6HIUYERS2CGA2XWO647H5BBQJS52QHPBFOI5R2WBGORS7RDL63SBF",
    blend0_strategy: "CAASG55NPLKJ6PHSUKU2RBD3UFOHHHPRHZE3LL5PXDGIRZNIV53ESD3Z",
    blend1_strategy: "CC5F273LMR5RGOL2EMZMJKRZLQLT4FC7SCWXI3YYVBLEII7QCHPE4UZP",
    hodl_strategy: "CAMK34KRQTBGV3WQOKRXO5WLPG6CNYW62V65ZQ37SGPXPIMYGNXFWQEE",
  },
  mainnet: {
    defindex_factory: undefined,
    hodl_strategy: undefined,
    fixed_apr_strategy: undefined,
    blend_strategy: undefined
  }
}

export const soroswapRouter = {
  testnet: "CACIQ6HWPBEMPQYKRRAZSM6ZQORTBTS7DNXCRTI6NQYMUP2BHOXTBUVD",
  mainnet: "CAG5LRYQ5JVEUI5TEID72EYOVX44TTUJT5BQR2J6J77FH65PCCFAJDDH"
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