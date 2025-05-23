using System;
using System.Numerics;
using System.Collections.Generic;
using DeFindex.Sdk.Interfaces;
using StellarDotnetSdk.Responses.SorobanRpc;

namespace DeFindex.Sdk.Services
{
    public static class Utils
    {
        public const uint BPS = 10000;
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
            uint vaultFeeBps)
        {
            decimal investedSum = 0;
            foreach (var strategy in managedFunds.StrategyAllocations){
                var supplyApy = calculateSupplyAPY(
                    reserveDict[strategy.StrategyAddress],
                    poolConfigDict[strategy.StrategyAddress]
                );
                var supplyApyWithFee = supplyApy*(BPS-vaultFeeBps)/BPS;
                investedSum=investedSum+strategy.Amount*(1+supplyApyWithFee);

            }
            var numerator = managedFunds.IdleAmount+investedSum;
            return numerator/managedFunds.TotalAmount-1;
        }


        /// <summary>
        /// Calculates the Supply APY (Annual Percentage Yield) for a pool based on supply parameters
        /// </summary>
        /// <param name="poolConfigDict">Dictionary of pool configurations</param>
        /// <param name="reserveDataDict">Dictionary of reserve data</param>
        /// <returns>The calculated Supply APY as a decimal value</returns>
        public static decimal calculateSupplyAPY(
            Dictionary<string, PoolConfig> poolConfigDict,
            Dictionary<string, Reserve> reserveDict
        )
        {
            // TODO: Implement the supply APY calculation logic
            return 0.0m;
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
            const long IR_MOD_SCALAR = 10000000;
            const long SCALAR_7 = 10000000;
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

            var supplyCapture = (SCALAR_7- poolConfig.BStopRate)*curUtil/SCALAR_7;

            var supplyApr = (decimal)curIr * (decimal)supplyCapture / (decimal)SCALAR_7 / (decimal)SCALAR_7;
            // Convert supplyApr to decimal, assuming supplyApr is in 7 decimals fixed-point
            return supplyApr;
        }
        public static decimal aprToApy(decimal apr)
        {
            return (decimal)Math.Pow(1 + (double)apr/52, 52) - 1;
        }

        public static decimal calculateSupplyAPY(
            Reserve reserve,
            PoolConfig poolConfig
        ){
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
        /// <param name="poolConfig">The pool's configuration</param>
        /// <param name="reserveEmissionData">The emission data for the reserve tokens</param>
        /// <param name="reserveData">The data for the reserve asset</param>
        /// <param name="assetReserves">The reserves for the asset (token0, token1)</param>
        /// <param name="blndReserves">The reserves for BLND token (token0, token1)</param>
        /// <returns>The calculated Emissions APR as a decimal value</returns>
        public static decimal calculateEmissionsAPR(
            ReserveEmissionData reserveEmissionData,
            (long, long) assetReserves,
            (long, long) blndReserves)
        {
            // TODO: Implement the emissions APR calculation logic
            return 0.0m;
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
            return (totalLiabilitiesVar*
            (BigInteger)Math.Pow(10,reserveConfig.Decimals)-1) / totalSupplyVar+1;

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
            return bTokenAmount *reserveData.BRate / (new BigInteger(Math.Pow(10,reserveConfig.Decimals)));
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