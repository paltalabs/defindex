'use client';

import { useMemo } from 'react';
import Image from 'next/image';
import Link from 'next/link';
import { useVaults } from '@/hooks/useVaultInfo';
import { useTokenPrices } from '@/hooks/useTokenPrices';
import { stroopsToNum, fmtUsd } from '@/utils/vaultFormatters';
import type { ManagedFunds } from '@/types/vault.types';

const MERU_VAULT_ADDRESS = 'CCA2ZJP5BVRXYTQH4FAGHCAUMRYCXVC4CRYC2NXHWMR7TIVX36U7F5HR';

const CARD_STYLE = {
  background: 'rgba(29,57,62,.40)',
  border: '1px solid rgba(193,200,201,.10)',
  borderRadius: 24,
  overflow: 'hidden',
  backdropFilter: 'blur(14px)',
  WebkitBackdropFilter: 'blur(14px)',
} as const;

export default function MeruTVLWidget() {
  const { sortedVaults, isAnyLoading } = useVaults({ vaultIds: [MERU_VAULT_ADDRESS] });
  const { prices } = useTokenPrices();

  const tvlUsd = useMemo(() => {
    if (!sortedVaults.length) return null;
    const vault = sortedVaults[0];
    const funds = vault.totalManagedFunds as ManagedFunds[];
    if (!funds?.length) return null;
    const amount = stroopsToNum(funds[0].total_amount);
    const price = prices[funds[0].asset] ?? 1;
    return amount * price;
  }, [sortedVaults, prices]);

  const apy = useMemo(() => {
    if (!sortedVaults.length) return null;
    return sortedVaults[0].apy ?? null;
  }, [sortedVaults]);

  return (
    <section className="py-6 md:py-10">
      <div style={CARD_STYLE}>
        <div
          style={{
            padding: 'clamp(32px, 5vw, 64px)',
            display: 'flex',
            flexDirection: 'column',
            alignItems: 'center',
            gap: 20,
            textAlign: 'center',
          }}
        >
          {/* Meru logo + name */}
          <div style={{ display: 'flex', alignItems: 'center', gap: 12 }}>
            <Image
              src="/images/meru-logo.svg"
              alt="Meru"
              width={36}
              height={36}
              style={{
                filter: 'brightness(0) invert(1)',
                objectFit: 'contain',
              }}
            />
            <span
              style={{
                fontFamily: 'Familjen Grotesk, sans-serif',
                fontSize: 22,
                fontWeight: 600,
                color: 'rgba(255,255,255,.75)',
                letterSpacing: '-0.01em',
              }}
            >
              Meru
            </span>
          </div>

          {/* Label */}
          <div
            style={{
              fontSize: 12,
              fontWeight: 600,
              color: 'rgba(255,255,255,.35)',
              letterSpacing: '0.14em',
              textTransform: 'uppercase',
            }}
          >
            Total Value Locked
          </div>

          {/* TVL number */}
          {isAnyLoading || tvlUsd === null ? (
            <div
              className="animate-pulse"
              style={{
                height: 'clamp(56px, 9vw, 96px)',
                width: 'clamp(160px, 25vw, 280px)',
                borderRadius: 16,
                background: 'rgba(255,255,255,.06)',
              }}
            />
          ) : (
            <div
              style={{
                fontFamily: 'Familjen Grotesk, sans-serif',
                fontSize: 'clamp(56px, 10vw, 96px)',
                fontWeight: 700,
                color: '#D9F99D',
                letterSpacing: '-0.04em',
                lineHeight: 1,
                fontFeatureSettings: '"tnum"',
              }}
            >
              {fmtUsd(tvlUsd)}
            </div>
          )}

          {/* APY badge */}
          {!isAnyLoading && apy !== null && apy > 0 && (
            <div
              style={{
                display: 'inline-flex',
                alignItems: 'center',
                gap: 6,
                padding: '6px 14px',
                borderRadius: 100,
                background: 'rgba(217,249,157,.10)',
                border: '1px solid rgba(217,249,157,.25)',
              }}
            >
              <span
                style={{
                  fontSize: 13,
                  fontWeight: 600,
                  color: '#D9F99D',
                  fontFeatureSettings: '"tnum"',
                }}
              >
                {apy.toFixed(2)}% APY
              </span>
            </div>
          )}

          {/* Powered by DeFindex */}
          <div
            style={{
              fontSize: 13,
              color: 'rgba(255,255,255,.3)',
              fontFamily: 'Inter Tight, sans-serif',
              marginTop: 4,
            }}
          >
            Powered by{' '}
            <Link
              href="/partners"
              style={{ color: '#FC5B31', fontWeight: 600, textDecoration: 'none' }}
            >
              DeFindex
            </Link>
          </div>
        </div>
      </div>
    </section>
  );
}
