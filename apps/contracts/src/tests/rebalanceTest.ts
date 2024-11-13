import { depositToVault } from "./vault.js";
import { AddressBook } from "../utils/address_book.js";
import { airdropAccount, invokeContract, invokeCustomContract } from "../utils/contract.js";
import { config } from "../utils/env_config.js";
import { checkUserBalance } from "./strategy.js";

import {
    Address,
    Asset,
    nativeToScVal,
    Networks,
    scValToNative,
    xdr,
    Keypair
} from "@stellar/stellar-sdk";
import { randomBytes } from "crypto";

export async function test_factory(addressBook: AddressBook) {
    if (network !== "mainnet") await airdropAccount(loadedConfig.admin);
    console.log("Admin publicKey:", loadedConfig.admin.publicKey());

    console.log("-------------------------------------------------------");
    console.log("Testing Create DeFindex on Factory");
    console.log("-------------------------------------------------------");

    // Setup roles
    const emergencyManager = loadedConfig.getUser("DEFINDEX_EMERGENCY_MANAGER_SECRET_KEY");
    const feeReceiver = loadedConfig.getUser("DEFINDEX_FEE_RECEIVER_SECRET_KEY");
    const manager = loadedConfig.getUser("DEFINDEX_MANAGER_SECRET_KEY");

    // Airdrop to role accounts if not on mainnet
    if (network !== "mainnet") {
        await Promise.all([
            airdropAccount(emergencyManager),
            airdropAccount(feeReceiver),
            airdropAccount(manager)
        ]);
    }

    const assets = [
        {
            address: new Address(xlm.contractId(passphrase)),
            strategies: [
                {
                    name: "Strategy 1",
                    address: addressBook.getContractId("hodl_strategy"),
                    paused: false
                }
            ]
        }
    ];

    const assetAllocations = assets.map((asset) => {
        return xdr.ScVal.scvMap([
            new xdr.ScMapEntry({
                key: xdr.ScVal.scvSymbol("address"),
                val: asset.address.toScVal(),
            }),
            new xdr.ScMapEntry({
                key: xdr.ScVal.scvSymbol("strategies"),
                val: xdr.ScVal.scvVec(
                    asset.strategies.map((strategy) =>
                        xdr.ScVal.scvMap([
                            new xdr.ScMapEntry({
                                key: xdr.ScVal.scvSymbol("address"),
                                val: new Address(strategy.address).toScVal(),
                            }),
                            new xdr.ScMapEntry({
                                key: xdr.ScVal.scvSymbol("name"),
                                val: nativeToScVal(strategy.name, { type: "string" }),
                            }),
                            new xdr.ScMapEntry({
                                key: xdr.ScVal.scvSymbol("paused"),
                                val: nativeToScVal(false, { type: "bool" }),
                            }),
                        ])
                    )
                ),
            }),
        ]);
    });

    const createDeFindexParams: xdr.ScVal[] = [
        new Address(emergencyManager.publicKey()).toScVal(),
        new Address(feeReceiver.publicKey()).toScVal(),
        nativeToScVal(100, { type: "u32" }),  // Setting vault_fee as 100 bps
        nativeToScVal("Test Vault", { type: "string" }),
        nativeToScVal("DFT-Test-Vault", { type: "string" }),
        new Address(manager.publicKey()).toScVal(),
        xdr.ScVal.scvVec(assetAllocations),
        nativeToScVal(randomBytes(32)),
    ];

    const result = await invokeContract(
        'defindex_factory',
        addressBook,
        'create_defindex_vault',
        createDeFindexParams,
        loadedConfig.admin
    );

    const deployedVault = scValToNative(result.returnValue);
    console.log('🚀 DeFindex Vault created with address:', deployedVault);
    return { deployedVault, manager };
}

const network = process.argv[2];
const addressBook = AddressBook.loadFromFile(network);
const xlm: Asset = Asset.native();
const passphrase = network === "mainnet" ? Networks.PUBLIC : network === "testnet" ? Networks.TESTNET : Networks.STANDALONE;
const loadedConfig = config(network);

async function main() {
    // Step 1: Deploy the vault and get the manager
    console.log("Step 1: Deploying vault...");
    const { deployedVault, manager } = await test_factory(addressBook);
    console.log("Vault deployed at:", deployedVault);
    console.log("Manager address:", manager.publicKey());

    // Step 2: Create and fund a new user for deposit
    console.log("\nStep 2: Creating new user for deposit...");
    const depositUser = Keypair.random();
    if (network !== "mainnet") await airdropAccount(depositUser);
    console.log("Deposit user created with address:", depositUser.publicKey());

    // Step 3: User deposits into vault
    console.log("\nStep 3: Making deposit...");
    const depositAmount = 1000000000; // 100 XLM
    const { balanceBefore: depositBalanceBefore, balanceAfter: depositBalanceAfter }
        = await depositToVault(deployedVault, depositAmount, depositUser);
    console.log("Deposit completed - Balance before:", depositBalanceBefore, "Balance after:", depositBalanceAfter);

    // Step 4: Manager invests in strategy
    console.log("\nStep 4: Manager investing in strategy...");
    const strategyAddress = addressBook.getContractId("hodl_strategy");
    const investmentAmount = depositAmount; // Invest all deposited amount

    const assetInvestments = [{
        asset: new Address(xlm.contractId(passphrase)),
        strategy_investments: [{
            strategy: new Address(strategyAddress),
            amount: BigInt(investmentAmount)
        }]
    }];

    const investParams: xdr.ScVal[] = [
        xdr.ScVal.scvVec(assetInvestments.map(investment =>
            xdr.ScVal.scvMap([
                new xdr.ScMapEntry({
                    key: xdr.ScVal.scvSymbol("asset"),
                    val: investment.asset.toScVal(),
                }),
                new xdr.ScMapEntry({
                    key: xdr.ScVal.scvSymbol("strategy_investments"),
                    val: xdr.ScVal.scvVec(investment.strategy_investments.map(si =>
                        xdr.ScVal.scvMap([
                            new xdr.ScMapEntry({
                                key: xdr.ScVal.scvSymbol("strategy"),
                                val: si.strategy.toScVal(),
                            }),
                            new xdr.ScMapEntry({
                                key: xdr.ScVal.scvSymbol("amount"),
                                val: nativeToScVal(si.amount, { type: "i128" }),
                            }),
                        ])
                    )),
                }),
            ])
        ))
    ];

    try {
        const investResult = await invokeCustomContract(
            deployedVault,
            "invest",
            investParams,
            manager
        );
        console.log("Investment successful:", scValToNative(investResult.returnValue));

        // Check strategy balance after investment
        const strategyBalance = await checkUserBalance(strategyAddress, depositUser.publicKey(), depositUser);
        console.log("Strategy balance after investment:", strategyBalance);
    } catch (error) {
        console.error("Investment failed:", error);
    }
}

// Run the test
main().catch(console.error); 