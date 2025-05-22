using System;
using System.Numerics;
using System.Collections.Generic;
using DeFindex.Sdk.Interfaces;
using StellarDotnetSdk.Responses.SorobanRpc;

namespace DeFindex.Sdk.Services
{
    public static class Utils
    {
        /// <summary>
        /// Calculates the APY (Annual Percentage Yield) for a pool based on various parameters
        /// </summary>
        /// <param name="poolConfigDict">Dictionary of pool configurations</param>
        /// <param name="reserveEmissionsDict">Dictionary of reserve emission data</param>
        /// <param name="reserveDataDict">Dictionary of reserve data</param>
        /// <param name="managedFunds">The total managed funds for the vault</param>
        /// <returns>The calculated APY as a decimal value</returns>
        public static decimal calculateAPY(
            Dictionary<string, PoolConfig> poolConfigDict,
            Dictionary<string, ReserveEmissionData> reserveEmissionsDict,
            Dictionary<string, ReserveData> reserveDataDict,
            ManagedFundsResult managedFunds)
        {
            // TODO: Implement the actual APY calculation logic using managedFunds
            return 0.0m;
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

        public static ulong getUtilization(
            PoolConfig poolConfig,
            ReserveData reserveData
        )
        {
            // TODO: Implement the utilization calculation logic
            return 0;
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