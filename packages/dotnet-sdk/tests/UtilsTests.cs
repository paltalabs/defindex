using Xunit;
using DeFindex.Sdk.Services;
using DeFindex.Sdk.Interfaces;
using System;
using System.Collections.Generic;

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
            DRate = 1008674205479,
            BRate = 1005739211313,
            IrMod = 11064275,
            BSupply = 1903776551256,
            DSupply = 1687391443495,
            BackstopCredit = 699805978,
            LastTime = 1747730608
        };

        private static readonly (long, long) DefaultAssetReserves = (2724548742322L, 2415996815711L);
        private static readonly (long, long) DefaultBlndReserves = (502L, 24012L);

        private static ReserveEmissionData CreateDefaultReserveEmissionData()
        {
            return new ReserveEmissionData
            {
                Expiration = DateTimeOffset.UtcNow.AddDays(365).ToUnixTimeSeconds(),
                Eps = 1000000, // 1 token per second with 7 decimals
                Index = 1000000000, // 1 token with 7 decimals
                LastTime = DateTimeOffset.UtcNow.ToUnixTimeSeconds()
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

        [Fact]
        public void CalculateEmissionsAPR_ReturnsExpectedValue()
        {
            // Arrange
            var reserveEmissionData = CreateDefaultReserveEmissionData();

            // Act
            var result = Utils.calculateEmissionsAPR(
                reserveEmissionData,
                DefaultAssetReserves,
                DefaultBlndReserves
            );

            // Assert
            Assert.Equal(0.0m, result);
        }
    }
} 