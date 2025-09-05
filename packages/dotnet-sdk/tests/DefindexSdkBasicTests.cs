using Xunit;
using DeFindex.Sdk.Services;
using StellarDotnetSdk.Soroban;
using StellarDotnetSdk.Accounts;
using System;
using System.Collections.Generic;
using System.Numerics;
using System.Threading.Tasks;

namespace DeFindex.Sdk.Tests
{
    /// <summary>
    /// Basic unit tests for DefindexSdk that don't require network access or complex mocking
    /// </summary>
    public class DefindexSdkBasicTests
    {
        private readonly string _testVaultAddress = "CAQTQIWPQUID3Z4KK5FNS3DG752KJBNZWAAQXPYZOHCWNJUPM43IDWUB";

        [Fact]
        public void Constructor_ValidParameters_ShouldCreateInstance()
        {
            // Arrange
            var sorobanServer = new SorobanServer("https://soroban-testnet.stellar.org");

            // Act
            var sdk = new DefindexSdk(_testVaultAddress, sorobanServer);

            // Assert
            Assert.NotNull(sdk);
            Assert.Equal(_testVaultAddress, sdk.ContractId);
            Assert.Equal(sorobanServer, sdk.Server);
            Assert.NotNull(sdk.Helpers);
            Assert.NotNull(sdk.Router);
        }

        [Theory]
        [InlineData("")]
        [InlineData("INVALID_ADDRESS")]
        public void Constructor_InvalidVaultAddress_ShouldThrowException(string invalidAddress)
        {
            // Arrange
            var sorobanServer = new SorobanServer("https://soroban-testnet.stellar.org");

            // Act & Assert
            Assert.Throws<ArgumentException>(() => new DefindexSdk(invalidAddress, sorobanServer));
        }

        [Fact]
        public void Constructor_NullSorobanServer_ShouldThrowException()
        {
            // Act & Assert
            Assert.Throws<ArgumentNullException>(() => new DefindexSdk(_testVaultAddress, null!));
        }

        [Fact]
        public void InitializeAsync_ShouldReturnTrue()
        {
            // Arrange
            var sorobanServer = new SorobanServer("https://soroban-testnet.stellar.org");
            var sdk = new DefindexSdk(_testVaultAddress, sorobanServer);

            // Act
            var result = sdk.InitializeAsync();

            // Assert
            Assert.True(result);
        }

        [Theory]
        [InlineData(0)]
        [InlineData(-1)]
        public async Task CreateWithdrawUnderlyingTx_InvalidWithdrawAmount_ShouldThrowException(long amount)
        {
            // Arrange
            var sorobanServer = new SorobanServer("https://soroban-testnet.stellar.org");
            var sdk = new DefindexSdk(_testVaultAddress, sorobanServer);
            BigInteger withdrawAmount = new BigInteger(amount);
            int toleranceBps = 100;
            var from = KeyPair.Random().AccountId;

            // Act & Assert
            await Assert.ThrowsAsync<ArgumentException>(() => 
                sdk.CreateWithdrawUnderlyingTx(withdrawAmount, toleranceBps, from));
        }

        [Theory]
        [InlineData(10001)]
        [InlineData(20000)]
        public async Task CreateWithdrawUnderlyingTx_InvalidToleranceBps_ShouldThrowException(int tolerance)
        {
            // Arrange
            var sorobanServer = new SorobanServer("https://soroban-testnet.stellar.org");
            var sdk = new DefindexSdk(_testVaultAddress, sorobanServer);
            BigInteger withdrawAmount = 10000000;
            var from = KeyPair.Random().AccountId;

            // Act & Assert
            await Assert.ThrowsAsync<ArgumentOutOfRangeException>(() => 
                sdk.CreateWithdrawUnderlyingTx(withdrawAmount, tolerance, from));
        }

        [Theory]
        [InlineData(0)]
        [InlineData(-1)]
        public async Task GetAssetAmountsPerShares_InvalidVaultShares_ShouldThrowException(long sharesAmount)
        {
            // Arrange
            var sorobanServer = new SorobanServer("https://soroban-testnet.stellar.org");
            var sdk = new DefindexSdk(_testVaultAddress, sorobanServer);
            BigInteger vaultShares = new BigInteger(sharesAmount);

            // Act & Assert
            await Assert.ThrowsAsync<ArgumentException>(() => 
                sdk.GetAssetAmountsPerShares(vaultShares));
        }
    }
}