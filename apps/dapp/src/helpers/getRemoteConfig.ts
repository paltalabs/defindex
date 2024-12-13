import axios from 'axios'
export const getRemoteConfig = async (network: string) => {
  try {
    "https://raw.githubusercontent.com/ydag/test-defindex/refs/heads/main/testnet.contracts.json"
    // const {data: remoteConfig} = await axios.get(`https://raw.githubusercontent.com/paltalabs/defindex/refs/heads/main/public/${network}.contracts.json`)
    const { data: remoteConfig } = await axios.get("https://raw.githubusercontent.com/ydag/test-defindex/refs/heads/main/testnet.contracts.json")
    return remoteConfig
  } catch (error) {
    console.error(error)
    return {}
  }
}