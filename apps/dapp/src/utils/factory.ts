import { getRemoteConfig } from "@/helpers/getRemoteConfig";

export async function fetchFactoryAddress(network: string): Promise<string> {
    if (network !== "testnet" && network !== "mainnet") {
        throw new Error(`Invalid network: ${network}. It should be testnet or mainnet`);
    }
    try {
        const remoteConfig: any = await getRemoteConfig(network);
        const factoryAddress = remoteConfig.defindex_factory;
        return factoryAddress;

    } catch (error) {
        if (error instanceof Error && error.message === `Deployment not found for network: ${network}`) {
            throw error;
        }
        else {
            throw new Error(`Failed to fetch factory address: ${error}`);
        }

    }
}
