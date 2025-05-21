using System;
using System.Numerics;
using StellarDotnetSdk.Soroban;

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
    /// Represents the configuration for a reserve
    /// </summary>
    public class ReserveConfig
    {
        public uint CFactor { get; }
        public uint Decimals { get; }
        public bool Enabled { get; }
        public uint Index { get; }
        public uint LFactor { get; }
        public uint MaxUtil { get; }
        public uint RBase { get; }
        public uint ROne { get; }
        public uint RThree { get; }
        public uint RTwo { get; }
        public uint Reactivity { get; }
        public BigInteger SupplyCap { get; }
        public uint Util { get; }

        public ReserveConfig(uint cFactor, uint decimals, bool enabled, uint index, uint lFactor, uint maxUtil, uint rBase, uint rOne, uint rThree, uint rTwo, uint reactivity, BigInteger supplyCap, uint util)
        {
            CFactor = cFactor;
            Decimals = decimals;
            Enabled = enabled;
            Index = index;
            LFactor = lFactor;
            MaxUtil = maxUtil;
            RBase = rBase;
            ROne = rOne;
            RThree = rThree;
            RTwo = rTwo;
            Reactivity = reactivity;
            SupplyCap = supplyCap;
            Util = util;
        }
    }

    /// <summary>
    /// Represents a reserve in the system
    /// </summary>
    public class Reserve
    {
        public string Asset { get; }
        public ReserveConfig? Config { get; }
        public ReserveData? Data { get; }
        public BigInteger Scalar { get; }

        public Reserve(string asset, ReserveConfig? config, ReserveData? data, BigInteger scalar)
        {
            Asset = asset;
            Config = config;
            Data = data;
            Scalar = scalar;
        }
    }

    /// <summary>
    /// Represents the data for a reserve asset
    /// </summary>
    public class ReserveData
    {
        public BigInteger BRate { get; set; }
        public BigInteger BSupply { get; set; }
        public BigInteger BackstopCredit { get; set; }
        public BigInteger DRate { get; set; }
        public BigInteger DSupply { get; set; }
        public BigInteger IrMod { get; set; }
        public ulong LastTime { get; set; }

        public ReserveData()
        {
            BRate = BigInteger.Zero;
            BSupply = BigInteger.Zero;
            BackstopCredit = BigInteger.Zero;
            DRate = BigInteger.Zero;
            DSupply = BigInteger.Zero;
            IrMod = BigInteger.Zero;
            LastTime = 0;
        }

        public ReserveData(BigInteger bRate, BigInteger bSupply, BigInteger backstopCredit, BigInteger dRate, BigInteger dSupply, BigInteger irMod, ulong lastTime)
        {
            BRate = bRate;
            BSupply = bSupply;
            BackstopCredit = backstopCredit;
            DRate = dRate;
            DSupply = dSupply;
            IrMod = irMod;
            LastTime = lastTime;
        }
    }

    /// <summary>
    /// Represents the emission data for the reserve b or d token
    /// </summary>
    public class ReserveEmissionData
    {

        public ulong Eps { get; set; }
        public ulong Expiration { get; set; }
        public BigInteger Index { get; set; }
        public ulong LastTime { get; set; }

        public ReserveEmissionData()
        {
            Eps = 0;
            Expiration = 0;
            Index = BigInteger.Zero;
            LastTime = 0;
        }

        public ReserveEmissionData(ulong eps, ulong expiration, BigInteger index, ulong lastTime)
        {
            Eps = eps;
            Expiration = expiration;
            Index = index;
            LastTime = lastTime;
        }
    }
} 