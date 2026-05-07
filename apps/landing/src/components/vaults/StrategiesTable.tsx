"use client";

import { useMemo } from "react";
import { useStrategies } from "@/hooks/useStrategies";
import StrategyRow from "./StrategyRow";
import { HeaderCell, GLASS_CARD } from "./TableCard";
import type { SortKey } from "@/types/vault.types";

interface StrategiesTableProps {
    search?: string;
    sort?: SortKey;
}

function SkeletonRow({ index }: { index: number }) {
    return (
        <div
            style={{
                display: "flex",
                alignItems: "center",
                gap: 16,
                padding: "22px 28px",
                borderTop: "1px solid rgba(193,200,201,.07)",
                opacity: 0.6,
                animation: `pulse 1.4s linear ${index * 0.1}s infinite`,
            }}
        >
            <div style={{ flex: "0 0 320px" }}>
                <div style={{ height: 14, width: 180, background: "rgba(255,255,255,.08)", borderRadius: 4, marginBottom: 6 }} />
                <div style={{ height: 11, width: 100, background: "rgba(255,255,255,.06)", borderRadius: 4 }} />
            </div>
            <div style={{ flex: "0 0 120px" }}>
                <div style={{ height: 20, width: 55, background: "rgba(255,255,255,.08)", borderRadius: 999 }} />
            </div>
            <div style={{ flex: "0 0 140px", display: "flex", alignItems: "center", gap: 8 }}>
                <div style={{ width: 22, height: 22, borderRadius: "50%", background: "rgba(255,255,255,.08)" }} />
                <div style={{ height: 13, width: 40, background: "rgba(255,255,255,.08)", borderRadius: 4 }} />
            </div>
            <div style={{ flex: "0 0 180px" }}>
                <div style={{ height: 14, width: 80, background: "rgba(255,255,255,.08)", borderRadius: 4, marginBottom: 6 }} />
                <div style={{ height: 11, width: 50, background: "rgba(255,255,255,.06)", borderRadius: 4 }} />
            </div>
            <div style={{ flex: "0 0 100px" }}>
                <div style={{ height: 24, width: 80, background: "rgba(255,255,255,.06)", borderRadius: 4 }} />
            </div>
            <div style={{ flex: 1, display: "flex", justifyContent: "flex-end" }}>
                <div style={{ height: 18, width: 55, background: "rgba(255,255,255,.08)", borderRadius: 4 }} />
            </div>
        </div>
    );
}


export default function StrategiesTable({ search = "", sort = "TVL" }: StrategiesTableProps) {
    const { strategies, isLoading, error } = useStrategies();

    const filtered = useMemo(() => {
        const q = search.toLowerCase();
        const result = q
            ? strategies.filter(s =>
                [s.name, s.assetSymbol, s.type].some(f =>
                    f.toLowerCase().includes(q)
                )
            )
            : [...strategies];

        return result.sort((a, b) => {
            if (sort === "TVL") return Number(b.tvl) - Number(a.tvl);
            if (sort === "APY") return (b.apy7d ?? 0) - (a.apy7d ?? 0);
            if (sort === "Name") return a.name.localeCompare(b.name);
            return 0;
        });
    }, [strategies, search, sort]);

    return (
        <div style={GLASS_CARD}>
            {/* Table header */}
            <div
                style={{
                    display: "flex",
                    alignItems: "center",
                    gap: 16,
                    padding: "16px 28px",
                    borderBottom: "1px solid rgba(193,200,201,.10)",
                }}
            >
                <HeaderCell width="320px">Strategy</HeaderCell>
                <HeaderCell width="120px">Type</HeaderCell>
                <HeaderCell width="140px">Asset</HeaderCell>
                <HeaderCell width="180px">TVL</HeaderCell>
                <HeaderCell align="right">APY · 7d</HeaderCell>
            </div>

            {/* Rows */}
            {filtered.map(s => (
                <StrategyRow key={s.address} strategy={s} />
            ))}

            {/* Loading skeletons */}
            {isLoading && filtered.length === 0 &&
                [0, 1, 2].map(i => <SkeletonRow key={`sk-${i}`} index={i} />)
            }

            {/* Error */}
            {error && (
                <div
                    style={{
                        padding: "24px 28px",
                        textAlign: "center",
                        color: "#FC5B31",
                        fontSize: 13,
                    }}
                >
                    Couldn&apos;t load strategies. Please retry.
                </div>
            )}

            {/* Empty search state */}
            {!isLoading && !error && filtered.length === 0 && search && (
                <div
                    style={{
                        padding: "48px 28px",
                        textAlign: "center",
                        color: "rgba(255,255,255,.5)",
                        fontSize: 14,
                    }}
                >
                    No matches for <em style={{ color: "#FC5B31" }}>{search}</em>
                </div>
            )}

            {/* Empty zero data state */}
            {!isLoading && !error && strategies.length === 0 && (
                <div
                    style={{
                        padding: "48px 28px",
                        textAlign: "center",
                        color: "rgba(255,255,255,.5)",
                        fontSize: 14,
                    }}
                >
                    No strategies available.
                </div>
            )}
        </div>
    );
}
