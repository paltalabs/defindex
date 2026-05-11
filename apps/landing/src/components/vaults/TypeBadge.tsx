'use client';

interface TypeBadgeProps {
  type: string;
}

const TYPE_STYLES: Record<string, { bg: string; bd: string; fg: string }> = {
  blend:     { bg: 'rgba(217,249,157,.10)', bd: 'rgba(217,249,157,.25)', fg: '#D9F99D' },
  lending:   { bg: 'rgba(222,201,244,.10)', bd: 'rgba(222,201,244,.25)', fg: '#DEC9F4' },
  liquidity: { bg: 'rgba(211,251,255,.10)', bd: 'rgba(211,251,255,.25)', fg: '#D3FBFF' },
};

export default function TypeBadge({ type }: TypeBadgeProps) {
  const short = type.replace(/Strategy$/i, '').toLowerCase();
  const s = TYPE_STYLES[short] ?? TYPE_STYLES.blend;

  return (
    <span
      style={{
        display: 'inline-flex',
        alignItems: 'center',
        padding: '3px 9px',
        borderRadius: 999,
        background: s.bg,
        border: `1px solid ${s.bd}`,
        color: s.fg,
        fontFamily: 'ui-monospace, SFMono-Regular, Menlo, monospace',
        fontSize: 11,
        fontWeight: 600,
        letterSpacing: '0.02em',
      }}
    >
      {short}
    </span>
  );
}
