import { Asset } from "@/contexts"
import { Address, nativeToScVal, xdr } from "@stellar/stellar-sdk";

export function getAssetAmountsSCVal(assets: Asset[]) {
  return assets.map((asset, index) => {
    const parsedAmount = assets[index]?.amount || 0;
    const truncatedAmount = Math.floor(parsedAmount * 1e7) / 1e7;
    const convertedAmount = Number(truncatedAmount) * Math.pow(10, 7)
    if (assets[index]?.amount === 0) return nativeToScVal(0, { type: "i128" });
    return nativeToScVal(Math.ceil(convertedAmount), { type: "i128" });
  })
}

export function getAssetParamsSCVal(assets: Asset[]) {
  return assets.map((asset) => {
    const strategyParamsScVal = asset.strategies.map((param) => {
      return xdr.ScVal.scvMap([
        new xdr.ScMapEntry({
          key: xdr.ScVal.scvSymbol('address'),
          val: new Address(param.address).toScVal(),
        }),
        new xdr.ScMapEntry({
          key: xdr.ScVal.scvSymbol('name'),
          val: nativeToScVal(param.name, { type: "string" }),
        }),
        new xdr.ScMapEntry({
          key: xdr.ScVal.scvSymbol('paused'),
          val: nativeToScVal(false, { type: "bool" }),
        }),
      ]);
    });
    const strategyParamsScValVec = xdr.ScVal.scvVec(strategyParamsScVal);
    return xdr.ScVal.scvMap([
      new xdr.ScMapEntry({
        key: xdr.ScVal.scvSymbol('address'),
        val: new Address(asset.asset).toScVal(),
      }),
      new xdr.ScMapEntry({
        key: xdr.ScVal.scvSymbol('strategies'),
        val: strategyParamsScValVec,
      }),
    ]);
  });
}

export function getCreateDeFindexVaultParams(
  emergency_manager: string,
  rebalance_manager: string,
  fee_receiver: string,
  manager: string,
  vault_fee: number,
  vault_name: string,
  vault_symbol: string,
  asset_allocations: xdr.ScVal[],
  router_address: string,
  upgradable: boolean,
): xdr.ScVal[] {
  const roles = xdr.ScVal.scvMap([
    new xdr.ScMapEntry({
      key: xdr.ScVal.scvU32(0),
      val: new Address(emergency_manager).toScVal(),
    }),
    new xdr.ScMapEntry({
      key: xdr.ScVal.scvU32(1),
      val: new Address(fee_receiver).toScVal(),
    }),
    new xdr.ScMapEntry({
      key: xdr.ScVal.scvU32(2),
      val: new Address(manager).toScVal(),
    }),
    new xdr.ScMapEntry({
      key: xdr.ScVal.scvU32(3),
      val: new Address(rebalance_manager).toScVal(),
    }),
  ]);

  const nameSymbol = xdr.ScVal.scvMap([
    new xdr.ScMapEntry({
      key: xdr.ScVal.scvString("name"),
      val: nativeToScVal(vault_name ?? "TestVault", { type: "string" }),
    }),
    new xdr.ScMapEntry({
      key: xdr.ScVal.scvString("symbol"),
      val: nativeToScVal(vault_symbol ?? "TSTV", { type: "string" }),
    }),
  ]);

  return [
    roles,
    nativeToScVal(vault_fee * 100, { type: "u32" }), // Converting vault_fee to basis points (bps)
    xdr.ScVal.scvVec(asset_allocations),
    new Address(router_address).toScVal(),
    nameSymbol,
    nativeToScVal(!!upgradable, { type: "bool" })
  ];
}

export function getCreateDeFindexVaultDepositParams(
  caller: string,
  emergency_manager: string,
  rebalance_manager: string,
  fee_receiver: string,
  manager: string,
  vault_fee: number,
  vault_name: string,
  vault_symbol: string,
  asset_allocations: xdr.ScVal[],
  router_address: string,
  upgradable: boolean,
  assets: Asset[]
){
  const defindexVaultParams = getCreateDeFindexVaultParams(
    emergency_manager,
    rebalance_manager,
    fee_receiver,
    manager,
    vault_fee,
    vault_name,
    vault_symbol,
    asset_allocations,
    router_address,
    upgradable
  );
  const callerAddress = new Address(caller);
  const amounts = getAssetAmountsSCVal(assets);
  return [callerAddress.toScVal(), ...defindexVaultParams, xdr.ScVal.scvVec(amounts)];
}

/* export function mapInstructionsToParams(
  instructions: RebalanceInstruction[]
): xdr.ScVal {
  return xdr.ScVal.scvVec(
    instructions.map((instruction) => {
      let SCALING_FACTOR = 10 ** 7;
      let parsedAmount = Math.ceil(instruction.amount * SCALING_FACTOR);
      switch (instruction.action) {
        case ActionType.Invest:
          return xdr.ScVal.scvVec([
            // Invest action
            xdr.ScVal.scvSymbol("Invest"), 
            new Address(instruction.strategy).toScVal(),
            nativeToScVal(parsedAmount, { type: "i128" }), // amount
          ]);
        case ActionType.Unwind:
          return xdr.ScVal.scvVec([
            // Unwind action
            xdr.ScVal.scvSymbol("Unwind"), 
            new Address(instruction.strategy).toScVal(),
            nativeToScVal(parsedAmount, { type: "i128" }), // amount
          ]);

        case ActionType.SwapExactIn:
          // Handle SwapExactIn action
          return xdr.ScVal.scvVec([
            xdr.ScVal.scvSymbol("SwapExactIn"),
            new Address(instruction.swapDetailsExactIn.token_in).toScVal(),
            new Address(instruction.swapDetailsExactIn.token_out).toScVal(),
            nativeToScVal(Math.ceil(instruction.swapDetailsExactIn.amount_in * SCALING_FACTOR), { type: "i128" }),
            nativeToScVal(Math.ceil(instruction.swapDetailsExactIn.amount_out_min * SCALING_FACTOR), { type: "i128" }),
            nativeToScVal(instruction.swapDetailsExactIn.deadline, { type: "u64" }),
          ]);

        case ActionType.SwapExactOut:
          // Handle SwapExactOut action
          return xdr.ScVal.scvVec([
            xdr.ScVal.scvSymbol("SwapExactOut"),
            new Address(instruction.swapDetailsExactOut.token_in).toScVal(),
            new Address(instruction.swapDetailsExactOut.token_out).toScVal(),
            nativeToScVal(Math.ceil(instruction.swapDetailsExactOut.amount_out * SCALING_FACTOR), { type: "i128" }),
            nativeToScVal(Math.ceil(instruction.swapDetailsExactOut.amount_in_max * SCALING_FACTOR), { type: "i128" }),
            nativeToScVal(instruction.swapDetailsExactOut.deadline, { type: "u64" }),
          ]);

        default:
          throw new Error(`Unsupported action type: ${instruction.action}`);
      }
    })
  );
} */