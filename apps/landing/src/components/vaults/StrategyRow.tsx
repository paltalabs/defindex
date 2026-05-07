"use client";

import Image from "next/image";
import type { StrategyApySnapshot } from "@/types/vault.types";
import { stroopsToNum, fmtTVL, fmtUsd, fmtApy, truncateAddress, formatStrategyName } from "@/utils/vaultFormatters";
import { getTokenInfo } from "@/lib/tokenIcons";
import TypeBadge from "./TypeBadge";
import { useState } from "react";

interface StrategyRowProps {
    strategy: StrategyApySnapshot & { paused?: boolean };
}

export default function StrategyRow({ strategy }: StrategyRowProps) {
    const [isHovered, setIsHovered] = useState(false);

    // TOKEN_ICONS is the source of truth: look up by contract address, then fall back to
    // 'native' if the API reports the symbol as 'XLM' (Stellar native XLM may not use key 'native').
    const tokenInfo =
        getTokenInfo(strategy.asset) ??
        (strategy.assetSymbol === 'XLM' ? getTokenInfo('native') : null);
    const tokenIcon = tokenInfo?.icon ?? null;
    const displayAssetSymbol = tokenInfo?.symbol ?? strategy.assetSymbol;
    const tvlNum = stroopsToNum(strategy.tvl, strategy.assetDecimals);
    const shortAddress = truncateAddress(strategy.address, 4);
    const displayName = formatStrategyName(strategy.name);
    const isIdle = strategy.apy7d == null;

    return (
        <div
            onMouseEnter={() => setIsHovered(true)}
            onMouseLeave={() => setIsHovered(false)}
            style={{
                display: "flex",
                alignItems: "center",
                gap: 16,
                padding: "22px 28px",
                borderTop: "1px solid rgba(193,200,201,.07)",
                background: isHovered ? "rgba(217,249,157,.03)" : "transparent",
                transition: "background 200ms cubic-bezier(0.4,0,0.2,1)",
                cursor: "pointer",
                opacity: strategy.paused ? 0.6 : 1,
            }}
        >
            {/* Strategy name + address */}
            <div style={{ flex: "0 0 320px", minWidth: 0 }}>
                <div style={{ display: "flex", alignItems: "center", gap: 8, flexWrap: "wrap" }}>
                    <span
                        style={{
                            fontSize: 14.5,
                            fontWeight: 600,
                            color: "#fff",
                            letterSpacing: "-0.01em",
                            fontFamily: "ui-monospace, SFMono-Regular, Menlo, monospace",
                            overflow: "hidden",
                            textOverflow: "ellipsis",
                            whiteSpace: "nowrap",
                        }}
                    >
                        {displayName}
                    </span>
                    {strategy.paused && (
                        <span
                            style={{
                                fontSize: 10,
                                fontWeight: 700,
                                padding: "2px 6px",
                                borderRadius: 4,
                                background: "rgba(252,91,49,.15)",
                                color: "#FC5B31",
                                textTransform: "uppercase",
                                letterSpacing: "0.04em",
                                flexShrink: 0,
                            }}
                        >
                            paused
                        </span>
                    )}
                </div>
                <div
                    style={{
                        fontSize: 11.5,
                        color: "rgba(255,255,255,.4)",
                        marginTop: 3,
                        fontFamily: "ui-monospace, SFMono-Regular, Menlo, monospace",
                    }}
                >
                    {shortAddress}
                </div>
            </div>

            {/* Type badge */}
            <div style={{ flex: "0 0 120px" }}>
                <TypeBadge type={strategy.type} />
            </div>

            {/* Asset */}
            <div style={{ flex: "0 0 140px", display: "flex", alignItems: "center", gap: 10 }}>
                {tokenIcon ? (
                    <Image
                        src={tokenIcon}
                        alt={displayAssetSymbol}
                        width={22}
                        height={22}
                        className="rounded-full flex-shrink-0"
                        unoptimized
                    />
                ) : (
                    <div
                        style={{
                            width: 22,
                            height: 22,
                            borderRadius: "50%",
                            background: "rgba(255,255,255,.12)",
                            flexShrink: 0,
                        }}
                    />
                )}
                <span style={{ fontSize: 14, fontWeight: 500, color: "#fff" }}>
                    {displayAssetSymbol}
                </span>
            </div>

            {/* TVL */}
            <div style={{ flex: "0 0 180px" }}>
                <div
                    style={{
                        fontSize: 15,
                        fontWeight: 600,
                        color: "#fff",
                        fontFeatureSettings: '"tnum"',
                    }}
                >
                    {fmtTVL(tvlNum, displayAssetSymbol)}
                </div>
                <div style={{ fontSize: 12, color: "rgba(255,255,255,.45)", marginTop: 2 }}>
                    {fmtUsd(tvlNum)}
                </div>
            </div>

            {/* APY 7d */}
            <div style={{ flex: 1, textAlign: "right" }}>
                <div
                    style={{
                        fontFamily: "Familjen Grotesk, sans-serif",
                        fontSize: 18,
                        fontWeight: 700,
                        color: isIdle ? "rgba(255,255,255,.35)" : "#D9F99D",
                        fontFeatureSettings: '"tnum"',
                        letterSpacing: "-0.01em",
                    }}
                >
                    {fmtApy(strategy.apy7d)}
                </div>
            </div>
        </div>
    );
}
