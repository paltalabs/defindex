import { Address, Asset, nativeToScVal, xdr } from "@stellar/stellar-sdk";
import { exit } from "process";
import { BLEND_POOL, BLEND_TOKEN, SOROSWAP_ROUTER } from "../constants.js";
import { AddressBook } from "./address_book.js";
import { EnvConfig } from "./env_config.js";


export enum Strategies {
  BLEND = "blend",
  HODL = "hodl",
  FIXED_APR = "fixed_apr",
}

export enum Assets {
  XLM = "XLM",
  USDC = "USDC",
}

export interface StrategyDeployArgs {
  assetAddress: Address;
  strategyData: StrategyData[];
}

export interface StrategyData {
  name: string;
  wasm_key: string;
  args: xdr.ScVal;
}

export function AssetFromString(asset: string, config: EnvConfig, externalAddressBook:AddressBook): Address {
  if (!asset || !Object.values(Assets).includes(asset.toUpperCase() as Assets)) {
    console.warn("Please provide a valid asset symbol");
    console.warn("Allowed assets are: \n - XLM \n - USDC");
    throw new Error("Invalid asset symbol");
  }
  const xlmAddress = new Address(
    Asset.native().contractId(config.passphrase)
  );

  let assetAddress: Address;

  switch (asset.toUpperCase()) {
    case Assets.XLM:
      assetAddress = xlmAddress;
      break;
    case Assets.USDC:
      assetAddress = new Address(externalAddressBook.getContractId("blend_pool_usdc"));
      break;
    default:
      throw new Error("Invalid asset symbol");
  }

  return assetAddress;
}

function isValidStrategy(strategies: Strategies[]): boolean {
  return strategies && strategies.length > 0 && Object.values(Strategies).some(strategy => strategies.includes(strategy));
}

export function InitStrategyDeploy(asset: string, strategies:Strategies[], externalAddressBook:AddressBook, config: EnvConfig) {

  const assetAddress = AssetFromString(asset, config, externalAddressBook);

  if (!isValidStrategy(strategies)) {
    console.warn("Please provide a valid strategy");
    console.warn("Allowed strategies are: \n - blend \n - hodl \n - fixed_apr");
    exit(1);
  }
  const strategyData = strategies.map((strategy) => {
    let args: xdr.ScVal;
      switch (strategy){
        case Strategies.HODL:
          args = xdr.ScVal.scvVec([]);
          break;
        case Strategies.FIXED_APR:
          args = xdr.ScVal.scvVec([
              nativeToScVal(1000, { type: "u32" }), // 10% APR
            ]);
          break;
        case Strategies.BLEND:
          args = xdr.ScVal.scvVec([
            new Address(BLEND_POOL).toScVal(), // blend_pool_address: The address of the Blend pool where assets are deposited
            new Address(BLEND_TOKEN).toScVal(), // blend_token: The address of the reward token (e.g., BLND) issued by the Blend pool
            new Address(SOROSWAP_ROUTER).toScVal(), // soroswap_router: The address of the Soroswap AMM router for asset swaps
            nativeToScVal(40, { type: "i128" }), // reward_threshold: The minimum reward amount that triggers reinvestment
            new Address(config.blendKeeper).toScVal() // keeper: The address of the keeper that can call the harvest function
          ]);
          break;
        default:
          args = xdr.ScVal.scvVec([]);
        break;
  
    }

    return {
      name: `${asset}_${strategy}_strategy`,
      wasm_key: `${strategy}_strategy`,
      args: args,
    };
  });

  return {
    assetAddress,
    strategyData,
  };
}

export function InitVaultDeploy(asset: string, strategies:Strategies[], publicAddressBook: AddressBook, externalAddressBook:AddressBook, config: EnvConfig) {
  const assetAddress = AssetFromString(asset, config, externalAddressBook);

  if (!strategies || strategies.length === 0 || !Object.values(Strategies).some(strategy => strategies.includes(strategy))) {
    console.warn("Please provide a valid strategy");
    console.warn("Allowed strategies are: \n - blend \n - hodl \n - fixed_apr");
    exit(1);
  }
  const strategyData = strategies.map((strategy) => {
    return {
      name: `${asset}_${strategy}_strategy`,
      address: publicAddressBook.getContractId(`${asset.toLowerCase()}_${strategy}_strategy`),
      paused: false,
    };
  });

  return {
    assetAddress,
    strategyData,
  };
}