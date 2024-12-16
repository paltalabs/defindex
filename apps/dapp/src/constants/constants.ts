import axios from 'axios'
const baseURL = 'https://raw.githubusercontent.com/paltalabs/defindex/refs/heads/main/public/'
const suffix = '.contracts.json'
export const configFile = async (network:string)=>{
  if(network != 'testnet' && network != 'mainnet' && network!= 'standalone') throw new Error(`Invalid network: ${network}. It should be testnet, mainnet or standalone`)
  const url = baseURL + network + suffix
  const data = await axios.get(url)
  if(data.status === 200) return data.data
  if(data.status === 404) throw new Error(`Deployment not found for network: ${network}`)
}