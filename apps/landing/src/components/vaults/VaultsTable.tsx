"use client";

import { useVaults } from "@/hooks/useVaultInfo";
import VaultRow from "./VaultRow";
import VaultRowSkeleton from "./VaultRowSkeleton";

const VAULT_ADDRESSES = [
    "CBNKCU3HGFKHFOF7JTGXQCNKE3G3DXS5RDBQUKQMIIECYKXPIOUGB2S3", // Beans USDC
    "CC24OISYJHWXZIFZBRJHFLVO5CNN3PQSKZE5BBBZLSSI5Z23TKC6GQY2", // Soroswap CETES
    "CA2FIPJ7U6BG3N7EOZFI74XPJZOEOD4TYWXFVCIO5VDCHTVAGS6F4UKK", // Soroswap USDC
    "CCKTLDG6I2MMJCKFWXXBXMA42LJ3XN2IOW6M7TK6EWNPJTS736ETFF2N", // Soroswap EURC
    "CAIZ3NMNPEN5SQISJV7PD2YY6NI6DIPFA4PCRUBOGDE4I7A3DXDLK5OI", // Beans EURC
    "CBUJZL5QAD5TOPD7JMCBQ3RHR6RZWY34A4QF7UHILTDH2JF2Z3VJGY2Y", // HANA USDC
    "CD4JGS6BB5NZVSNKRNI43GUC6E3OBYLCLBQZJVTZLDVHQ5KDAOHVOIQF", // xPortal USDC
    "CC767WIU5QGJMXYHDDYJAJEF2YWPHOXOZDWD3UUAZVS4KQPRXCKPT2YZ", // Seevcash
    "CCDRFMZ7CH364ATQ5YSVTEJ3G3KPNFVM6TTC6N4T5REHWJS6LGVFP7MY", // Rozo
];

const TABLE_HEADERS = [
    { key: "vault", label: "Vault" },
    { key: "tvl", label: "TVL" },
    { key: "exposure", label: "Exposure" },
    { key: "apy", label: "APY" },
];

export default function VaultsTable() {
    const { vaultStates, sortedVaults, isAnyLoading } = useVaults({
        vaultIds: VAULT_ADDRESSES,
    });

    const pendingCount = vaultStates.filter(
        (v) => v.status === "pending" || v.status === "loading",
    ).length;

    const errorVaults = vaultStates.filter((v) => v.status === "error");

    return (
        <div className="bg-cyan-950/30 backdrop-blur-sm border border-cyan-800/30 rounded-2xl overflow-hidden">
            <div className="overflow-x-auto">
                <table className="w-full">
                    <thead>
                        <tr className="border-b border-cyan-800/50">
                            {TABLE_HEADERS.map((header) => (
                                <th
                                    key={header.key}
                                    className="px-4 py-4 text-left font-manrope font-semibold text-sm text-lime-200/80"
                                >
                                    {header.label}
                                </th>
                            ))}
                        </tr>
                    </thead>
                    <tbody>
                        {/* Render loaded vaults */}
                        {sortedVaults.map((vault) => (
                            <VaultRow key={vault.address} vault={vault} />
                        ))}

                        {/* Render skeletons for pending/loading vaults */}
                        {Array.from({ length: pendingCount }).map((_, index) => (
                            <VaultRowSkeleton key={`skeleton-${index}`} />
                        ))}

                        {/* Render error rows */}
                        {errorVaults.map((v) => (
                            <tr key={`error-${v.address}`} className="border-b border-cyan-800/30">
                                <td
                                    colSpan={4}
                                    className="px-4 py-4 text-center text-red-400/80 text-sm"
                                >
                                    Failed to load vault: {v.error}
                                </td>
                            </tr>
                        ))}
                    </tbody>
                </table>
            </div>

            {/* Loading indicator */}
            {isAnyLoading && (
                <div className="px-4 py-3 text-center text-sm text-white/50 border-t border-cyan-800/30">
                    Loading vaults...
                </div>
            )}
        </div>
    );
}
