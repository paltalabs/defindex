using Xunit;
using DeFindex.Sdk.Services;
using StellarDotnetSdk.Soroban;
using StellarDotnetSdk.Accounts;
using StellarDotnetSdk;
using System.Collections.Generic;
using System.Threading.Tasks;
using System.Diagnostics;
using System.Numerics;
using System;
using System.Linq;
using DotNetEnv;

namespace DeFindex.Sdk.Tests
{
    [Collection("Performance")]
    public class DefindexSdkPerformanceTests : IClassFixture<TestEnvironmentFixture>
    {
        private readonly TestEnvironmentFixture _fixture;
        private readonly DefindexSdk _defindexSdk;

        public DefindexSdkPerformanceTests(TestEnvironmentFixture fixture)
        {
            _fixture = fixture;
            var sorobanServer = new SorobanServer(_fixture.TestnetUrl);
            _defindexSdk = new DefindexSdk(_fixture.VaultAddress, sorobanServer);
            
            // Ensure test account is funded before running performance tests
            _fixture.EnsureAccountFunded(_fixture.TestUserKeypair).Wait();
        }

        [Fact]
        [Trait("Category", "Performance")]
        public async Task GetVaultFee_Performance_ShouldCompleteWithinTimeout()
        {
            // Arrange
            var stopwatch = Stopwatch.StartNew();
            var maxExecutionTime = TimeSpan.FromSeconds(30); // Max 30 seconds for network call

            // Act
            var result = await _defindexSdk.GetVaultFee();
            stopwatch.Stop();

            // Assert
            Assert.NotNull(result);
            Assert.True(stopwatch.Elapsed < maxExecutionTime, 
                $"GetVaultFee took {stopwatch.Elapsed.TotalSeconds} seconds, expected less than {maxExecutionTime.TotalSeconds}");
        }

        [Fact]
        [Trait("Category", "Performance")]
        public async Task GetVaultTotalShares_Performance_ShouldCompleteWithinTimeout()
        {
            // Arrange
            var stopwatch = Stopwatch.StartNew();
            var maxExecutionTime = TimeSpan.FromSeconds(30);

            // Act
            var result = await _defindexSdk.GetVaultTotalShares();
            stopwatch.Stop();

            // Assert
            Assert.True(result >= 0);
            Assert.True(stopwatch.Elapsed < maxExecutionTime,
                $"GetVaultTotalShares took {stopwatch.Elapsed.TotalSeconds} seconds, expected less than {maxExecutionTime.TotalSeconds}");
        }

        [Fact]
        [Trait("Category", "Performance")]
        public async Task CreateDepositTransaction_Performance_ShouldCompleteWithinTimeout()
        {
            // Arrange
            var stopwatch = Stopwatch.StartNew();
            var maxExecutionTime = TimeSpan.FromSeconds(10); // Transaction creation should be fast
            var amountsDesired = new List<ulong> { 1000000 };
            var amountsMin = new List<ulong> { 900000 };
            var from = _fixture.TestUserKeypair.AccountId;

            // Act
            var result = await _defindexSdk.CreateDepositTransaction(amountsDesired, amountsMin, from, false);
            stopwatch.Stop();

            // Assert
            Assert.NotNull(result);
            Assert.True(stopwatch.Elapsed < maxExecutionTime,
                $"CreateDepositTransaction took {stopwatch.Elapsed.TotalSeconds} seconds, expected less than {maxExecutionTime.TotalSeconds}");
        }

        [Fact]
        [Trait("Category", "Performance")]
        public async Task MultipleOperations_Performance_ShouldMaintainPerformance()
        {
            // Arrange
            var iterations = 5;
            var maxAverageTime = TimeSpan.FromSeconds(15); // Average time per operation
            var totalTime = Stopwatch.StartNew();
            var individualTimes = new List<TimeSpan>();

            // Act
            for (int i = 0; i < iterations; i++)
            {
                var operationTime = Stopwatch.StartNew();
                
                var fee = await _defindexSdk.GetVaultFee();
                var shares = await _defindexSdk.GetVaultTotalShares();
                var userShares = await _defindexSdk.GetUserShares(_fixture.TestUserKeypair.AccountId);
                
                operationTime.Stop();
                individualTimes.Add(operationTime.Elapsed);
            }
            
            totalTime.Stop();

            // Assert
            var averageTime = TimeSpan.FromTicks(individualTimes.Select(t => t.Ticks).Sum() / iterations);
            
            Assert.True(averageTime < maxAverageTime,
                $"Average operation time was {averageTime.TotalSeconds} seconds, expected less than {maxAverageTime.TotalSeconds}");
            
            // Ensure no significant performance degradation across iterations
            var firstHalf = individualTimes.Take(iterations / 2).Average(t => t.TotalSeconds);
            var secondHalf = individualTimes.Skip(iterations / 2).Average(t => t.TotalSeconds);
            var performanceDegradationRatio = secondHalf / firstHalf;
            
            Assert.True(performanceDegradationRatio < 2.0,
                $"Performance degraded by {performanceDegradationRatio:F2}x from first half to second half");
        }

        [Fact]
        [Trait("Category", "Performance")]
        public async Task ConcurrentOperations_Performance_ShouldHandleConcurrency()
        {
            // Arrange
            var concurrentOperations = 3;
            var maxExecutionTime = TimeSpan.FromSeconds(60); // Allow more time for concurrent operations
            var stopwatch = Stopwatch.StartNew();

            // Act
            var tasks = new List<Task>();
            
            for (int i = 0; i < concurrentOperations; i++)
            {
                tasks.Add(Task.Run(async () =>
                {
                    var fee = await _defindexSdk.GetVaultFee();
                    var shares = await _defindexSdk.GetVaultTotalShares();
                    var userShares = await _defindexSdk.GetUserShares(_fixture.TestUserKeypair.AccountId);
                    
                    Assert.NotNull(fee);
                    Assert.True(shares >= 0);
                    Assert.NotNull(userShares);
                }));
            }

            await Task.WhenAll(tasks);
            stopwatch.Stop();

            // Assert
            Assert.True(stopwatch.Elapsed < maxExecutionTime,
                $"Concurrent operations took {stopwatch.Elapsed.TotalSeconds} seconds, expected less than {maxExecutionTime.TotalSeconds}");
        }

        [Fact]
        [Trait("Category", "Performance")]
        public async Task TransactionCreation_Performance_BulkOperations()
        {
            // Arrange
            var transactionCount = 10;
            var maxAverageCreationTime = TimeSpan.FromSeconds(5); // 5 seconds average per transaction creation
            var creationTimes = new List<TimeSpan>();
            var userAddress = _fixture.TestUserKeypair.AccountId;

            // Act
            for (int i = 0; i < transactionCount; i++)
            {
                var stopwatch = Stopwatch.StartNew();
                
                var amountsDesired = new List<ulong> { (ulong)(1000000 + i * 100000) }; // Vary amounts slightly
                var amountsMin = new List<ulong> { (ulong)(900000 + i * 90000) };
                
                var transaction = await _defindexSdk.CreateDepositTransaction(amountsDesired, amountsMin, userAddress, false);
                
                stopwatch.Stop();
                creationTimes.Add(stopwatch.Elapsed);
                
                Assert.NotNull(transaction);
            }

            // Assert
            var averageCreationTime = TimeSpan.FromTicks(creationTimes.Select(t => t.Ticks).Sum() / transactionCount);
            
            Assert.True(averageCreationTime < maxAverageCreationTime,
                $"Average transaction creation time was {averageCreationTime.TotalSeconds} seconds, expected less than {maxAverageCreationTime.TotalSeconds}");
            
            // Ensure no transaction took unreasonably long
            var maxIndividualTime = creationTimes.Max();
            var maxAllowedIndividualTime = TimeSpan.FromSeconds(15);
            
            Assert.True(maxIndividualTime < maxAllowedIndividualTime,
                $"Longest individual transaction creation took {maxIndividualTime.TotalSeconds} seconds, expected less than {maxAllowedIndividualTime.TotalSeconds}");
        }

        [Fact]
        [Trait("Category", "Performance")]
        public async Task MemoryUsage_Performance_ShouldNotLeakMemory()
        {
            // Arrange
            var iterations = 20;
            var initialMemory = GC.GetTotalMemory(true);
            var userAddress = _fixture.TestUserKeypair.AccountId;

            // Act
            for (int i = 0; i < iterations; i++)
            {
                var fee = await _defindexSdk.GetVaultFee();
                var shares = await _defindexSdk.GetVaultTotalShares();
                var userShares = await _defindexSdk.GetUserShares(userAddress);
                var managedFunds = await _defindexSdk.FetchTotalManagedFunds();
                
                // Force garbage collection every few iterations
                if (i % 5 == 0)
                {
                    GC.Collect();
                    GC.WaitForPendingFinalizers();
                    GC.Collect();
                }
            }

            // Final garbage collection
            GC.Collect();
            GC.WaitForPendingFinalizers();
            GC.Collect();
            
            var finalMemory = GC.GetTotalMemory(false);
            var memoryIncrease = finalMemory - initialMemory;
            var maxAllowedIncrease = 50 * 1024 * 1024; // 50MB

            // Assert
            Assert.True(memoryIncrease < maxAllowedIncrease,
                $"Memory increased by {memoryIncrease / (1024 * 1024)} MB, expected less than {maxAllowedIncrease / (1024 * 1024)} MB");
        }
    }
}