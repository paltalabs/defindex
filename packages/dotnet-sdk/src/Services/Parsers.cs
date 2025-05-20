using DeFindex.Sdk.Interfaces;
using DeFindex.Sdk.Services;
using StellarDotnetSdk.Responses.SorobanRpc;
using StellarDotnetSdk.Soroban;
using System.Text.Json.Nodes;

public class DefindexResponseParser
{
    public static List<ManagedFundsResult> ParseManagedFundsResult(SimulateTransactionResponse response)
    {
        var managedFundsResults = new List<ManagedFundsResult>();

        if (response.Results == null || response.Results.Length == 0)
        {
            Console.WriteLine("No results found.");
            return managedFundsResults;
        }

        var xdrString = response.Results[0].Xdr;

        if (xdrString == null)
        {
            Console.WriteLine("XDR string for total managed funds is null.");
            return managedFundsResults;
        }

        var result_xdr = new StellarDotnetSdk.Xdr.XdrDataInputStream(Convert.FromBase64String(xdrString));
        var xdr = StellarDotnetSdk.Xdr.SCVal.Decode(result_xdr);
        foreach (var i in xdr.Vec.InnerValue)
        {
            string Asset = "";
            ulong IdleAmount = 0;
            ulong InvestedAmount = 0;
            ulong TotalAmount = 0;
            List<StrategyAllocation> StrategyAllocations = new List<StrategyAllocation>();
            foreach (var j in xdr.Vec.InnerValue[0].Map.InnerValue)
            {
                switch (j.Key.Sym.InnerValue)
                {
                    case "asset":
                        var contractId = (SCContractId)SCContractId.FromSCValXdr(j.Val);
                        Asset = contractId.InnerValue;
                        break;
                    case "idle_amount":
                        var idleAmount = (SCInt128)SCInt128.FromSCValXdr(j.Val);
                        IdleAmount = idleAmount.Lo;
                        break;
                    case "invested_amount":
                        var investedAmount = (SCInt128)SCInt128.FromSCValXdr(j.Val);
                        InvestedAmount = investedAmount.Lo;
                        break;
                    case "total_amount":
                        var totalAmount = (SCInt128)SCInt128.FromSCValXdr(j.Val);
                        TotalAmount = totalAmount.Lo;
                        break;
                    case "strategy_allocations":
                        var strategyAllocations = (SCVec)SCVec.FromSCValXdr(j.Val);
                        foreach (var strategyAllocation in strategyAllocations.InnerValue)
                        {
                            var strategyAllocationsMap = (SCMap)SCMap.FromSCValXdr(strategyAllocation.ToXdr());
                            ulong Amount = 0;
                            bool Paused = false;
                            string StrategyAddress = "";
                            foreach (var entry in strategyAllocationsMap.Entries)
                            {
                                var key = (SCSymbol)SCSymbol.FromSCValXdr(entry.Key.ToXdr());
                                switch (key.InnerValue)
                                {
                                    case "amount":
                                        var amount = (SCInt128)SCInt128.FromSCValXdr(entry.Value.ToXdr());
                                        Amount = amount.Lo;
                                        break;
                                    case "paused":
                                        var paused = (SCBool)SCBool.FromSCValXdr(entry.Value.ToXdr());
                                        Paused = paused.InnerValue;
                                        break;
                                    case "strategy_address":
                                        var strategyAddress = (SCContractId)SCContractId.FromSCValXdr(entry.Value.ToXdr());
                                        StrategyAddress = strategyAddress.InnerValue;
                                        break;
                                }
                            }
                            StrategyAllocations.Add(new StrategyAllocation(Amount, Paused, StrategyAddress));
                        }
                        break;
                }
            }
            var result = new ManagedFundsResult(Asset, IdleAmount, InvestedAmount, TotalAmount, StrategyAllocations);
            managedFundsResults.Add(result);
        }
        return managedFundsResults;
    }
    public static List<TransactionResult> ParseSubmittedTransaction(SCVal result, string txHash)
    {
        List<ulong> Amounts = new List<ulong>();
        ulong SharesChanged = 0;
        var res = result.ToXdr().Vec.InnerValue;
        if (res.Length == 3)
        {
            var depositedAmounts = res[0];
            SharesChanged = res[1].I128.Lo.InnerValue;
            foreach (var depositedAmount in depositedAmounts.Vec.InnerValue)
            {
                var amount = (SCInt128)SCInt128.FromSCValXdr(depositedAmount);
                Amounts.Add(amount.Lo);
            }
        }
        else if (res.Length == 1)
        {
            foreach (var i in res)
            {
                var amount = (SCInt128)SCInt128.FromSCValXdr(i);
                SharesChanged += amount.Lo;
                Amounts.Add(amount.Lo);
            }
        }
        else
        {
            Console.WriteLine("Unexpected number of results received.");
            return new List<TransactionResult>();
        }
        var response = new TransactionResult(true, txHash, Amounts, SharesChanged);
        return new List<TransactionResult> { response };
    }
  
    public static PoolConfig? ParsePoolConfigResult(SimulateTransactionResponse response)
    {
        if (response.Results == null || response.Results.Length == 0)
        {
            Console.WriteLine("No results found in SimulateTransactionResponse for PoolConfig.");
            return null;
        }

        var xdrString = response.Results[0].Xdr;

        if (string.IsNullOrEmpty(xdrString))
        {
            Console.WriteLine("XDR string for PoolConfig is null or empty.");
            return null;
        }

        try
        {
            var scVal = StellarDotnetSdk.Soroban.SCVal.FromXdrBase64(xdrString);

            if (scVal is not SCMap poolConfigMap)
            {
                Console.WriteLine("Expected SCMap for PoolConfig but received different type.");
                return null;
            }

            uint bStopRate = 0;
            uint maxPositions = 0;
            ulong minCollateral = 0;
            string oracleAddress = string.Empty;
            uint status = 0;

            foreach (var entry in poolConfigMap.Entries)
            {
                if (entry.Key is not SCSymbol keySymbol) continue;

                switch (keySymbol.InnerValue)
                {
                    case "bstop_rate":
                        if (entry.Value is SCUint32 valBStopRate)
                            bStopRate = valBStopRate.InnerValue;
                        break;
                    case "max_positions":
                        if (entry.Value is SCUint32 valMaxPositions)
                            maxPositions = valMaxPositions.InnerValue;
                        break;
                    case "min_collateral":
                        if (entry.Value is SCInt128 valMinCollateral)
                        {
                            var tempCollateral = (SCInt128)SCInt128.FromSCValXdr(entry.Value.ToXdr());
                            minCollateral = tempCollateral.Lo;
                        }
                        break;
                    case "oracle":
                        if (entry.Value is SCAddress valOracleAddress)
                        {
                            var tempOracleAddress = (SCContractId)SCContractId.FromSCValXdr(entry.Value.ToXdr());
                            oracleAddress = tempOracleAddress.InnerValue;
                        }
                        else if (entry.Value is SCContractId valOracleContractId)
                             oracleAddress = valOracleContractId.InnerValue;
                        break;
                    case "status":
                        if (entry.Value is SCUint32 valStatus)
                            status = valStatus.InnerValue;
                        break;
                }
            }
            return new PoolConfig(bStopRate, maxPositions, minCollateral, oracleAddress, status);
        }
        catch (Exception ex)
        {
            Console.WriteLine($"Error parsing PoolConfig: {ex.Message}");
            return null;
        }
    }

    public static Reserve? ParseReserveResult(SimulateTransactionResponse response)
    {
        if (response.Results == null || response.Results.Length == 0)
        {
            Console.WriteLine("No results found in SimulateTransactionResponse for Reserve.");
            return null;
        }

        var xdrString = response.Results[0].Xdr;
        if (string.IsNullOrEmpty(xdrString))
        {
            Console.WriteLine("XDR string for Reserve is null or empty.");
            return null;
        }

        try
        {
            var scVal = StellarDotnetSdk.Soroban.SCVal.FromXdrBase64(xdrString);
            if (scVal is not SCMap reserveMap)
            {
                Console.WriteLine("Expected SCMap for Reserve but received different type.");
                return null;
            }

            string asset = string.Empty;
            ReserveConfig? config = null;
            ReserveData? data = null;
            SCInt128 scalar = new SCInt128(0, 0);

            foreach (var entry in reserveMap.Entries)
            {
                if (entry.Key is not SCSymbol keySymbol) continue;
                switch (keySymbol.InnerValue)
                {
                    case "asset":
                        if (entry.Value is SCAddress scAddressValue){
                            var tempAddress = (SCContractId)SCContractId.FromSCValXdr(entry.Value.ToXdr());
                            asset = tempAddress.InnerValue;
                        }
                        else if (entry.Value is SCContractId scContractIdValue)
                            asset = scContractIdValue.InnerValue;
                        break;
                    case "config":
                        if (entry.Value is SCMap configMap)
                            config = ParseReserveConfigMap(configMap);
                        break;
                    case "data":
                        if (entry.Value is SCMap dataMap)
                            data = ParseReserveDataMap(dataMap);
                        break;
                    case "scalar":
                        if (entry.Value is SCInt128 scalarVal)
                            scalar = scalarVal;
                        break;
                }
            }

            if (string.IsNullOrEmpty(asset) || config == null || data == null)
            {
                Console.WriteLine($"Failed to parse all required fields for Reserve. Asset: {!string.IsNullOrEmpty(asset)}, Config: {config != null}, Data: {data != null}");
                return null;
            }

            return new Reserve(asset, config, data, scalar);
        }
        catch (Exception ex)
        {
            Console.WriteLine($"Error parsing Reserve: {ex.Message}");
            return null;
        }
    }

    private static ReserveConfig? ParseReserveConfigMap(SCMap configMap)
    {
        uint cFactor = 0;
        uint decimals = 0;
        bool enabled = false;
        uint index = 0;
        uint lFactor = 0;
        uint maxUtil = 0;
        uint rBase = 0;
        uint rOne = 0;
        uint rThree = 0;
        uint rTwo = 0;
        uint reactivity = 0;
        SCInt128 supplyCap = new SCInt128(0, 0);
        uint util = 0;

        foreach (var entry in configMap.Entries)
        {
            if (entry.Key is not SCSymbol keySymbol) continue;
            switch (keySymbol.InnerValue)
            {
                case "c_factor":
                    if (entry.Value is SCUint32 valCFactor) cFactor = valCFactor.InnerValue;
                    break;
                case "decimals":
                    if (entry.Value is SCUint32 valDecimals) decimals = valDecimals.InnerValue;
                    break;
                case "enabled":
                    if (entry.Value is SCBool valEnabled) enabled = valEnabled.InnerValue;
                    break;
                case "index":
                    if (entry.Value is SCUint32 valIndex) index = valIndex.InnerValue;
                    break;
                case "l_factor":
                    if (entry.Value is SCUint32 valLFactor) lFactor = valLFactor.InnerValue;
                    break;
                case "max_util":
                    if (entry.Value is SCUint32 valMaxUtil) maxUtil = valMaxUtil.InnerValue;
                    break;
                case "r_base":
                    if (entry.Value is SCUint32 valRBase) rBase = valRBase.InnerValue;
                    break;
                case "r_one":
                    if (entry.Value is SCUint32 valROne) rOne = valROne.InnerValue;
                    break;
                case "r_three":
                    if (entry.Value is SCUint32 valRThree) rThree = valRThree.InnerValue;
                    break;
                case "r_two":
                    if (entry.Value is SCUint32 valRTwo) rTwo = valRTwo.InnerValue;
                    break;
                case "reactivity":
                    if (entry.Value is SCUint32 valReactivity) reactivity = valReactivity.InnerValue;
                    break;
                case "supply_cap":
                    if (entry.Value is SCInt128 valSupplyCap) supplyCap = valSupplyCap;
                    break;
                case "util":
                    if (entry.Value is SCUint32 valUtil) util = valUtil.InnerValue;
                    break;
            }
        }
        return new ReserveConfig(cFactor, decimals, enabled, index, lFactor, maxUtil, rBase, rOne, rThree, rTwo, reactivity, supplyCap, util);
    }

    private static ReserveData? ParseReserveDataMap(SCMap dataMap)
    {
        SCInt128 bRate = new SCInt128(0, 0);
        SCInt128 bSupply = new SCInt128(0, 0);
        SCInt128 backstopCredit = new SCInt128(0, 0);
        SCInt128 dRate = new SCInt128(0, 0);
        SCInt128 dSupply = new SCInt128(0, 0);
        SCInt128 irMod = new SCInt128(0, 0);
        ulong lastTime = 0;

        foreach (var entry in dataMap.Entries)
        {
            if (entry.Key is not SCSymbol keySymbol) continue;
            switch (keySymbol.InnerValue)
            {
                case "b_rate":
                    if (entry.Value is SCInt128 valBRate) bRate = valBRate;
                    break;
                case "b_supply":
                    if (entry.Value is SCInt128 valBSupply) bSupply = valBSupply;
                    break;
                case "backstop_credit":
                    if (entry.Value is SCInt128 valBackstopCredit) backstopCredit = valBackstopCredit;
                    break;
                case "d_rate":
                    if (entry.Value is SCInt128 valDRate) dRate = valDRate;
                    break;
                case "d_supply":
                    if (entry.Value is SCInt128 valDSupply) dSupply = valDSupply;
                    break;
                case "ir_mod":
                    if (entry.Value is SCInt128 valIrMod) irMod = valIrMod;
                    break;
                case "last_time":
                    if (entry.Value is SCUint64 valLastTime) lastTime = valLastTime.InnerValue;
                    break;
            }
        }
        return new ReserveData(bRate, bSupply, backstopCredit, dRate, dSupply, irMod, lastTime);
    }

    public static ReserveEmissionData? ParseReserveEmissionData(SimulateTransactionResponse response)
    {
        if (response.Results == null || response.Results.Length == 0)
        {
            Console.WriteLine("No results found in SimulateTransactionResponse for ReserveEmissionData.");
            return null;
        }

        var xdrString = response.Results[0].Xdr;
        if (string.IsNullOrEmpty(xdrString))
        {
            Console.WriteLine("XDR string for ReserveEmissionData is null or empty.");
            return null;
        }

        try
        {
            var scVal = StellarDotnetSdk.Soroban.SCVal.FromXdrBase64(xdrString);
            if (scVal is not SCMap emissionMap)
            {
                Console.WriteLine("Expected SCMap for ReserveEmissionData but received different type.");
                return null;
            }

            ulong eps = 0;
            ulong expiration = 0;
            SCInt128 index = new SCInt128(0, 0);
            ulong lastTime = 0;

            foreach (var entry in emissionMap.Entries)
            {
                if (entry.Key is not SCSymbol keySymbol) continue;
                switch (keySymbol.InnerValue)
                {
                    case "eps":
                        if (entry.Value is SCUint64 valEps) eps = valEps.InnerValue;
                        break;
                    case "expiration":
                        if (entry.Value is SCUint64 valExpiration) expiration = valExpiration.InnerValue;
                        break;
                    case "index":
                        if (entry.Value is SCInt128 valIndex) index = valIndex;
                        break;
                    case "last_time":
                        if (entry.Value is SCUint64 valLastTime) lastTime = valLastTime.InnerValue;
                        break;
                }
            }
            return new ReserveEmissionData(new SCUint64(eps), expiration, index, lastTime);
        }
        catch (Exception ex)
        {
            Console.WriteLine($"Error parsing ReserveEmissionData: {ex.Message}");
            return null;
        }
    }
}