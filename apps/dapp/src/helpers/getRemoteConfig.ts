import { configFile } from '@/constants/constants';
const isLocal = process.env.NEXT_PUBLIC_IS_LOCAL
const localDeployment = require('../../../contracts/.soroban/deployments.json')
export const getRemoteConfig = async (network: string) => {
  if(isLocal === 'true') {
    return localDeployment
  } else if (isLocal === 'false') {
    const deployments = await configFile(network)
    return deployments
  }
}