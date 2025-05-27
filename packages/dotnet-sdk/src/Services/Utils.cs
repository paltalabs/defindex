using System;
using System.Numerics;
using System.Collections.Generic;
using DeFindex.Sdk.Interfaces;
using StellarDotnetSdk.Responses.SorobanRpc;
using Paltalabs.Defindex.Services;
using StellarDotnetSdk;
using StellarDotnetSdk.Soroban;
using Newtonsoft.Json;

namespace DeFindex.Sdk.Services
{
    public static class Utils
    {
        public readonly static SoroswapRouter router = new SoroswapRouter();
        public const uint BPS = 10000;
        const long IR_MOD_SCALAR = 10000000;
        const long SCALAR_7 = 10000000;
        public const string BLND = "CD25MNVTZDL4Y3XBCPCJXGXATV5WUHHOWMYFF4YBEGU5FCPGMYTVG5JY"; // BLND token address
        /// <summary>
        /// Calculates the APY (Annual Percentage Yield) for a pool based on various parameters
        /// </summary>
        /// <param name="poolConfigDict">Dictionary of pool configurations</param>
        /// <param name="reserveEmissionsDict">Dictionary of reserve emission data</param>
        /// <param name="reserveDict">Dictionary of reserve </param>
        /// <param name="managedFunds">The total managed funds for the vault</param>
        /// <returns>The calculated APY as a decimal value</returns>
        public static decimal calculateAPY(
            Dictionary<string, PoolConfig> poolConfigDict,
            Dictionary<string, ReserveEmissionData> reserveEmissionsDict,
            Dictionary<string, Reserve> reserveDict,
            ManagedFundsResult managedFunds)
        {

            // TODO: Implement the actual APY calculation logic using managedFunds
            return 0.0m;
        }

        public static decimal calculateAssetAPY(
            Dictionary<string, PoolConfig> poolConfigDict,
            Dictionary<string, ReserveEmissionData> reserveEmissionsDict,
            Dictionary<string, Reserve> reserveDict,
            ManagedFundsResult managedFunds,
            Dictionary<string, BigInteger> pairReserves,
            uint vaultFeeBps
        )
        {
            BigInteger investedSum = 0;
            foreach (var strategy in managedFunds.StrategyAllocations)
            {
                Console.WriteLine($"Strategy Address: {strategy.StrategyAddress}");
                var assetAddress = managedFunds.Asset;
                var supplyApy = calculateSupplyAPY(
                    reserveDict[strategy.StrategyAddress],
                    poolConfigDict[strategy.StrategyAddress]
                );
                // TODO: Implement the Emissions APY
                var emmissionsAPR = calculateEmissionsAPR(
                    reserveEmissionsDict[strategy.StrategyAddress],
                    reserveDict[strategy.StrategyAddress].Data,
                    pairReserves,
                    assetAddress
                );
                var emmisionsAPY = calculateEmissionsAPY(
                    emmissionsAPR
                );
                var bigSupplyApy = (BigInteger)(SCALAR_7 * supplyApy);
                var bigEmmisionsAPY = (BigInteger)(SCALAR_7 * emmisionsAPY);
                var supplyApyWithFee = bigSupplyApy + bigEmmisionsAPY * (BPS - vaultFeeBps) / BPS;
                investedSum = investedSum + strategy.Amount * (SCALAR_7 + (BigInteger)supplyApyWithFee) / SCALAR_7;

            }
            var numerator = managedFunds.IdleAmount + investedSum;
            Console.WriteLine($"Managed Funds Total Amount: {managedFunds.TotalAmount}");
            Console.WriteLine($"Numerator: {numerator}");
            var result = (decimal)numerator / (decimal)managedFunds.TotalAmount  - 1.0m;

            return result;
        }

        public static decimal calculateSupplyAPR(
            Reserve reserve,
            PoolConfig poolConfig
        )
        {
            var curUtil = getUtilization(reserve.Config, reserve.Data);
            if (curUtil == 0)
            {
                return 0.0m;
            }
            var targetUtil = new BigInteger(reserve.Config.Util);
            var fixed_95_percent = new BigInteger(9500000);
            var fixed_5_percent = new BigInteger(500000);

            BigInteger curIr = 0;

            if (curUtil <= targetUtil)
            {
                var utilScalar = curUtil * SCALAR_7 / targetUtil;
                var baseRate = (utilScalar * reserve.Config.ROne) / SCALAR_7 + reserve.Config.RBase;
                curIr = baseRate * reserve.Data.IrMod / IR_MOD_SCALAR;
            }
            else if (curUtil <= fixed_95_percent)
            {
                var utilScalar = DivCeil(curUtil - targetUtil, fixed_95_percent - targetUtil, SCALAR_7);
                var baseRate = MulCeil(utilScalar, reserve.Config.RTwo, SCALAR_7) + reserve.Config.ROne + reserve.Config.RBase;
                curIr = MulCeil(baseRate, reserve.Data.IrMod, IR_MOD_SCALAR);
            }
            else
            {
                var utilScalar = DivCeil(curUtil - fixed_95_percent, fixed_5_percent, SCALAR_7);
                var extraRate = MulCeil(utilScalar, reserve.Config.RThree, SCALAR_7);
                var intersection = MulCeil(
                    reserve.Data.IrMod,
                    reserve.Config.RTwo + reserve.Config.ROne + reserve.Config.RBase,
                    IR_MOD_SCALAR
                );
                curIr = extraRate + intersection;
            }

            var supplyCapture = (SCALAR_7 - poolConfig.BStopRate) * curUtil / SCALAR_7;
            Console.WriteLine($"Current Utilization: {curUtil}");
            Console.WriteLine($"Current Interest Rate: {curIr}");
            Console.WriteLine($"Supply Capture: {supplyCapture}");
            
            var supplyApr = (decimal)curIr * (decimal)supplyCapture / (decimal)SCALAR_7 / (decimal)SCALAR_7;
            Console.WriteLine($"Supply APR: {supplyApr}");
            // Convert supplyApr to decimal, assuming supplyApr is in 7 decimals fixed-point
            return supplyApr;
        }
        public static decimal aprToApy(decimal apr)
        {
            return (decimal)Math.Pow(1 + (double)apr / 52, 52) - 1;
        }

        public static decimal calculateSupplyAPY(
            Reserve reserve,
            PoolConfig poolConfig
        )
        {
            return aprToApy(
                calculateSupplyAPR(
                    reserve,
                    poolConfig
                )
            );
        }

        // Helper for fixed-point division with ceiling
        private static BigInteger DivCeil(BigInteger a, BigInteger b, BigInteger scalar)
        {
            if (b == 0) return 0;
            return (a * scalar + b - 1) / b;
        }

        // Helper for fixed-point multiplication with ceiling
        private static BigInteger MulCeil(BigInteger a, BigInteger b, BigInteger scalar)
        {
            return (a * b + scalar - 1) / scalar;
        }

        /// <summary>
        /// Calculates the Emissions APR (Annual Percentage Rate) for a pool based on emission parameters
        /// </summary>
        /// <param name="reserveEmissionData">The emission data for the reserve tokens</param>
        /// <param name="reserveData">The data for the reserve asset</param>
        /// <param name="pairReserves">The reserves for the asset pair (underliying[token0], BLND[token1])</param>
        /// <returns>The calculated Emissions APR as a decimal value</returns>
        public static decimal calculateEmissionsAPR(
            ReserveEmissionData reserveEmissionData,
            ReserveData blendReserveData,
            Dictionary<string, BigInteger> pairReserves,
            string AssetAddress)
        {
            // Formula:Total emissions per year = EPS*Seconds in a year / Supply
            // EmissionsAPR = Total emissions per year * PrecioBLND/Precio UnderlyingAsset
            // EmissionsAPR = Total emissions per year *ReserveUnderlyingAsset/ReserveBLND
            // TODO: Implement the emissions APR calculation logic
            // supply = toAssetFromBToken( Reserve.Data.BSupply)
            BigInteger secondsInYear = (365 * 24 * 60 * 60);
            Console.WriteLine($"🟡pairReserve {JsonConvert.SerializeObject(pairReserves, Formatting.Indented)}");
            var num = reserveEmissionData.Eps * secondsInYear * pairReserves[AssetAddress];
            var denom = blendReserveData.BSupply * pairReserves[BLND];
            var totalEmissionsPerYear = num / denom;

            var emmissionsAPR = (decimal)totalEmissionsPerYear;
            if (emmissionsAPR > 0)
            {
            Console.WriteLine($"Emissions APR: {emmissionsAPR}");
                return emmissionsAPR;
            }
            return emmissionsAPR;
        }

        public static decimal calculateEmissionsAPY(
            decimal emissionsAPR
        )
        {
            // Anualize the emissions APR per day.
            double baseValue = 1 + (double)emissionsAPR / SCALAR_7 / 365;
            double powResult = Math.Pow(baseValue, 365) - 1;
            if (double.IsInfinity(powResult) || powResult > (double)decimal.MaxValue)
            {
                return decimal.MaxValue;
            }
            if (powResult < (double)decimal.MinValue)
            {
                return decimal.MinValue;
            }
            Console.WriteLine($"Emissions APY: {powResult}");
            return (decimal)powResult;
        }

        public static BigInteger getUtilization(
            ReserveConfig reserveConfig,
            ReserveData reserveData
        )
        {
            var totalSupplyVar = totalSupply(reserveData, reserveConfig);
            if (totalSupplyVar == 0)
            {
                return 0;
            }
            var totalLiabilitiesVar = totalLiabilities(reserveData, reserveConfig);
            return (totalLiabilitiesVar *
            (BigInteger)Math.Pow(10, reserveConfig.Decimals) - 1) / totalSupplyVar + 1;

        }

        public static BigInteger totalSupply(
            ReserveData reserveData,
            ReserveConfig reserveConfig
        )
        {
            return toAssetFromBToken(reserveData.BSupply, reserveData, reserveConfig);
        }

        public static BigInteger totalLiabilities(
            ReserveData reserveData,
            ReserveConfig reserveConfig
        )
        {
            return toAssetFromDToken(reserveData.DSupply, reserveData);
        }

        public static BigInteger toAssetFromBToken(
            BigInteger bTokenAmount,
            ReserveData reserveData,
            ReserveConfig reserveConfig
        )
        {
            if (bTokenAmount == 0)
            {
                return 0;
            }
            return bTokenAmount * reserveData.BRate / (new BigInteger(Math.Pow(10, reserveConfig.Decimals)));
        }

        public static BigInteger toAssetFromDToken(
            BigInteger dTokenAmount,
            ReserveData reserveData
        )
        {
            if (dTokenAmount == 0)
            {
                return 0;
            }
            return dTokenAmount * reserveData.DRate / (new BigInteger(Math.Pow(10, 7))); // Using 7 decimals as seen in the test data
        }
    }
} 