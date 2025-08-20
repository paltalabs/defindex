import {
  Address,
  Asset,
  Contract,
  Keypair,
  Operation,
  StrKey,
  TimeoutInfinite,
  TransactionBuilder,
  hash,
  scValToNative,
  xdr,
} from "@stellar/stellar-sdk";
import { randomBytes } from "crypto";
import { readFileSync } from "fs";
import path from "path";
import { fileURLToPath } from "url";
import { AddressBook } from "./address_book.js";
import { config } from "./env_config.js";
import { createTxBuilder, invoke, invokeTransaction } from "./tx.js";

// Relative paths from __dirname
const CONTRACT_REL_PATH: object = {
  defindex_vault:
    "../../target/wasm32-unknown-unknown/release/defindex_vault.optimized.wasm",
  defindex_factory:
    "../../target/wasm32-unknown-unknown/release/defindex_factory.optimized.wasm",
  soroswap_adapter:
    "../../target/wasm32-unknown-unknown/release/soroswap_adapter.optimized.wasm",
  hodl_strategy:
    "../../target/wasm32-unknown-unknown/release/hodl_strategy.optimized.wasm",
  blend_strategy:
    "../../target/wasm32-unknown-unknown/release/blend_strategy.optimized.wasm",
  fixed_apr_strategy:
    "../../target/wasm32-unknown-unknown/release/fixed_apr_strategy.optimized.wasm",
};

const network = process.argv[2];
const loadedConfig = config(network);

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

export async function installContract(
  wasmKey: string,
  addressBook: AddressBook,
  source: Keypair
) {
  const contractWasm = readFileSync(
    path.join(__dirname, CONTRACT_REL_PATH[wasmKey as keyof object])
  );
  const wasmHash = hash(contractWasm);
  addressBook.setWasmHash(wasmKey, wasmHash.toString("hex"));
  console.log("Installing:", wasmKey, wasmHash.toString("hex"));
  const op = Operation.invokeHostFunction({
    func: xdr.HostFunction.hostFunctionTypeUploadContractWasm(contractWasm),
    auth: [],
  });
  addressBook.writeToFile();
  try { 
    const result = await invoke(op, source, false);
    if (result.status == 'ERROR'){
      console.log("Error installing contract:", result.errorResult.result());
      throw new Error("Error installing contract");
    }

  } catch (error:any) {
    throw new Error(error);
  }
  
}

export async function deployContract(
  contractKey: string,
  wasmKey: string,
  addressBook: AddressBook,
  args: xdr.ScVal[],
  source: Keypair
) {
  const contractIdSalt = randomBytes(32);
  const networkId = hash(Buffer.from(loadedConfig.passphrase));
  const contractIdPreimage =
    xdr.ContractIdPreimage.contractIdPreimageFromAddress(
      new xdr.ContractIdPreimageFromAddress({
        address: Address.fromString(source.publicKey()).toScAddress(),
        salt: contractIdSalt,
      })
    );

  const hashIdPreimage = xdr.HashIdPreimage.envelopeTypeContractId(
    new xdr.HashIdPreimageContractId({
      networkId: networkId,
      contractIdPreimage: contractIdPreimage,
    })
  );
  console.log("Deploying WASM", wasmKey, "for", contractKey);
  const contractId = StrKey.encodeContract(hash(hashIdPreimage.toXDR()));
  addressBook.setContractId(contractKey, contractId);
  const wasmHash = Buffer.from(addressBook.getWasmHash(wasmKey), "hex");

  const deployFunction = xdr.HostFunction.hostFunctionTypeCreateContractV2(
    new xdr.CreateContractArgsV2({
      contractIdPreimage,
      executable: xdr.ContractExecutable.contractExecutableWasm(wasmHash),
      constructorArgs: args,
    })
  );

  addressBook.writeToFile();
  await invoke(
    Operation.invokeHostFunction({
      func: deployFunction,
      auth: [],
    }),
    source,
    false
  );
}

export async function invokeContract(
  contractKey: string,
  addressBook: AddressBook,
  method: string,
  params: xdr.ScVal[],
  source: Keypair,
  simulation?: boolean
) {
  console.log("Invoking contract: ", contractKey, " with method: ", method);
  const contractAddress = addressBook.getContractId(contractKey);
  console.log("🚀 « contractAddress:", contractAddress);
  const contractInstance = new Contract(contractAddress);

  const contractOperation = contractInstance.call(method, ...params);
  return await invoke(contractOperation, source, simulation ?? false);
}

export async function invokeCustomContract(
  contractId: string,
  method: string,
  params: xdr.ScVal[],
  source: Keypair,
  simulation?: boolean
) {
  console.log("Invoking contract: ", contractId, " with method: ", method);
  const contractInstance = new Contract(contractId);

  const contractOperation = contractInstance.call(method, ...params);
  return await invoke(contractOperation, source, simulation ?? false);
}

export async function deployStellarAsset(asset: Asset, source: Keypair) {
  const xdrAsset = asset.toXDRObject();
  const networkId = hash(Buffer.from(loadedConfig.passphrase));
  const preimage = xdr.HashIdPreimage.envelopeTypeContractId(
    new xdr.HashIdPreimageContractId({
      networkId: networkId,
      contractIdPreimage:
        xdr.ContractIdPreimage.contractIdPreimageFromAsset(xdrAsset),
    })
  );
  const contractId = StrKey.encodeContract(hash(preimage.toXDR()));
  console.log("🚀 « deployed Stellar Asset:", contractId);

  const deployFunction = xdr.HostFunction.hostFunctionTypeCreateContract(
    new xdr.CreateContractArgs({
      contractIdPreimage:
        xdr.ContractIdPreimage.contractIdPreimageFromAsset(xdrAsset),
      executable: xdr.ContractExecutable.contractExecutableStellarAsset(),
    })
  );
  return await invoke(
    Operation.invokeHostFunction({
      func: deployFunction,
      auth: [],
    }),
    source,
    false
  );
}

export async function bumpContractInstance(
  contractKey: string,
  addressBook: AddressBook,
  source: Keypair
) {
  const address = Address.fromString(addressBook.getContractId(contractKey));
  console.log("bumping contract instance: ", address.toString());
  const contractInstanceXDR = xdr.LedgerKey.contractData(
    new xdr.LedgerKeyContractData({
      contract: address.toScAddress(),
      key: xdr.ScVal.scvLedgerKeyContractInstance(),
      durability: xdr.ContractDataDurability.persistent(),
    })
  );
  const bumpTransactionData = new xdr.SorobanTransactionData({
    resources: new xdr.SorobanResources({
      footprint: new xdr.LedgerFootprint({
        readOnly: [contractInstanceXDR],
        readWrite: [],
      }),
      instructions: 0,
      diskReadBytes: 0,
      writeBytes: 0,
    }),
    resourceFee: xdr.Int64.fromString("0"),
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    ext: new xdr.ExtensionPoint(0),
  });

  const txBuilder = await createTxBuilder(source);
  txBuilder.addOperation(Operation.extendFootprintTtl({ extendTo: 535670 })); // 1 year
  txBuilder.setSorobanData(bumpTransactionData);
  const result = await invokeTransaction(txBuilder.build(), source, false);
  //@ts-ignore
  console.log(result.status, "\n");
}

export async function bumpContractCode(
  wasmKey: string,
  addressBook: AddressBook,
  source: Keypair
) {
  console.log("bumping contract code: ", wasmKey);
  const wasmHash = Buffer.from(addressBook.getWasmHash(wasmKey), "hex");
  const contractCodeXDR = xdr.LedgerKey.contractCode(
    new xdr.LedgerKeyContractCode({
      hash: wasmHash,
    })
  );
  const bumpTransactionData = new xdr.SorobanTransactionData({
    resources: new xdr.SorobanResources({
      footprint: new xdr.LedgerFootprint({
        readOnly: [contractCodeXDR],
        readWrite: [],
      }),
      instructions: 0,
      diskReadBytes: 0,
      writeBytes: 0,
    }),
    resourceFee: xdr.Int64.fromString("0"),
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    ext: new xdr.ExtensionPoint(0),
  });

  const txBuilder = await createTxBuilder(source);
  txBuilder.addOperation(Operation.extendFootprintTtl({ extendTo: 535670 })); // 1 year
  txBuilder.setSorobanData(bumpTransactionData);
  const result = await invokeTransaction(txBuilder.build(), source, false);
  //@ts-ignore
  console.log(result.status, "\n");
}

export async function airdropAccount(user: Keypair) {
  try {
    console.log("Start funding");
    await loadedConfig.rpc.requestAirdrop(
      user.publicKey(),
      loadedConfig.friendbot
    );
    console.log("Funded: ", user.publicKey());
  } catch (e) {
    console.log(user.publicKey(), " already funded");
  }
}

export async function airdropAddress(user: string) {
  try {
    console.log("Start funding");
    await loadedConfig.rpc.requestAirdrop(
      user,
      loadedConfig.friendbot
    );
    console.log("Funded: ", user);
  } catch (e) {
    console.log(user, " already funded");
  }
}

export async function deploySorobanToken(
  wasmKey: string,
  addressBook: AddressBook,
  source: Keypair
) {
  const contractIdSalt = randomBytes(32);
  const networkId = hash(Buffer.from(loadedConfig.passphrase));
  const contractIdPreimage =
    xdr.ContractIdPreimage.contractIdPreimageFromAddress(
      new xdr.ContractIdPreimageFromAddress({
        address: Address.fromString(source.publicKey()).toScAddress(),
        salt: contractIdSalt,
      })
    );

  const hashIdPreimage = xdr.HashIdPreimage.envelopeTypeContractId(
    new xdr.HashIdPreimageContractId({
      networkId: networkId,
      contractIdPreimage: contractIdPreimage,
    })
  );
  const contractId = StrKey.encodeContract(hash(hashIdPreimage.toXDR()));
  const wasmHash = Buffer.from(addressBook.getWasmHash(wasmKey), "hex");

  const deployFunction = xdr.HostFunction.hostFunctionTypeCreateContract(
    new xdr.CreateContractArgs({
      contractIdPreimage: contractIdPreimage,
      executable: xdr.ContractExecutable.contractExecutableWasm(wasmHash),
    })
  );

  // addressBook.writeToFile();
  const result = await invoke(
    Operation.invokeHostFunction({
      func: deployFunction,
      auth: [],
    }),
    source,
    false
  );

  if (result) {
    return contractId;
  }
}

export async function getTokenBalance(
  contractId: string,
  from: string,
  source: Keypair
) {
  const tokenContract = new Contract(contractId);
  const op = tokenContract.call("balance", new Address(from).toScVal());

  const result = await invoke(op, source, true);
  const parsedResult = scValToNative(result.result.retval).toString();

  if (!parsedResult) {
    throw new Error("The operation has no result.");
  }
  if (parsedResult == 0) {
    return parsedResult;
  }
  const resultNumber = parseInt(parsedResult.slice(0, -1));
  return resultNumber;
}

/**
 * Sets a trustline for a token on the Stellar network.
 * @param {Object} options - The options object.
 * @param {string} options.tokenSymbol - The symbol of the token.
 * @param {Keypair} options.source - The source keypair for the transaction.
 * @returns {Promise<StellarSdk.TransactionResponse>} A promise that resolves with the transaction response.
 * @throws {Error} Throws an error if there is no active chain, no sorobanServer connected, or if network passphrase is missing.
 */
export async function setBlendTrustline({
  source,
  tokenSymbol,
}: {
  source: Keypair
  tokenSymbol: string
}) {
  let tokenAdmin = loadedConfig.getUser("BLEND_DEPLOYER_SECRET_KEY");
  let account = await loadedConfig.rpc.getAccount(source.publicKey());
  const operation = Operation.changeTrust({
    source: source.publicKey(),
    asset: new Asset(tokenSymbol, tokenAdmin.publicKey()),
  })

  const txn = new TransactionBuilder(account, {
    fee: '100',
    timebounds: { minTime: 0, maxTime: 0 },
    networkPassphrase: loadedConfig.passphrase,
  })
    .addOperation(operation)
    .setTimeout(TimeoutInfinite)
    .build()

  txn.sign(source);
  
  try {
    const submitted_transaction = await loadedConfig.horizonRpc.submitTransaction(txn);
    if(submitted_transaction.successful){
      console.log("Trustline set successfully");
    } else {
      throw submitted_transaction;
    }
  } catch (error) {
    console.log("Error setting trustline:", error);
  }
}