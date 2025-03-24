import { Asset } from "@/store/lib/types";
import { Address, nativeToScVal, xdr } from "@stellar/stellar-sdk";

export function getAssetAmountsSCVal(assets: Asset[]) {
  return assets.map((asset, index) => {
    const parsedAmount = assets[index]?.amount || 0;
    const truncatedAmount = Math.floor(parsedAmount * 1e7) / 1e7;
    const convertedAmount = Number(truncatedAmount) * Math.pow(10, 7)
    if (assets[index]?.amount === 0) return nativeToScVal(0, { type: "i128" });
    return nativeToScVal(convertedAmount, { type: "i128" });
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
        val: new Address(asset.address).toScVal(),
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
    nativeToScVal(vault_fee, { type: "u32" }), // Setting vault_fee as 100 bps for demonstration
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