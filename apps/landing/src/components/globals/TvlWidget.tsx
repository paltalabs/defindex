"use client";

import { useMemo } from "react";
import Link from "next/link";
import { useVaults } from "@/hooks/useVaultInfo";
import { useTokenPrices } from "@/hooks/useTokenPrices";
import { stroopsToNum, fmtUsd } from "@/utils/vaultFormatters";
import { formatVaultName, getPartnerInfo } from "@/lib/vaultLogos";
import { VAULT_ADDRESSES } from "@/components/vaults/VaultsTable";
import type { ManagedFunds } from "@/types/vault.types";

const GLASS_CARD = {
    background: "rgba(29,57,62,.40)",
    border: "1px solid rgba(193,200,201,.10)",
    borderRadius: 20,
    backdropFilter: "blur(14px)",
    WebkitBackdropFilter: "blur(14px)",
} as const;

function StatItem({
    label,
    value,
    highlight,
    loading,
}: {
    label: string;
    value: string;
    highlight?: boolean;
    loading?: boolean;
}) {
    return (
        <div className="text-center">
            <div
                style={{
                    fontSize: 11,
                    fontWeight: 500,
                    color: "rgba(255,255,255,.4)",
                    letterSpacing: "0.08em",
                    textTransform: "uppercase",
                    marginBottom: 6,
                }}
            >
                {label}
            </div>
            {loading ? (
                <div
                    className="h-7 w-16 sm:h-9 sm:w-28"
                    style={{
                        background: "rgba(255,255,255,.08)",
                        borderRadius: 8,
                        margin: "0 auto",
                        animation: "pulse 1.4s linear infinite",
                    }}
                />
            ) : (
                <div
                    className="text-xl sm:text-3xl lg:text-4xl"
                    style={{
                        fontFamily: "Familjen Grotesk, sans-serif",
                        fontWeight: 700,
                        color: highlight ? "#D9F99D" : "#fff",
                        letterSpacing: "-0.02em",
                        fontFeatureSettings: '"tnum"',
                        lineHeight: 1,
                    }}
                >
                    {value}
                </div>
            )}
        </div>
    );
}

function Divider() {
    return (
        <div
            className="hidden sm:block flex-shrink-0"
            style={{
                width: 1,
                height: 48,
                background: "rgba(255,255,255,.10)",
                alignSelf: "center",
            }}
        />
    );
}

export default function TvlWidget() {
    const { sortedVaults, isAnyLoading } = useVaults({ vaultIds: VAULT_ADDRESSES });
    const { prices } = useTokenPrices();

    const { totalTvl, partnerCount } = useMemo(() => {
        const tvl = sortedVaults.reduce((sum, v) => {
            const funds = v.totalManagedFunds as ManagedFunds[];
            if (!funds?.length) return sum;
            const amount = stroopsToNum(funds[0].total_amount);
            const price = prices[funds[0].asset] ?? 1;
            return sum + amount * price;
        }, 0);

        const names = new Set(
            sortedVaults.map((v) => getPartnerInfo(formatVaultName(v.name)).name)
        );

        return { totalTvl: tvl, partnerCount: names.size };
    }, [sortedVaults, prices]);

    const loading = isAnyLoading && sortedVaults.length === 0;

    return (
        <section className="py-6 md:py-10 px-4">
            <div className="max-w-[1180px] mx-auto">
                <div
                    style={GLASS_CARD}
                    className="px-5 py-6 sm:px-10 sm:py-8 lg:px-12"
                >
                    <div className="flex flex-col sm:flex-row items-center justify-between gap-6 sm:gap-4 lg:gap-0">

                        {/* Left label */}
                        <div className="text-center sm:text-left sm:flex-none" style={{ minWidth: 130 }}>
                            <div
                                style={{
                                    fontSize: 11,
                                    fontWeight: 600,
                                    color: "#FC5B31",
                                    letterSpacing: "0.12em",
                                    textTransform: "uppercase",
                                    marginBottom: 4,
                                }}
                            >
                                Live
                            </div>
                            <div
                                style={{
                                    fontFamily: "Familjen Grotesk, sans-serif",
                                    fontSize: 18,
                                    fontWeight: 700,
                                    color: "#fff",
                                    letterSpacing: "-0.01em",
                                }}
                            >
                                Protocol Metrics
                            </div>
                        </div>

                        {/* Stats — grid on mobile so each item gets equal width, flex on sm+ */}
                        <div className="grid grid-cols-3 items-center gap-3 w-full sm:flex sm:flex-row sm:w-auto sm:flex-1 sm:justify-center sm:gap-10 lg:gap-16">
                            <StatItem
                                label="Total TVL"
                                value={fmtUsd(totalTvl)}
                                highlight
                                loading={loading}
                            />
                            <Divider />
                            <StatItem
                                label="Active Partners"
                                value={loading ? "—" : String(partnerCount)}
                                loading={loading}
                            />
                            <Divider />
                            <StatItem
                                label="Vaults"
                                value={String(VAULT_ADDRESSES.length)}
                            />
                        </div>

                        {/* CTA */}
                        <div className="hidden lg:block lg:flex-none text-center lg:text-right" style={{ minWidth: 130 }}>
                            <Link
                                href="/partners"
                                style={{
                                    display: "inline-flex",
                                    alignItems: "center",
                                    gap: 6,
                                    fontSize: 13,
                                    fontWeight: 600,
                                    color: "rgba(255,255,255,.7)",
                                    textDecoration: "none",
                                    letterSpacing: "0.01em",
                                    transition: "color 0.2s",
                                    fontFamily: "Inter Tight, sans-serif",
                                }}
                                onMouseEnter={(e) => {
                                    (e.currentTarget as HTMLAnchorElement).style.color = "#fff";
                                }}
                                onMouseLeave={(e) => {
                                    (e.currentTarget as HTMLAnchorElement).style.color =
                                        "rgba(255,255,255,.7)";
                                }}
                            >
                                View all partners
                                <svg
                                    width="14"
                                    height="14"
                                    viewBox="0 0 14 14"
                                    fill="none"
                                    xmlns="http://www.w3.org/2000/svg"
                                    style={{ flexShrink: 0 }}
                                >
                                    <path
                                        d="M2.917 7h8.166M7.583 4l3.5 3-3.5 3"
                                        stroke="currentColor"
                                        strokeWidth="1.4"
                                        strokeLinecap="round"
                                        strokeLinejoin="round"
                                    />
                                </svg>
                            </Link>
                        </div>

                    </div>
                </div>
            </div>
        </section>
    );
}
