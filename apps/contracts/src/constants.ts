import { Address } from "@stellar/stellar-sdk";
import { AddressBook } from "./utils/address_book.js";

const otherAddressbook = AddressBook.loadFromFile('../../../public');

export const USDC_ADDRESS = new Address(otherAddressbook.getContractId("soroswap_usdc"));
export const SOROSWAP_ROUTER = otherAddressbook.getContractId("soroswap_router");
export const BLEND_POOL = otherAddressbook.getContractId("blend_fixed_xlm_usdc_pool");
export const BLEND_TOKEN = otherAddressbook.getContractId("blnd_token");