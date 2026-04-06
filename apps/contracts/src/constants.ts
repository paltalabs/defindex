import { Address } from "@stellar/stellar-sdk";
import { AddressBook } from "./utils/address_book.js";

const network = process.argv[2];
const otherAddressbook = AddressBook.loadFromFile(network, '../../public');

export const USDC_ADDRESS = new Address(otherAddressbook.getContractId("soroswap_usdc"));
export const BLEND_USDC_ADDRESS = new Address(otherAddressbook.getContractId("blend_pool_usdc"));
export const XTAR_ADDRESS = new Address(otherAddressbook.getContractId("soroswap_xtar"));
export const SOROSWAP_ROUTER = otherAddressbook.getContractId("soroswap_router");
export const BLEND_POOL = otherAddressbook.getContractId("blend_fixed_xlm_usdc_pool");
export const BLEND_TOKEN = otherAddressbook.getContractId("blnd_token");
function tryGetContractId(key: string): string {
  try {
    return otherAddressbook.getContractId(key);
  } catch {
    return "";
  }
}

export const CETES_BLEND_POOL = tryGetContractId("blend_pool_cetes");
export const CETES_TOKEN = tryGetContractId("cetes_token");
