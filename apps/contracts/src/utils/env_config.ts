import { Horizon, Keypair, rpc } from "@stellar/stellar-sdk";
import dotenv from "dotenv";
import * as fs from "fs";
import path from "path";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
dotenv.config({ path: path.join(__dirname, "../../.env") });

interface NetworkConfig {
  network: string;
  friendbot_url?: string;
  horizon_rpc_url: string;
  soroban_rpc_url?: string;
  soroban_network_passphrase: string;
  blend_keeper: string;
  defindex_factory_admin: string;
  defindex_fee_receiver: string;
  defindex_fee: string;
  vault_fee_receiver: string;
  vault_manager: string;
  vault_emergency_manager: string;
  vault_rebalance_manager: string;
  vault_name: string;
  vault_symbol: string;
}

interface Config {
  previewHash: string;
  quickstartHash: string;
  networkConfig: NetworkConfig[];
}

export class EnvConfig {
  rpc: rpc.Server;
  horizonRpc: Horizon.Server;
  passphrase: string;
  friendbot: string | undefined;
  admin: Keypair;
  blendKeeper: string;
  defindexFeeReceiver: string;
  defindexFactoryAdmin: string;
  defindexFee: string;
  vaultFeeReceiver: string;
  vaultManager: string;
  vaultEmergencyManager: string;
  vaultRebalanceManager: string;
  vaultName: string;
  vaultSymbol: string;

  constructor(
    rpc: rpc.Server,
    horizonRpc: Horizon.Server,
    passphrase: string,
    friendbot: string | undefined,
    admin: Keypair,
    blendKeeper: string,
    defindexFeeReceiver: string,
    defindexFactoryAdmin: string,
    defindexFee: string,
    vaultFeeReceiver: string,
    vaultManager: string,
    vaultEmergencyManager: string,
    vaultRebalanceManager: string,
    vaultName: string,
    vaultSymbol: string
  ) {
    this.rpc = rpc;
    this.horizonRpc = horizonRpc;
    this.passphrase = passphrase;
    this.friendbot = friendbot;
    this.admin = admin;
    this.blendKeeper = blendKeeper;
    this.defindexFeeReceiver = defindexFeeReceiver;
    this.defindexFactoryAdmin = defindexFactoryAdmin;
    this.defindexFee = defindexFee;
    this.vaultFeeReceiver = vaultFeeReceiver;
    this.vaultManager = vaultManager;
    this.vaultEmergencyManager = vaultEmergencyManager;
    this.vaultRebalanceManager = vaultRebalanceManager;
    this.vaultName = vaultName;
    this.vaultSymbol = vaultSymbol;
  }

  /**
   * Load the environment config from the .env file
   * @returns Environment config
   */
  static loadFromFile(network: string): EnvConfig {
    const fileContents = fs.readFileSync(
      path.join(__dirname, "../../configs.json"),
      "utf8"
    );
    const configs: Config = JSON.parse(fileContents);

    let rpc_url, horizon_rpc_url, friendbot_url, passphrase, blendKeeper, defindexFeeReceiver, defindexFactoryAdmin, defindexFee, vaultFeeReceiver, vaultManager;
    let vaultEmergencyManager, vaultRebalanceManager, vaultName, vaultSymbol;

    const networkConfig = configs.networkConfig.find(
      (config) => config.network === network
    );
    if (!networkConfig) {
      console.error("You must provide a valid network name");
      throw new Error(`Network configuration for '${network}' not found`);
    }
    const config_fields: string[] = [
      "network",
      "horizon_rpc_url",
      "soroban_network_passphrase",
      "blend_keeper",
      "defindex_fee_receiver",
      "defindex_factory_admin",
      "defindex_fee",
      "vault_fee_receiver",
      "vault_manager",
      "vault_emergency_manager",
      "vault_rebalance_manager",
      "vault_name",
      "vault_symbol",
    ];

    // Common assignments
    passphrase = networkConfig.soroban_network_passphrase;
    horizon_rpc_url = networkConfig.horizon_rpc_url;
    blendKeeper = networkConfig.blend_keeper;
    defindexFeeReceiver = networkConfig.defindex_fee_receiver;
    defindexFactoryAdmin = networkConfig.defindex_factory_admin;
    defindexFee = networkConfig.defindex_fee;
    vaultFeeReceiver = networkConfig.vault_fee_receiver;
    vaultManager = networkConfig.vault_manager;
    vaultEmergencyManager = networkConfig.vault_emergency_manager;
    vaultRebalanceManager = networkConfig.vault_rebalance_manager;
    vaultName = networkConfig.vault_name;
    vaultSymbol = networkConfig.vault_symbol;

    if (network === "mainnet") {
      rpc_url = process.env.MAINNET_RPC_URL;
      friendbot_url = undefined;
    } else {
      rpc_url = networkConfig.soroban_rpc_url;
      friendbot_url = networkConfig.friendbot_url;
      config_fields.push("friendbot_url");
      config_fields.push("soroban_rpc_url");
    }

    const admin = process.env.DEPLOYER_SECRET_KEY;

    for (const field of config_fields) {
      if (!(field in networkConfig)) {
        console.error(
          `Missing field '${field}' in network configuration for '${network}'`
        );
        throw new Error(
          `Missing field '${field}' in network configuration for '${network}'`
        );
      }
      if ((networkConfig[field as keyof NetworkConfig] ?? "").length < 1) {
        console.error(
          `Field '${field}' in network configuration for '${network}' must be defined`
        );
        throw new Error(
          `Field '${field}' in network configuration for '${network}' must be defined`
        );
      }
    }

    const allowHttp = network === "standalone";

    return new EnvConfig(
      new rpc.Server(rpc_url!, { allowHttp }),
      new Horizon.Server(horizon_rpc_url, { allowHttp }),
      passphrase,
      friendbot_url,
      Keypair.fromSecret(admin!),
      blendKeeper,
      defindexFeeReceiver,
      defindexFactoryAdmin,
      defindexFee,
      vaultFeeReceiver,
      vaultManager,
      vaultEmergencyManager,
      vaultRebalanceManager,
      vaultName,
      vaultSymbol
    );
  }

  /**
   * Get the Keypair for a user from the env file
   * @param userKey - The name of the user in the env file
   * @returns Keypair for the user
   */
  getUser(userKey: string): Keypair {
    const userSecretKey = process.env[userKey];
    if (userSecretKey != undefined) {
      return Keypair.fromSecret(userSecretKey);
    } else {
      throw new Error(`${userKey} secret key not found in .env`);
    }
  }
}

export const config = (network: string) => {
  return EnvConfig.loadFromFile(network);
};
