using Xunit;
using DeFindex.Sdk.Services;
using DeFindex.Sdk.Interfaces;
using System;
using System.Collections.Generic;
using StellarDotnetSdk.Soroban;
using System.Numerics;

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

        private static readonly ReserveConfig DefaultReserveConfig = new ReserveConfig
        (
            1, // uint CFactor
            7, // uint Decimals
            true, // bool Enabled
            1, // uint Index
            1, // uint LFactor
            1, // uint MaxUtil
            1, // uint RBase
            1, // uint ROne
            1, // uint RThree
            1, // uint RTwo
            1, // uint Reactivity
            1, // BigInteger SupplyCap
            1 // uint Util
        );

        private static readonly ReserveData DefaultReserveData = new ReserveData
        {
            DRate = new BigInteger(1008674205479),
            BRate = new BigInteger(1005739211313),
            IrMod = new BigInteger(11064275),
            BSupply = new BigInteger(1903776551256),
            DSupply = new BigInteger(1687391443495),
            BackstopCredit = new BigInteger(699805978),
            LastTime = 1747730608UL
        };

        private static readonly Reserve DefaultReserve = new Reserve(
            "mock_asset",
            DefaultReserveConfig,
            DefaultReserveData,
            10000000
        );

        private static readonly (BigInteger, BigInteger) DefaultAssetReserves = (new BigInteger(2724548742322), new BigInteger(2415996815711));
        private static readonly (BigInteger, BigInteger) DefaultBlndReserves = (new BigInteger(502), new BigInteger(24012));

        private static ReserveEmissionData CreateDefaultReserveEmissionData()
        {
            return new ReserveEmissionData
            {
                Expiration = (ulong)DateTimeOffset.UtcNow.AddDays(365).ToUnixTimeSeconds(),
                Eps = 1000000, // 1 token per second with 7 decimals
                Index = 1000000000, // 1 token with 7 decimals
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
        public void CalculateSupplyAPY_ReturnsExpectedValue()
        {
            Console.WriteLine("--------------------------------------------------");
            Console.WriteLine("---------------------LOGGING----------------------");
            Console.WriteLine("--------------------------------------------------");
            // Arrange
            var poolConfigDict = new Dictionary<string, PoolConfig>
            {
                { "test_pool", DefaultPoolConfig }
            };

            var reserveDict = new Dictionary<string, Reserve>
            {
                { "test_pool", DefaultReserve }
            };

            var toAssetFromBTokenResult = Utils.toAssetFromBToken(
                reserveDict["test_pool"].Data.BSupply, 
                reserveDict["test_pool"].Data, 
                reserveDict["test_pool"].Config);

            Console.WriteLine(toAssetFromBTokenResult.ToString());
            Console.WriteLine(toAssetFromBTokenResult/(new BigInteger(Math.Pow(10,17))));
            Console.WriteLine(toAssetFromBTokenResult/(new BigInteger(Math.Pow(10,3))));
            // We verify that the order of magnitud is correct
            // On calc i got 1.91470272717639E+017
            var amountToCheck=toAssetFromBTokenResult/(new BigInteger(Math.Pow(10,3)));
            Assert.True((amountToCheck)==191470272717639,$"Failed check of 13 first number, it was {amountToCheck}");
            Assert.True((toAssetFromBTokenResult/(new BigInteger(Math.Pow(10,17)))) == 1, $"it failed with {toAssetFromBTokenResult}");
            
            // Act
            // var result = Utils.calculateSupplyAPY(
            //     poolConfigDict,
            //     reserveDict
            // );

            // Assert
            // Assert.Equal(0.0m, result);
            // Assert that result is between 9 and 10
            // Assert.True(result >= 9.0m && result <= 10.0m, $"Expected result to be between 9 and 10, but got {result}");
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