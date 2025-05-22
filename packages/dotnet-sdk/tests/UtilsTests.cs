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
            9500000, // uint CFactor
            7, // uint Decimals
            true, // bool Enabled
            1, // uint Index
            9500000, // uint LFactor
            9500000, // uint MaxUtil
            300000, // uint RBase
            400000, // uint ROne
            50000000, // uint RThree
            1200000, // uint RTwo
            20, // uint Reactivity
            BigInteger.Parse("2000000000000000"), // BigInteger SupplyCap
            8000000 // uint Util
        );

        private static readonly ReserveData DefaultReserveData = new ReserveData
        {
            BRate = new BigInteger(1006465757461),
            BSupply = new BigInteger(12851732605704),
            BackstopCredit = new BigInteger(56783860),
            DRate = new BigInteger(1009204687675),
            DSupply = new BigInteger(12097835563259),
            IrMod = new BigInteger(8744173),
            LastTime = 1747913623UL
        };

        private static readonly Reserve DefaultReserve = new Reserve(
            "CCW67TSZV3SSS2HXMBQ5JFGCKJNXKZM7UQUWUZPUTHXSTZLEO7SJMI75",
            DefaultReserveConfig,
            DefaultReserveData,
            new BigInteger(10000000)
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

            // Console.WriteLine(toAssetFromBTokenResult.ToString());
            // Console.WriteLine(toAssetFromBTokenResult/(new BigInteger(Math.Pow(10,17))));
            // Console.WriteLine(toAssetFromBTokenResult/(new BigInteger(Math.Pow(10,3))));
            // On calc i got 1.91470272717639E+017
            var amountToCheck=toAssetFromBTokenResult/(new BigInteger(Math.Pow(10,4)));
            Assert.True((amountToCheck)==129348287916861,$"Failed check of 13 first number, it was {amountToCheck}");
            // // We verify that the order of magnitud is correct
            Assert.True((toAssetFromBTokenResult/(new BigInteger(Math.Pow(10,18)))) == 1, $"it failed with {toAssetFromBTokenResult}");
            
            var totalSupplyResult = Utils.totalSupply(
                reserveDict["test_pool"].Data,
                reserveDict["test_pool"].Config
            );
            // Console.WriteLine(totalSupplyResult.ToString());

            Assert.Equal(totalSupplyResult, toAssetFromBTokenResult);

            var toAssetFromDTokenResult = Utils.toAssetFromDToken(
                reserveDict["test_pool"].Data.DSupply,
                reserveDict["test_pool"].Data
            );
            // Console.WriteLine(toAssetFromDTokenResult.ToString());
            Assert.True(toAssetFromDTokenResult/(new BigInteger(Math.Pow(10,18)))== 1 ,$"Magnitud incorrect, with {toAssetFromDTokenResult/(new BigInteger(Math.Pow(10,18)))}");

            var totalLiabilitiesResult = Utils.totalLiabilities(
                reserveDict["test_pool"].Data,
                reserveDict["test_pool"].Config
            );
            // Console.WriteLine(totalLiabilitiesResult.ToString());
            Assert.Equal(totalLiabilitiesResult, toAssetFromDTokenResult);
            
            var getUtilizationResult = Utils.getUtilization(
                DefaultReserve.Config,
                DefaultReserveData
            );
            // Console.WriteLine(getUtilizationResult.ToString());
            Assert.True(getUtilizationResult == 9439006
, $"Failed to check getUtilization, it was {getUtilizationResult}");

            var strategyApr = Utils.calculateSupplyAPR(
                DefaultReserve,
                DefaultPoolConfig
            );
            Assert.True(strategyApr > 0, $"StrategyAPR: {strategyApr}");
            Assert.True(strategyApr <1, $"StrategyAPR: {strategyApr}");

            var strategyApy = Utils.aprToApy(strategyApr);
            Assert.True(strategyApy == (decimal)0.12985563048252, $"Failed apy with {strategyApy}");
            
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

        // [Fact]
        // public void CalculateAPY_ReturnsExpectedValue()
        // {
        //     // Arrange
        //     var poolConfigDict = new Dictionary<string, PoolConfig>
        //     {
        //         { "test_pool", DefaultPoolConfig }
        //     };

        //     var reserveEmissionsDict = new Dictionary<string, ReserveEmissionData>
        //     {
        //         { "test_pool", CreateDefaultReserveEmissionData() }
        //     };

        //     var reserveDataDict = new Dictionary<string, ReserveData>
        //     {
        //         { "test_pool", DefaultReserveData }
        //     };

        //     var managedFunds = CreateDefaultManagedFunds();

        //     // Act
        //     var result = Utils.calculateAPY(
        //         poolConfigDict,
        //         reserveEmissionsDict,
        //         reserveDataDict,
        //         managedFunds
        //     );

        //     // Assert
        //     Assert.Equal(0.0m, result); // This will need to be updated once the actual APY calculation is implemented
        // }

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