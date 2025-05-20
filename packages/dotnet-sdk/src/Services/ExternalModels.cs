using System;

namespace DeFindex.Sdk.Services
{
    /// <summary>
    /// Represents the pool's configuration
    /// </summary>
    public class PoolConfig
    {
        /// <summary>
        /// The contract address of the oracle
        /// </summary>
        public string OracleAddress { get; set; }

        /// <summary>
        /// The minimum amount of collateral required to open a liability position
        /// </summary>
        public ulong MinCollateral { get; set; }

        /// <summary>
        /// The rate the backstop takes on accrued debt interest, expressed in 7 decimals
        /// </summary>
        public uint BStopRate { get; set; }

        /// <summary>
        /// The status of the pool
        /// </summary>
        public uint Status { get; set; }

        /// <summary>
        /// The maximum number of effective positions a single user can hold
        /// </summary>
        public uint MaxPositions { get; set; }

        public PoolConfig(uint bStopRate, uint maxPositions, ulong minCollateral, string oracleAddress, uint status)
        {
            BStopRate = bStopRate;
            MaxPositions = maxPositions;
            MinCollateral = minCollateral;
            OracleAddress = oracleAddress;
            Status = status;
        }
    }

    /// <summary>
    /// Represents the emission data for the reserve b or d token
    /// </summary>
    public class ReserveEmissionData
    {
        /// <summary>
        /// The expiration timestamp of the emission
        /// </summary>
        public long Expiration { get; set; }

        /// <summary>
        /// The emission per second rate
        /// </summary>
        public ulong Eps { get; set; }

        /// <summary>
        /// The current emission index
        /// </summary>
        public long Index { get; set; }

        /// <summary>
        /// The last time the emission data was updated
        /// </summary>
        public long LastTime { get; set; }
    }

    /// <summary>
    /// Represents the data for a reserve asset
    /// </summary>
    public class ReserveData
    {
        /// <summary>
        /// The conversion rate from dToken to underlying with 12 decimals
        /// </summary>
        public long DRate { get; set; }

        /// <summary>
        /// The conversion rate from bToken to underlying with 12 decimals
        /// </summary>
        public long BRate { get; set; }

        /// <summary>
        /// The interest rate curve modifier with 7 decimals
        /// </summary>
        public long IrMod { get; set; }

        /// <summary>
        /// The total supply of b tokens, in the underlying token's decimals
        /// </summary>
        public long BSupply { get; set; }

        /// <summary>
        /// The total supply of d tokens, in the underlying token's decimals
        /// </summary>
        public long DSupply { get; set; }

        /// <summary>
        /// The amount of underlying tokens currently owed to the backstop
        /// </summary>
        public long BackstopCredit { get; set; }

        /// <summary>
        /// The last time the reserve data was updated
        /// </summary>
        public long LastTime { get; set; }
    }
} 