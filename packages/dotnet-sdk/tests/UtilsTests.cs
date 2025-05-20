using Xunit;
using DeFindex.Sdk.Services;
using DeFindex.Sdk.Interfaces;
using System;
using System.Collections.Generic;

namespace DeFindex.Sdk.Tests
{
    public class UtilsTests
    {
        [Fact]
        public void CalculateAPY_ReturnsExpectedValue()
        {
            // Arrange
            var poolConfig = new PoolConfig
            {
                Oracle = "GAAZI4TCR3TY5OJHCTJC2A4QSY6CJWJH5IAJTGKIN2ER7LBNVKOCCWN7",
                MinCollateral = 1000000000, // 1 token with 7 decimals
                BstopRate = 500000, // 5% in 7 decimals
                Status = 1, // Active
                MaxPositions = 100
            };

            var reserveEmissionData = new ReserveEmissionData
            {
                Expiration = DateTimeOffset.UtcNow.AddDays(365).ToUnixTimeSeconds(),
                Eps = 1000000, // 1 token per second with 7 decimals
                Index = 1000000000, // 1 token with 7 decimals
                LastTime = DateTimeOffset.UtcNow.ToUnixTimeSeconds()
            };

            var reserveData = new ReserveData
            {
                DRate = 1000000000000, // 1:1 with 12 decimals
                BRate = 1000000000000, // 1:1 with 12 decimals
                IrMod = 10000000, // 1 with 7 decimals
                BSupply = 1000000000, // 1 token with 7 decimals
                DSupply = 1000000000, // 1 token with 7 decimals
                BackstopCredit = 0,
                LastTime = DateTimeOffset.UtcNow.ToUnixTimeSeconds()
            };

            var assetReserves = (1000000000L, 1000000000L); // (1 token, 1 token) with 7 decimals
            var blndReserves = (1000000000L, 1000000000L); // (1 token, 1 token) with 7 decimals

            var managedFunds = new ManagedFundsResult(
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

            // Act
            var result = Utils.calculateAPY(
                poolConfig,
                reserveEmissionData,
                reserveData,
                assetReserves,
                blndReserves,
                managedFunds
            );

            // Assert
            Assert.Equal(0.0m, result); // This will need to be updated once the actual APY calculation is implemented
        }

        [Fact]
        public void CalculateSupplyAPY_ReturnsExpectedValue()
        {
            // Arrange
            var poolConfig = new PoolConfig
            {
                Oracle = "GAAZI4TCR3TY5OJHCTJC2A4QSY6CJWJH5IAJTGKIN2ER7LBNVKOCCWN7",
                MinCollateral = 1000000000,
                BstopRate = 500000,
                Status = 1,
                MaxPositions = 100
            };
            var reserveData = new ReserveData
            {
                DRate = 1000000000000,
                BRate = 1000000000000,
                IrMod = 10000000,
                BSupply = 1000000000,
                DSupply = 1000000000,
                BackstopCredit = 0,
                LastTime = DateTimeOffset.UtcNow.ToUnixTimeSeconds()
            };
            var assetReserves = (1000000000L, 1000000000L);
            var blndReserves = (1000000000L, 1000000000L);

            // Act
            var result = Utils.calculateSupplyAPY(
                poolConfig,
                reserveData
            );

            // Assert
            Assert.Equal(0.0m, result);
        }

        [Fact]
        public void CalculateEmissionsAPR_ReturnsExpectedValue()
        {
            // Arrange
            var reserveEmissionData = new ReserveEmissionData
            {
                Expiration = DateTimeOffset.UtcNow.AddDays(365).ToUnixTimeSeconds(),
                Eps = 1000000,
                Index = 1000000000,
                LastTime = DateTimeOffset.UtcNow.ToUnixTimeSeconds()
            };
            var assetReserves = (1000000000L, 1000000000L);
            var blndReserves = (1000000000L, 1000000000L);

            // Act
            var result = Utils.calculateEmissionsAPR(
                reserveEmissionData,
                assetReserves,
                blndReserves
            );

            // Assert
            Assert.Equal(0.0m, result);
        }
    }
} 