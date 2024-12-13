import {
  Account,
  Keypair,
  rpc,
  Transaction,
  TransactionBuilder,
  xdr,
} from "@stellar/stellar-sdk";
import { config } from "./env_config.js";

type txResponse =
  | rpc.Api.SendTransactionResponse
  | rpc.Api.GetTransactionResponse;
type txStatus =
  | rpc.Api.SendTransactionStatus
  | rpc.Api.GetTransactionStatus;

const network = process.argv[2];
const loadedConfig = config(network);

export async function signWithKeypair(
  txXdr: string,
  passphrase: string,
  source: Keypair
): Promise<string> {
  const tx = new Transaction(txXdr, passphrase);
  tx.sign(source);
  return tx.toXDR();
}

export async function invoke(
  operation: string | xdr.Operation,
  source: Keypair,
  sim: boolean
): Promise<any> {
  const txBuilder = await createTxBuilder(source);
  if (typeof operation === "string") {
    operation = xdr.Operation.fromXDR(operation, "base64");
  }
  txBuilder.addOperation(operation);
  const tx = txBuilder.build();
  return invokeTransaction(tx, source, sim);
}

export async function invokeTransaction(
  tx: Transaction,
  source: Keypair,
  sim: boolean
) {
  // simulate the TX
  console.log(tx.toXDR());
  const simulation_resp = await loadedConfig.rpc.simulateTransaction(tx);
  if (rpc.Api.isSimulationError(simulation_resp)) {
    // No resource estimation available from a simulation error. Allow the response formatter
    // to fetch the error.
    console.log("simulation_resp", simulation_resp);
    throw Error(`Simulation error`);
  } else if (sim) {
    // Only simulate the TX. Assemble the TX to borrow the resource estimation algorithm in
    return simulation_resp;
  }

  // assemble and sign the TX
  const txResources = simulation_resp.transactionData.build().resources();
  simulation_resp.minResourceFee = (
    Number(simulation_resp.minResourceFee) + 10000000
  ).toString();
  const sim_tx_data = simulation_resp.transactionData
    .setResources(
      txResources.instructions() == 0 ? 0 : txResources.instructions() + 500000,
      txResources.readBytes(),
      txResources.writeBytes()
    )
    .build();
  const assemble_tx = rpc.assembleTransaction(tx, simulation_resp);
  sim_tx_data.resourceFee(
    xdr.Int64.fromString(
      (Number(sim_tx_data.resourceFee().toString()) + 100000).toString()
    )
  );
  const prepped_tx = assemble_tx.setSorobanData(sim_tx_data).build();
  prepped_tx.sign(source);
  const tx_hash = prepped_tx.hash().toString("hex");
  console.log('🛑 SIGNED TX:', prepped_tx.toXDR());

  console.log("submitting tx...");
  let response: txResponse = await loadedConfig.rpc.sendTransaction(prepped_tx);
  let status: txStatus = response.status;
  console.log(`Hash: ${tx_hash}`);
  // Poll this until the status is not "NOT_FOUND"
  while (status === "PENDING" || status === "NOT_FOUND") {
    // See if the transaction is complete
    await new Promise((resolve) => setTimeout(resolve, 2000));
    console.log("checking tx...");
    response = await loadedConfig.rpc.getTransaction(tx_hash);
    status = response.status;
  }
  return response;
}

export async function createTxBuilder(
  source: Keypair
): Promise<TransactionBuilder> {
  try {
    const account: Account = await loadedConfig.rpc.getAccount(
      source.publicKey()
    );
    return new TransactionBuilder(account, {
      fee: "10000",
      timebounds: { minTime: 0, maxTime: 0 },
      networkPassphrase: loadedConfig.passphrase,
    });
  } catch (e: any) {
    console.error(e);
    throw Error("unable to create txBuilder");
  }
}

export const getCurrentTimePlusOneHour = () => {
  // Get the current time in milliseconds
  const now = Date.now();

  // Add one hour (3600000 milliseconds)
  const oneHourLater = now + 3600000;

  return oneHourLater;
};

export function getTransactionBudget(tx: any): { instructions: number, readBytes: number, writeBytes: number } {
  const resources = tx.envelopeXdr.value().tx().ext().value().resources()
  const warningTolerance = 0.85
  const MAXWRITEBYTES = 132096
  const MAXREADBYTES = 200000
  const MAXINSTRUCTIONS = 100000000
  const budget= {
      instructions: resources.instructions(),
      readBytes: resources.readBytes(),
      writeBytes: resources.writeBytes(),
  }
  const getPercentage = (value: number, max: number)=>{
      return (value * 100)/max
  }
  if(budget.instructions > MAXINSTRUCTIONS * warningTolerance){
      console.warn('Instructions budget exceeded')
      console.table({
          value:{
              instructions: budget.instructions,
              maxInstructions: MAXINSTRUCTIONS,
              '%': getPercentage(budget.instructions, MAXINSTRUCTIONS)
          },
      })
  }
  if(budget.readBytes > MAXREADBYTES * warningTolerance){
      console.warn('ReadBytes budget exceeded')
      console.table({
          value: {
              readBytes: budget.readBytes,
              maxReadBytes: MAXREADBYTES,
              '%': getPercentage(budget.readBytes, MAXREADBYTES)
          }
      })
  }
  if(budget.writeBytes > MAXWRITEBYTES * warningTolerance){
      console.warn('WriteBytes budget exceeded')
      console.table({
          value:{
              writeBytes: budget.writeBytes,
              maxWriteBytes: MAXWRITEBYTES,
              '%': getPercentage(budget.writeBytes, MAXWRITEBYTES)
          }
      })
  }
  return budget
}