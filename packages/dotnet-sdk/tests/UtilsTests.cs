using Xunit;
using DeFindex.Sdk.Services;
using DeFindex.Sdk.Interfaces;
using System;
using System.Collections.Generic;
using StellarDotnetSdk.Soroban;

namespace DeFindex.Sdk.Tests
{
    public class UtilsTests
    {
        // Shared test data
        private static readonly PoolConfig DefaultPoolConfig = new PoolConfig
        (
            // BStopRate
            2000000, // 20% in 7 decimals
            // MaxPositions
            6,
            // MinCollateral
            50000000, // 0.05 token with 7 decimals
            // OracleAddress
            "CD74A3C54EKUVEGUC6WNTUPOTHB624WFKXN3IYTFJGX3EHXDXHCYMXXR",
            // Status
            1 // Active
        );

        private static readonly ReserveData DefaultReserveData = new ReserveData
        {
            DRate = new SCInt128("1008674205479"),
            BRate = new SCInt128("1005739211313"),
            IrMod = new SCInt128("11064275"),
            BSupply = new SCInt128("1903776551256"),
            DSupply = new SCInt128("1687391443495"),
            BackstopCredit = new SCInt128("699805978"),
            LastTime = 1747730608UL
        };

        private static readonly (SCInt128, SCInt128) DefaultAssetReserves = (new SCInt128("2724548742322"), new SCInt128("2415996815711"));
        private static readonly (SCInt128, SCInt128) DefaultBlndReserves = (new SCInt128("502"), new SCInt128("24012"));

        private static ReserveEmissionData CreateDefaultReserveEmissionData()
        {
            return new ReserveEmissionData
            {
                Expiration = (ulong)DateTimeOffset.UtcNow.AddDays(365).ToUnixTimeSeconds(),
                Eps = new SCUint64(1000000), // 1 token per second with 7 decimals
                Index = new SCInt128("1000000000"), // 1 token with 7 decimals
                LastTime = (ulong)DateTimeOffset.UtcNow.ToUnixTimeSeconds()
            };
        }

        private static ManagedFundsResult CreateDefaultManagedFunds()
        {
            return new ManagedFundsResult(
                Asset: "USDC",
                IdleAmount: 1000000000, // 1 token with 7 decimals
                InvestedAmount: 2000000000, // 2 tokens with 7 decimals
                TotalAmount: 3000000000, // 3 tokens with 7 decimals
                StrategyAllocations: new List<StrategyAllocation>
                {
                    new StrategyAllocation(
                        Amount: 1000000000, // 1 token with 7 decimals
                        Paused: false,
                        StrategyAddress: "GAAZI4TCR3TY5OJHCTJC2A4QSY6CJWJH5IAJTGKIN2ER7LBNVKOCCWN7"
                    )
                }
            );
        }

        [Fact]
        public void CalculateAPY_ReturnsExpectedValue()
        {
            // Arrange
            var poolConfigDict = new Dictionary<string, PoolConfig>
            {
                { "test_pool", DefaultPoolConfig }
            };

            var reserveEmissionsDict = new Dictionary<string, ReserveEmissionData>
            {
                { "test_pool", CreateDefaultReserveEmissionData() }
            };

            var reserveDataDict = new Dictionary<string, ReserveData>
            {
                { "test_pool", DefaultReserveData }
            };

            var managedFunds = CreateDefaultManagedFunds();

            // Act
            var result = Utils.calculateAPY(
                poolConfigDict,
                reserveEmissionsDict,
                reserveDataDict,
                managedFunds
            );

            // Assert
            Assert.Equal(0.0m, result); // This will need to be updated once the actual APY calculation is implemented
        }

        [Fact]
        public void CalculateSupplyAPY_ReturnsExpectedValue()
        {
            // Arrange
            var poolConfigDict = new Dictionary<string, PoolConfig>
            {
                { "test_pool", DefaultPoolConfig }
            };

            var reserveDataDict = new Dictionary<string, ReserveData>
            {
                { "test_pool", DefaultReserveData }
            };

            // Act
            var result = Utils.calculateSupplyAPY(
                poolConfigDict,
                reserveDataDict
            );

            // Assert
            Assert.Equal(0.0m, result);
        }

        // [Fact]
        // public void CalculateEmissionsAPR_ReturnsExpectedValue()
        // {
        //     // Arrange
        //     var reserveEmissionData = CreateDefaultReserveEmissionData();

        //     // Act
        //     var result = Utils.calculateEmissionsAPR(
        //         reserveEmissionData,
        //         DefaultAssetReserves,
        //         DefaultBlndReserves
        //     );

        //     // Assert
        //     Assert.Equal(0.0m, result);
        // }
    }
} 