import { Address, Keypair, nativeToScVal, scValToNative, xdr } from "@stellar/stellar-sdk";
import { airdropAccount, invokeCustomContract } from "../../utils/contract.js";
import { getDfTokenBalance } from "../vault.js";
import { randomBytes } from "crypto";
import { TxResponse } from '@soroban-react/contracts';
const blendStrategyAddress = "CCNFSOPH4XFQ5TNWGTJB4ZVKUUARSNQ67SETXVIQLUIW3B3F7KHA3NKJ"
const factoryAddress = "CB6RQM6ECU775ZC26NMZ6RJNKQLKQGLIJWWN2VZO6AGSE4V4DBQDL23O"
const XLMAddress = "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC"
const BlendUSDCAddress = ""
const network = process.argv[2];
const purple = '\x1b[35m%s\x1b[0m';
const green = '\x1b[32m%s\x1b[0m';


const newVault = {
  address: '',
  emergencyManager: 'GCH6YKNJ3KPESGSAIGBNHRNCIYXXXSRVU7OC552RDGQFHZ4SYRI26DQE',
  feeReceiver: 'GCH6YKNJ3KPESGSAIGBNHRNCIYXXXSRVU7OC552RDGQFHZ4SYRI26DQE',
  manager: 'GCH6YKNJ3KPESGSAIGBNHRNCIYXXXSRVU7OC552RDGQFHZ4SYRI26DQE',
  name: 'Test',
  symbol: 'Test1',
  vaultShare: 10,
  assets: [
    {
      address: 'CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC',
      strategies: [
        {
          address: 'CCNFSOPH4XFQ5TNWGTJB4ZVKUUARSNQ67SETXVIQLUIW3B3F7KHA3NKJ',
          name: 'Blend',
          paused: false
        }
      ],
      symbol: 'XLM',
      amount: 1000
    }
  ],
  TVL: 0
}
export async function createVault(user?: Keypair) {
  // Create and fund a new user account if not provided
  console.log(purple, '--------------------------------------------------------------------')
  console.log(purple, '----------------------- Creating new account -----------------------')
  console.log(purple, '--------------------------------------------------------------------')
  const newUser = Keypair.random();
  console.log('🚀 ~ depositToVault ~ newUser.publicKey():', newUser.publicKey());
  console.log('🚀 ~ depositToVault ~ newUser.secret():', newUser.secret());

  console.log(green, '----------------------- New account created -------------------------')
  console.log(green, 'Public key: ',newUser.publicKey())
  console.log(green, '---------------------------------------------------------------------')

  if (network !== "mainnet") {
    console.log(purple, '-------------------------------------------------------------------')
    console.log(purple, '----------------------- Funding new account -----------------------')
    console.log(purple, '-------------------------------------------------------------------')
    await airdropAccount(newUser);
  }
  console.log("New user publicKey:", newUser.publicKey());


  const indexName = "test";
  const indexSymbol = "TEST";
  const indexShare = 10;
  const managerString = newUser.publicKey();
  const vaultName = nativeToScVal(indexName, { type: "string" })
  const vaultSymbol = nativeToScVal(indexSymbol, { type: "string" })
  const vaultShare = nativeToScVal(indexShare, { type: "u32" })
  const emergencyManager = new Address(managerString)
  const feeReceiver = new Address(managerString)
  const manager = new Address(managerString)
  const salt = randomBytes(32)

  const strategyParamsScVal = xdr.ScVal.scvMap([
      new xdr.ScMapEntry({
        key: xdr.ScVal.scvSymbol('address'),
        val: new Address(blendStrategyAddress).toScVal(),
      }),
      new xdr.ScMapEntry({
        key: xdr.ScVal.scvSymbol('name'),
        val: nativeToScVal('Blend', { type: "string" }),
      }),
      new xdr.ScMapEntry({
        key: xdr.ScVal.scvSymbol('paused'),
        val: nativeToScVal(false, { type: "bool" }),
      }),
    ]);
  const strategyParamsScValVec = xdr.ScVal.scvVec([strategyParamsScVal]);
  const assetsParams = xdr.ScVal.scvMap([
    new xdr.ScMapEntry({
      key: xdr.ScVal.scvSymbol('address'),
      val: new Address(newVault.assets[0].address).toScVal(),
    }),
    new xdr.ScMapEntry({
      key: xdr.ScVal.scvSymbol('strategies'),
      val: strategyParamsScValVec,
    }),
  ]);
  const assetParamsScValVec = xdr.ScVal.scvVec([assetsParams]);
  const createDefindexParams: xdr.ScVal[] = [
    emergencyManager.toScVal(),
    feeReceiver.toScVal(),
    vaultShare,
    vaultName,
    vaultSymbol,
    manager.toScVal(),
    assetParamsScValVec,
    nativeToScVal(salt),
  ];
  let result: any;
  let blendVaultAddress: string;
  try {
    console.log(purple, '--------------------------------------------------------------')
    console.log(purple, '----------------------- Creating vault -----------------------')
    console.log(purple, '--------------------------------------------------------------')
    result = await invokeCustomContract(
      factoryAddress,
      'create_defindex_vault',
      createDefindexParams,
      newUser,
      false
    );
    blendVaultAddress = scValToNative(result.returnValue);
    console.log(green, '----------------------- Vault created -------------------------')
    console.log(green, 'result', blendVaultAddress)
    console.log(green, '---------------------------------------------------------------')
    
    
    // Deposit assets to the vault

    console.log(purple, '---------------------------------------------------------------------------')
    console.log(purple, '----------------------- Depositing XLM to the vault -----------------------')
    console.log(purple, '---------------------------------------------------------------------------')
    const depositParams: xdr.ScVal[] = [
      xdr.ScVal.scvVec([nativeToScVal(987654321, { type: "i128" })]),
      xdr.ScVal.scvVec([nativeToScVal(Math.ceil(987654321 * 0.9), { type: "i128" })]),
      new Address(newUser.publicKey()).toScVal(),
    ]
    const depositResult = await invokeCustomContract(
      blendVaultAddress,
      'deposit',
      depositParams,
      newUser,
      false
    );
    const depositResultValue = scValToNative(depositResult.returnValue);
    
    console.log(green, '------------ XLM deposited to the vault ------------')
    console.log(green, 'depositResult', depositResultValue)
    console.log(green, '----------------------------------------------------')
    
    // Withdraw assets from the vault

    console.log(purple, '------------------------------------------------------------------------------')
    console.log(purple, '----------------------- Withdrawing XLM from the vault -----------------------')
    console.log(purple, '------------------------------------------------------------------------------')
    const withdrawAmount = Math.ceil(100);
    const withdrawParams: xdr.ScVal[] = [
      nativeToScVal(withdrawAmount, { type: "i128" }),
      new Address(newUser.publicKey()).toScVal(),
    ]
    const withdrawResult = await invokeCustomContract(
      blendVaultAddress,
      'withdraw',
      withdrawParams,
      newUser,
      false
    );
    const withdrawResultValue = scValToNative(withdrawResult.returnValue);
    console.log(green, '---------------- XLM withdrawn from the vault ----------------')
    console.log(green, 'Withdrawed: ', withdrawResultValue, ' from the vault')
    console.log(green, '--------------------------------------------------------------')

    // Invest in strategy

    console.log(purple, '---------------------------------------------------------------------------')
    console.log(purple, '-------------------------- Investing in strategy --------------------------')
    console.log(purple, '---------------------------------------------------------------------------')

    const investment: any = [{
      "asset": "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC",
      "strategy_investments": [
        {
          "amount": 24,
          "strategy": "CCWUMJGE6LKWRDJ2IYEJBLCWJSMSUC3QCYZNI2MHTOEYPZRWZN56MIVA"
        }
      ],
    }]
    
    const investmentParams = investment.map((entry:any) =>
      xdr.ScVal.scvMap([
        new xdr.ScMapEntry({
          key: xdr.ScVal.scvSymbol("asset"),
          val: new Address(entry.asset).toScVal()// Convert asset address to ScVal
        }),
        new xdr.ScMapEntry({
          key: xdr.ScVal.scvSymbol("strategy_investments"),
          val: xdr.ScVal.scvVec(
            entry.strategy_investments.map((strategy_investment: any) => {
              return xdr.ScVal.scvMap([
                new xdr.ScMapEntry({
                  key: xdr.ScVal.scvSymbol("amount"),
                  val: nativeToScVal(BigInt((strategy_investment.amount ?? 0) * 10 ** 7), { type: "i128" }), // Ensure i128 conversion
                }),
                new xdr.ScMapEntry({
                  key: xdr.ScVal.scvSymbol("strategy"),
                  val: new Address(strategy_investment.strategy).toScVal() // Convert strategy address
                }),
              ])
            })
          ),
        }),
      ])
    )
    const investmentParamsScValVec = xdr.ScVal.scvVec(investmentParams);

    const investResult = await invokeCustomContract(
      blendVaultAddress,
      'invest',
      [investmentParamsScValVec],
      newUser,
      false
    );
    const investResultValue = scValToNative(investResult.returnValue);
    console.log(green, '---------------------- Invested in strategy ----------------------')
    console.log(green, 'Invested: ', investResultValue, ' in the strategy')
    console.log(green, '------------------------------------------------------------------')

  }catch(e){
    console.log('error', e)
  }
}
await createVault();