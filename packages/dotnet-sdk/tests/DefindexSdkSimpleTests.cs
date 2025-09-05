using Xunit;
using DeFindex.Sdk.Services;
using StellarDotnetSdk.Soroban;
using System;

namespace DeFindex.Sdk.Tests
{
    /// <summary>
    /// Simple basic tests that ensure the SDK can be instantiated and basic validation works
    /// </summary>
    public class DefindexSdkSimpleTests
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
    }
}