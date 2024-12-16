import axios from 'axios'
import localDeployment from '../../../contracts/.soroban/testnet.contracts.json';
import { configFile } from '@/constants/constants';
const isLocal = process.env.NEXT_PUBLIC_IS_LOCAL
export const getRemoteConfig = async (network: string) => {
  if(isLocal === 'true') {
    return localDeployment
  } else if (isLocal === 'false') {
    const deployments = await configFile(network)
    return deployments
  }
}