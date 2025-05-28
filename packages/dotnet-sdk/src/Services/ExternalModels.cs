using System;
using System.Numerics;
using StellarDotnetSdk.Soroban;

namespace DeFindex.Sdk.Services
{
    public class PoolConfig
    {
        public string OracleAddress { get; set; }
        public ulong MinCollateral { get; set; }
        public uint BStopRate { get; set; }
        public uint Status { get; set; }
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

    public class ReserveConfig
    {
        public uint CFactor { get; set; }
        public uint Decimals { get; set; }
        public bool Enabled { get; set; }
        public uint Index { get; set; }
        public uint LFactor { get; set; }
        public uint MaxUtil { get; set; }
        public uint RBase { get; set; }
        public uint ROne { get; set; }
        public uint RThree { get; set; }
        public uint RTwo { get; set; }
        public uint Reactivity { get; set; }
        public BigInteger SupplyCap { get; set; }
        public uint Util { get; set; }

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
        public string Asset { get; set; }
        public ReserveConfig? Config { get; set; }
        public ReserveData? Data { get; set; }
        public BigInteger Scalar { get; set; }

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