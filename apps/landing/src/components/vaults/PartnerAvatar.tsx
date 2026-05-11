'use client';

import Image from 'next/image';

interface PartnerAvatarProps {
  name: string;
  color: string;
  glyph: string;
  logo?: string | null;
  /** Override background when the logo needs a specific color to be visible (e.g. white for dark SVGs) */
  logoBg?: string;
  size?: number;
}

const DARK_BG_COLORS = ['#000000', '#0F0F0F', '#3A1B5C'];

export default function PartnerAvatar({ name, color, glyph, logo, logoBg, size = 36 }: PartnerAvatarProps) {
  const isDark = DARK_BG_COLORS.includes(color);

  // When a logo is present: transparent bg by default, or use logoBg if provided
  const background = logo
    ? (logoBg ?? 'transparent')
    : color;

  return (
    <div
      aria-label={name}
      style={{
        width: size,
        height: size,
        borderRadius: '50%',
        background,
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        fontFamily: 'Familjen Grotesk, sans-serif',
        fontWeight: 700,
        fontSize: size * 0.42,
        color: isDark ? '#fff' : '#0a1f24',
        flexShrink: 0,
        boxShadow: 'inset 0 0 0 1px rgba(255,255,255,.08)',
        overflow: 'hidden',
      }}
    >
      {logo ? (
        <Image
          src={logo}
          alt={name}
          width={size}
          height={size}
          style={{ width: '100%', height: '100%', objectFit: 'cover' }}
        />
      ) : (
        glyph
      )}
    </div>
  );
}
