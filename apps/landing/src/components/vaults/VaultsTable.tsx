"use client";

import { useMemo } from "react";
import { useVaults } from "@/hooks/useVaultInfo";
import { formatVaultName, getPartnerInfo } from "@/lib/vaultLogos";
import { stroopsToNum } from "@/utils/vaultFormatters";
import type { ManagedFunds } from "@/types/vault.types";
import PartnerGroupRow, { buildPartnerGroups } from "./PartnerGroupRow";
import { HeaderCell, GLASS_CARD } from "./TableCard";
import type { SortKey } from "@/types/vault.types";

const VAULT_ADDRESSES = [
    "CBNKCU3HGFKHFOF7JTGXQCNKE3G3DXS5RDBQUKQMIIECYKXPIOUGB2S3",
    "CC24OISYJHWXZIFZBRJHFLVO5CNN3PQSKZE5BBBZLSSI5Z23TKC6GQY2",
    "CA2FIPJ7U6BG3N7EOZFI74XPJZOEOD4TYWXFVCIO5VDCHTVAGS6F4UKK",
    "CCKTLDG6I2MMJCKFWXXBXMA42LJ3XN2IOW6M7TK6EWNPJTS736ETFF2N",
    "CAIZ3NMNPEN5SQISJV7PD2YY6NI6DIPFA4PCRUBOGDE4I7A3DXDLK5OI",
    "CBUJZL5QAD5TOPD7JMCBQ3RHR6RZWY34A4QF7UHILTDH2JF2Z3VJGY2Y",
    "CD4JGS6BB5NZVSNKRNI43GUC6E3OBYLCLBQZJVTZLDVHQ5KDAOHVOIQF",
    "CC767WIU5QGJMXYHDDYJAJEF2YWPHOXOZDWD3UUAZVS4KQPRXCKPT2YZ",
    "CCDRFMZ7CH364ATQ5YSVTEJ3G3KPNFVM6TTC6N4T5REHWJS6LGVFP7MY",
    "CCA2ZJP5BVRXYTQH4FAGHCAUMRYCXVC4CRYC2NXHWMR7TIVX36U7F5HR",
];

interface VaultsTableProps {
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
            <div style={{ flex: "0 0 16px" }} />
            <div style={{ flex: "0 0 280px", display: "flex", alignItems: "center", gap: 14 }}>
                <div style={{ width: 36, height: 36, borderRadius: "50%", background: "rgba(255,255,255,.08)" }} />
                <div style={{ flex: 1 }}>
                    <div style={{ height: 14, width: 120, background: "rgba(255,255,255,.08)", borderRadius: 4, marginBottom: 6 }} />
                    <div style={{ height: 11, width: 60, background: "rgba(255,255,255,.06)", borderRadius: 4 }} />
                </div>
            </div>
            <div style={{ flex: "0 0 180px" }}>
                <div style={{ height: 14, width: 80, background: "rgba(255,255,255,.08)", borderRadius: 4, marginBottom: 6 }} />
                <div style={{ height: 11, width: 50, background: "rgba(255,255,255,.06)", borderRadius: 4 }} />
            </div>
            <div style={{ flex: "0 0 280px", display: "flex", gap: 6 }}>
                {[1, 2].map(i => (
                    <div key={i} style={{ width: 18, height: 18, borderRadius: "50%", background: "rgba(255,255,255,.08)" }} />
                ))}
            </div>
            <div style={{ flex: 1, display: "flex", justifyContent: "flex-end" }}>
                <div style={{ height: 18, width: 60, background: "rgba(255,255,255,.08)", borderRadius: 4 }} />
            </div>
        </div>
    );
}


export default function VaultsTable({ search = "", sort = "TVL" }: VaultsTableProps) {
    const { vaultStates, sortedVaults, isAnyLoading } = useVaults({ vaultIds: VAULT_ADDRESSES });

    const pendingCount = vaultStates.filter(
        v => v.status === "pending" || v.status === "loading"
    ).length;

    const groups = useMemo(() => {
        const allGroups = buildPartnerGroups(sortedVaults);

        const filtered = search
            ? allGroups.filter(g => {
                const q = search.toLowerCase();
                if (g.partnerName.toLowerCase().includes(q)) return true;
                return g.vaults.some(v =>
                    [formatVaultName(v.name), v.symbol, ...(v.assets?.flatMap(a =>
                        a.strategies?.map(s => s.name) ?? []
                    ) ?? [])]
                        .some(f => f.toLowerCase().includes(q))
                );
            })
            : allGroups;

        return [...filtered].sort((a, b) => {
            if (sort === "TVL") return b.tvlSum - a.tvlSum;
            if (sort === "APY") return b.weightedApy - a.weightedApy;
            if (sort === "Name") return a.partnerName.localeCompare(b.partnerName);
            return 0;
        });
    }, [sortedVaults, search, sort]);

    const errorVaults = vaultStates.filter(v => v.status === "error");

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
                <div style={{ flex: "0 0 16px" }} />
                <HeaderCell width="280px">Partner</HeaderCell>
                <HeaderCell width="180px">TVL</HeaderCell>
                <HeaderCell width="280px">Token exposure</HeaderCell>
                <HeaderCell align="right">APY</HeaderCell>
            </div>

            {/* Partner groups */}
            {groups.map(group => (
                <PartnerGroupRow key={group.partnerName} group={group} />
            ))}

            {/* Loading skeletons */}
            {Array.from({ length: Math.min(pendingCount, 4) }).map((_, i) => (
                <SkeletonRow key={`sk-${i}`} index={i} />
            ))}

            {/* Error rows */}
            {errorVaults.map(v => (
                <div
                    key={`err-${v.address}`}
                    style={{
                        padding: "16px 28px",
                        borderTop: "1px solid rgba(193,200,201,.07)",
                        color: "#FC5B31",
                        fontSize: 13,
                        textAlign: "center",
                    }}
                >
                    Failed to load vault: {v.error}
                </div>
            ))}

            {/* Empty search state */}
            {!isAnyLoading && groups.length === 0 && search && (
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
            {!isAnyLoading && groups.length === 0 && !search && sortedVaults.length === 0 && (
                <div
                    style={{
                        padding: "48px 28px",
                        textAlign: "center",
                        color: "rgba(255,255,255,.5)",
                        fontSize: 14,
                    }}
                >
                    No partners live yet — be the first to integrate.
                </div>
            )}
        </div>
    );
}

export { VAULT_ADDRESSES };
