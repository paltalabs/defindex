import { configFile } from '@/constants/constants';
//import localDeployment from '../../../contracts/.soroban/testnet.contracts.json'

const isLocal = process.env.NEXT_PUBLIC_IS_LOCAL

export const getRemoteConfig = async (network: string) => {
  if (isLocal === 'false' || isLocal === undefined) {
    const deployments = await configFile(network)
    return deployments
  }
  /* else if(isLocal === 'true') {
    return localDeployment.ids
  } */
}