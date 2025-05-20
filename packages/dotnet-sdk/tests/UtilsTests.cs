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
                Oracle = "CD74A3C54EKUVEGUC6WNTUPOTHB624WFKXN3IYTFJGX3EHXDXHCYMXXR",
                MinCollateral = 50000000, // 0.05 token with 7 decimals
                BstopRate = 2000000, // 20% in 7 decimals
                Status = 1, // Active
                MaxPositions = 6
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
                DRate = 1008674205479, // Updated from 1007254005472
                BRate = 1005739211313, // Updated from 1004999864177
                IrMod = 11064275, // Updated from 8325672
                BSupply = 1903776551256, // Updated from 12913989438990
                DSupply = 1687391443495, // Updated from 11615633968856
                BackstopCredit = 699805978, // Updated from 1503385393
                LastTime = 1747730608 // Updated from 1747728348
            };

            var assetReserves = (2724548742322L, 2415996815711L); // Updated from (1000000000L, 1000000000L)
            var blndReserves = (502L, 24012L); // Updated from (1000000000L, 1000000000L)

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
                Oracle = "CD74A3C54EKUVEGUC6WNTUPOTHB624WFKXN3IYTFJGX3EHXDXHCYMXXR",
                MinCollateral = 50000000,
                BstopRate = 2000000,
                Status = 1,
                MaxPositions = 6
            };
            var reserveData = new ReserveData
            {
                DRate = 1008674205479,
                BRate = 1005739211313,
                IrMod = 11064275,
                BSupply = 1903776551256,
                DSupply = 1687391443495,
                BackstopCredit = 699805978,
                LastTime = 1747730608
            };
            var assetReserves = (2724548742322L, 2415996815711L); // Updated from (1000000000L, 1000000000L)
            var blndReserves = (502L, 24012L); // Updated from (1000000000L, 1000000000L)

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
            var assetReserves = (2724548742322L, 2415996815711L); // Updated from (1000000000L, 1000000000L)
            var blndReserves = (502L, 24012L); // Updated from (1000000000L, 1000000000L)

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