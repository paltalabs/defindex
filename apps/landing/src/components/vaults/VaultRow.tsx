"use client";

import Image from "next/image";
import type { VaultWithAddress, ManagedFunds } from "@/types/vault.types";
import {
    calculateTotalTVL,
    formatTokenAmountByAddress,
    formatAPY,
    getPrimaryAssetAddress,
} from "@/utils/vaultFormatters";
import { getTokenSymbol } from "@/lib/tokenIcons";
import { getVaultLogo, formatVaultName } from "@/lib/vaultLogos";
import TokenExposure from "./TokenExposure";

interface VaultRowProps {
    vault: VaultWithAddress;
}

export default function VaultRow({ vault }: VaultRowProps) {
    const managedFunds = vault.totalManagedFunds as ManagedFunds[];
    const totalTVL = calculateTotalTVL(managedFunds);
    const primaryAssetAddress = getPrimaryAssetAddress(managedFunds);
    const primarySymbol = getTokenSymbol(primaryAssetAddress);

    const vaultLogo = getVaultLogo(vault.address);
    const displayName = formatVaultName(vault.name);
    const avatarLetters = vault.symbol.slice(0, 2).toUpperCase();
    const isXPortal = vault.address === "CD4JGS6BB5NZVSNKRNI43GUC6E3OBYLCLBQZJVTZLDVHQ5KDAOHVOIQF";

    return (
        <tr className="border-b border-cyan-800/30 hover:bg-cyan-900/20 transition-colors">
            {/* Vault Name */}
            <td className="px-4 py-4">
                <div className="flex items-center gap-3">
                    <div className="w-10 h-10 rounded-full bg-cyan-900/50 border border-cyan-800/50 flex items-center justify-center flex-shrink-0 overflow-hidden">
                        {vaultLogo ? (
                            <Image
                                src={vaultLogo}
                                alt={vault.name}
                                width={40}
                                height={40}
                                className={`w-full h-full object-cover ${isXPortal ? "bg-white" : ""}`}
                            />
                        ) : (
                            <span className="text-sm font-bold text-lime-200">{avatarLetters}</span>
                        )}
                    </div>
                    <div className="min-w-0 flex flex-col">
                        <p className="font-manrope font-semibold text-white truncate leading-tight">
                            {displayName}
                        </p>
                        <p className="text-xs text-white/50 leading-tight">{vault.symbol}</p>
                    </div>
                </div>
            </td>

            {/* TVL */}
            <td className="px-4 py-4">
                <div className="flex flex-col">
                    <p className="font-manrope font-semibold text-white whitespace-nowrap leading-tight">
                        {formatTokenAmountByAddress(totalTVL, primaryAssetAddress)}
                    </p>
                    <p className="text-xs text-white/50 leading-tight">{primarySymbol}</p>
                </div>
            </td>

            {/* Exposure */}
            <td className="px-4 py-4">
                <TokenExposure assets={vault.assets} />
            </td>

            {/* APY */}
            <td className="px-4 py-4">
                <span className="font-manrope font-bold text-lime-200">{formatAPY(vault.apy)}</span>
            </td>
        </tr>
    );
}
