'use client';

import Image from 'next/image';
import { getTokenIcon, getTokenInfo } from '@/lib/tokenIcons';

interface Asset {
  address: string;
  symbol: string;
  name?: string;
}

interface TokenExposureProps {
  assets: Asset[];
  maxDisplay?: number;
}

export default function TokenExposure({ assets, maxDisplay = 4 }: TokenExposureProps) {
  const displayedAssets = assets.slice(0, maxDisplay);
  const remaining = assets.length - maxDisplay;

  return (
    <div className="flex items-center -space-x-2">
      {displayedAssets.map((asset) => {
        const iconUrl = getTokenIcon(asset.address);
        const info = getTokenInfo(asset.address);

        return (
          <div
            key={asset.address}
            className="w-7 h-7 rounded-full border-2 border-cyan-950 bg-cyan-900/50 flex items-center justify-center overflow-hidden"
            title={info?.name ?? asset.name ?? asset.symbol}
          >
            {iconUrl ? (
              <Image
                src={iconUrl}
                alt={asset.symbol}
                width={24}
                height={24}
                className="rounded-full object-cover"
                unoptimized
              />
            ) : (
              <span className="text-[9px] font-semibold text-white/70">
                {asset.symbol.slice(0, 2)}
              </span>
            )}
          </div>
        );
      })}
      {remaining > 0 && (
        <div className="w-7 h-7 rounded-full border-2 border-cyan-950 bg-cyan-800 flex items-center justify-center">
          <span className="text-[10px] font-semibold text-white">+{remaining}</span>
        </div>
      )}
    </div>
  );
}
