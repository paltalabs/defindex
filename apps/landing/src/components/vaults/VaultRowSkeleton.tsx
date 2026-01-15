'use client';

export default function VaultRowSkeleton() {
  return (
    <tr className="animate-pulse border-b border-cyan-800/30">
      {/* Vault Name */}
      <td className="px-4 py-4">
        <div className="flex items-center gap-3">
          <div className="w-10 h-10 bg-cyan-800/40 rounded-full" />
          <div className="space-y-2">
            <div className="h-4 w-32 bg-cyan-800/40 rounded" />
            <div className="h-3 w-16 bg-cyan-800/30 rounded" />
          </div>
        </div>
      </td>

      {/* TVL */}
      <td className="px-4 py-4">
        <div className="h-4 w-24 bg-cyan-800/40 rounded" />
      </td>

      {/* Exposure */}
      <td className="px-4 py-4">
        <div className="flex -space-x-2">
          {[1, 2, 3].map((i) => (
            <div key={i} className="w-6 h-6 bg-cyan-800/40 rounded-full border-2 border-cyan-950" />
          ))}
        </div>
      </td>

      {/* APY */}
      <td className="px-4 py-4">
        <div className="h-4 w-16 bg-cyan-800/40 rounded" />
      </td>
    </tr>
  );
}
