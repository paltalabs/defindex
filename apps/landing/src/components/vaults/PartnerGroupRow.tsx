'use client';

import { useState } from 'react';
import Image from 'next/image';
import type { VaultWithAddress, ManagedFunds } from '@/types/vault.types';
import {
  stroopsToNum,
  fmtTVL,
  fmtUsd,
} from '@/utils/vaultFormatters';
import { getTokenSymbol } from '@/lib/tokenIcons';
import { getPartnerInfo, getVaultLogo, getVaultLogoBg, formatVaultName } from '@/lib/vaultLogos';
import TokenExposure from './TokenExposure';
import PartnerAvatar from './PartnerAvatar';

export interface PartnerGroup {
  partnerName: string;
  vaults: VaultWithAddress[];
  tvlUsdSum: number;
  weightedApy: number;
  exposureSymbols: string[];
}

interface PartnerGroupRowProps {
  group: PartnerGroup;
  prices: Record<string, number>;
}

function Caret({ open }: { open: boolean }) {
  return (
    <svg
      width="12"
      height="12"
      viewBox="0 0 12 12"
      style={{
        transition: 'transform 200ms cubic-bezier(0.4,0,0.2,1)',
        transform: open ? 'rotate(90deg)' : 'rotate(0deg)',
        flexShrink: 0,
      }}
      aria-hidden="true"
    >
      <path
        d="M4 2 L8 6 L4 10"
        fill="none"
        stroke="rgba(255,255,255,.55)"
        strokeWidth="1.6"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
    </svg>
  );
}

function getPrimaryTvlNum(vault: VaultWithAddress): number {
  const funds = vault.totalManagedFunds as ManagedFunds[];
  if (!funds?.length) return 0;
  return stroopsToNum(funds[0].total_amount);
}

function getPrimaryTvlUsd(vault: VaultWithAddress, prices: Record<string, number>): number {
  const funds = vault.totalManagedFunds as ManagedFunds[];
  if (!funds?.length) return 0;
  const amount = stroopsToNum(funds[0].total_amount);
  const price = prices[funds[0].asset] ?? 1;
  return amount * price;
}

function getPrimaryAssetSymbol(vault: VaultWithAddress): string {
  const funds = vault.totalManagedFunds as ManagedFunds[];
  if (!funds?.length) return '';
  return getTokenSymbol(funds[0].asset);
}

function getPrimaryStrategy(vault: VaultWithAddress): string {
  const firstAsset = vault.assets?.[0];
  if (!firstAsset?.strategies?.length) return '';
  const live = firstAsset.strategies.find(s => !s.paused);
  return (live ?? firstAsset.strategies[0])?.name ?? '';
}

export function buildPartnerGroups(
  vaults: VaultWithAddress[],
  prices: Record<string, number> = {}
): PartnerGroup[] {
  const map = new Map<string, VaultWithAddress[]>();
  for (const vault of vaults) {
    const clean = formatVaultName(vault.name);
    const info = getPartnerInfo(clean);
    if (!map.has(info.name)) map.set(info.name, []);
    map.get(info.name)!.push(vault);
  }

  return Array.from(map.entries()).map(([partnerName, groupVaults]) => {
    const tvlUsdSum = groupVaults.reduce((s, v) => s + getPrimaryTvlUsd(v, prices), 0);

    const liveVaults = groupVaults.filter(v => (v.apy ?? 0) > 0);
    const wTotal = liveVaults.reduce((s, v) => s + getPrimaryTvlUsd(v, prices), 0);
    const weightedApy =
      wTotal > 0
        ? liveVaults.reduce((s, v) => s + (v.apy ?? 0) * getPrimaryTvlUsd(v, prices), 0) / wTotal
        : 0;

    const exposureSymbols = Array.from(
      new Set(groupVaults.flatMap(v => v.assets?.map(a => a.symbol) ?? []))
    );

    return { partnerName, vaults: groupVaults, tvlUsdSum, weightedApy, exposureSymbols };
  });
}

function VaultSubRow({
  vault,
  prices,
  isLast,
}: {
  vault: VaultWithAddress;
  prices: Record<string, number>;
  isLast: boolean;
}) {
  const tvlNum = getPrimaryTvlNum(vault);
  const tvlUsd = getPrimaryTvlUsd(vault, prices);
  const tvlSymbol = getPrimaryAssetSymbol(vault);
  const strategyName = getPrimaryStrategy(vault);
  const isIdle = (vault.apy ?? 0) === 0;
  const logo = getVaultLogo(vault.address);
  const logoBg = getVaultLogoBg(vault.address);
  const chipBg = logo ? (logoBg ?? 'transparent') : 'rgba(255,255,255,.04)';

  return (
    <div
      className="sub-row"
      style={{
        display: 'flex',
        alignItems: 'center',
        gap: 16,
        padding: '18px 28px',
        background: 'rgba(0,0,0,.18)',
        borderTop: '1px solid rgba(193,200,201,.05)',
        position: 'relative',
        animation: 'subSlide 220ms cubic-bezier(0.4,0,0.2,1) both',
      }}
    >
      {/* Indent guide */}
      <div style={{ flex: '0 0 16px', position: 'relative', alignSelf: 'stretch' }}>
        <div
          style={{
            position: 'absolute',
            left: 7,
            top: -4,
            bottom: isLast ? '50%' : -4,
            width: 1,
            background: 'rgba(193,200,201,.15)',
          }}
        />
        <div
          style={{
            position: 'absolute',
            left: 7,
            top: '50%',
            width: 8,
            height: 1,
            background: 'rgba(193,200,201,.15)',
          }}
        />
      </div>

      {/* Vault name */}
      <div
        style={{
          flex: '0 0 280px',
          display: 'flex',
          alignItems: 'center',
          gap: 14,
          paddingLeft: 8,
          minWidth: 0,
        }}
      >
        <div
          style={{
            width: 26,
            height: 26,
            borderRadius: 8,
            background: chipBg,
            border: '1px solid rgba(193,200,201,.08)',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            flexShrink: 0,
            overflow: 'hidden',
          }}
        >
          {logo ? (
            <Image
              src={logo}
              alt={formatVaultName(vault.name)}
              width={26}
              height={26}
              style={{ width: '100%', height: '100%', objectFit: 'cover' }}
            />
          ) : (
            <span style={{ fontSize: 10, fontWeight: 700, color: 'rgba(255,255,255,.6)' }}>
              {tvlSymbol.slice(0, 2)}
            </span>
          )}
        </div>
        <div style={{ minWidth: 0 }}>
          <div
            style={{
              fontSize: 13.5,
              fontWeight: 500,
              color: 'rgba(255,255,255,.85)',
              overflow: 'hidden',
              textOverflow: 'ellipsis',
              whiteSpace: 'nowrap',
            }}
          >
            {formatVaultName(vault.name)}
          </div>
          <div
            style={{
              fontSize: 11,
              color: 'rgba(255,255,255,.4)',
              marginTop: 1,
              fontFamily: 'ui-monospace, SFMono-Regular, Menlo, monospace',
            }}
          >
            {vault.symbol}
          </div>
        </div>
      </div>

      {/* TVL */}
      <div style={{ flex: '0 0 180px' }}>
        <div
          style={{
            fontSize: 13.5,
            fontWeight: 500,
            color: 'rgba(255,255,255,.85)',
            fontFeatureSettings: '"tnum"',
          }}
        >
          {fmtTVL(tvlNum, tvlSymbol)}
        </div>
        <div style={{ fontSize: 11, color: 'rgba(255,255,255,.4)', marginTop: 1 }}>
          {fmtUsd(tvlUsd)}
        </div>
      </div>

      {/* Exposure */}
      <div style={{ flex: '0 0 280px' }}>
        <TokenExposure assets={vault.assets ?? []} />
      </div>

      {/* APY */}
      <div style={{ flex: 1, textAlign: 'right', overflow: 'hidden' }}>
        <div
          style={{
            fontFamily: 'Familjen Grotesk, sans-serif',
            fontSize: 16,
            fontWeight: 600,
            color: isIdle ? 'rgba(255,255,255,.3)' : '#D9F99D',
            fontFeatureSettings: '"tnum"',
            letterSpacing: '-0.01em',
          }}
        >
          {(vault.apy ?? 0).toFixed(2)}%
        </div>
        <div style={{ fontSize: 11, color: 'rgba(255,255,255,.4)', marginTop: 1, whiteSpace: 'nowrap', overflow: 'hidden', textOverflow: 'ellipsis' }}>
          {strategyName}
        </div>
      </div>
    </div>
  );
}

export default function PartnerGroupRow({ group, prices }: PartnerGroupRowProps) {
  const [isExpanded, setIsExpanded] = useState(false);
  const [isHovered, setIsHovered] = useState(false);
  const [isFocused, setIsFocused] = useState(false);

  const isSingle = group.vaults.length === 1;
  const singleVault = isSingle ? group.vaults[0] : null;
  const partnerInfo = getPartnerInfo(group.partnerName);
  const firstVaultAddress = group.vaults[0]?.address ?? '';
  const partnerLogo = getVaultLogo(firstVaultAddress);
  const partnerLogoBg = getVaultLogoBg(firstVaultAddress);

  const singleTvlNum = singleVault ? getPrimaryTvlNum(singleVault) : 0;
  const singleTvlUsd = singleVault ? getPrimaryTvlUsd(singleVault, prices) : 0;
  const singleTvlSymbol = singleVault ? getPrimaryAssetSymbol(singleVault) : '';
  const singleStrategy = singleVault ? getPrimaryStrategy(singleVault) : '';
  const singleIsIdle = singleVault ? (singleVault.apy ?? 0) === 0 : false;

  const subRowsId = `partner-sub-${group.partnerName.replace(/\s+/g, '-')}`;

  return (
    <div>
      <div
        role={isSingle ? undefined : 'button'}
        tabIndex={isSingle ? undefined : 0}
        aria-expanded={isSingle ? undefined : isExpanded}
        aria-controls={isSingle ? undefined : subRowsId}
        onClick={() => !isSingle && setIsExpanded(p => !p)}
        onKeyDown={e => {
          if (!isSingle && (e.key === 'Enter' || e.key === ' ')) {
            e.preventDefault();
            setIsExpanded(p => !p);
          }
        }}
        onMouseEnter={() => setIsHovered(true)}
        onMouseLeave={() => setIsHovered(false)}
        style={{
          display: 'flex',
          alignItems: 'center',
          gap: 16,
          padding: '22px 28px',
          background: isHovered ? 'rgba(217,249,157,.03)' : 'transparent',
          transition: 'background 200ms cubic-bezier(0.4,0,0.2,1)',
          cursor: isSingle ? 'default' : 'pointer',
          borderTop: '1px solid rgba(193,200,201,.07)',
          outline: isFocused && !isSingle ? '4px solid rgba(217,249,157,.40)' : 'none',
        }}
        onFocus={() => setIsFocused(true)}
        onBlur={() => setIsFocused(false)}
      >
        {/* Caret */}
        <div style={{ flex: '0 0 16px', display: 'flex', justifyContent: 'center' }}>
          {!isSingle && <Caret open={isExpanded} />}
        </div>

        {/* Partner */}
        <div
          style={{
            flex: '0 0 280px',
            display: 'flex',
            alignItems: 'center',
            gap: 14,
            minWidth: 0,
          }}
        >
          <PartnerAvatar
            name={partnerInfo.name}
            color={partnerInfo.color}
            glyph={partnerInfo.glyph}
            logo={partnerLogo}
            logoBg={partnerLogoBg}
          />
          <div style={{ minWidth: 0 }}>
            <div
              style={{
                fontSize: 15,
                fontWeight: 600,
                color: '#fff',
                letterSpacing: '-0.01em',
                overflow: 'hidden',
                textOverflow: 'ellipsis',
                whiteSpace: 'nowrap',
              }}
            >
              {group.partnerName}
            </div>
            <div
              style={{
                fontSize: 12,
                color: 'rgba(255,255,255,.45)',
                marginTop: 2,
                fontFamily: isSingle
                  ? 'ui-monospace, SFMono-Regular, Menlo, monospace'
                  : 'Inter Tight, sans-serif',
              }}
            >
              {isSingle ? singleVault!.symbol : `${group.vaults.length} vaults`}
            </div>
          </div>
        </div>

        {/* TVL */}
        <div style={{ flex: '0 0 180px' }}>
          <div
            style={{
              display: 'flex',
              alignItems: 'baseline',
              gap: 6,
              fontFeatureSettings: '"tnum"',
            }}
          >
            <span style={{ fontSize: 15, fontWeight: 600, color: '#fff' }}>
              {isSingle ? fmtTVL(singleTvlNum, singleTvlSymbol) : fmtUsd(group.tvlUsdSum)}
            </span>
            {!isSingle && (
              <span
                style={{
                  fontSize: 10,
                  fontWeight: 600,
                  color: 'rgba(255,255,255,.35)',
                  letterSpacing: '0.06em',
                  textTransform: 'uppercase',
                }}
              >
                USD
              </span>
            )}
          </div>
          <div style={{ fontSize: 12, color: 'rgba(255,255,255,.45)', marginTop: 2 }}>
            {isSingle ? fmtUsd(singleTvlUsd) : 'across all vaults'}
          </div>
        </div>

        {/* Exposure */}
        <div style={{ flex: '0 0 280px' }}>
          <TokenExposure
            assets={
              isSingle
                ? singleVault!.assets ?? []
                : group.vaults.flatMap(v => v.assets ?? []).filter(
                    (a, i, arr) => arr.findIndex(x => x.symbol === a.symbol) === i
                  )
            }
          />
        </div>

        {/* APY */}
        <div style={{ flex: 1, textAlign: 'right', overflow: 'hidden' }}>
          <div
            style={{
              fontFamily: 'Familjen Grotesk, sans-serif',
              fontSize: 18,
              fontWeight: 700,
              color: singleIsIdle && isSingle ? 'rgba(255,255,255,.3)' : '#D9F99D',
              fontFeatureSettings: '"tnum"',
              letterSpacing: '-0.01em',
            }}
          >
            {isSingle
              ? `${(singleVault!.apy ?? 0).toFixed(2)}%`
              : `${group.weightedApy.toFixed(2)}%`}
          </div>
          <div style={{ fontSize: 11, color: 'rgba(255,255,255,.4)', marginTop: 2, whiteSpace: 'nowrap', overflow: 'hidden', textOverflow: 'ellipsis' }}>
            {isSingle ? (singleStrategy ? `via ${singleStrategy}` : '') : 'TVL-weighted avg'}
          </div>
        </div>
      </div>

      {/* Sub-rows */}
      {isExpanded && !isSingle && (
        <div id={subRowsId}>
          {group.vaults.map((vault, i) => (
            <VaultSubRow
              key={vault.address}
              vault={vault}
              prices={prices}
              isLast={i === group.vaults.length - 1}
            />
          ))}
        </div>
      )}
    </div>
  );
}
