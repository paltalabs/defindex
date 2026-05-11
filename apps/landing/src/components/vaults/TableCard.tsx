'use client';

export const GLASS_CARD = {
  background: 'rgba(29,57,62,.40)',
  border: '1px solid rgba(193,200,201,.10)',
  borderRadius: 20,
  overflow: 'hidden',
  backdropFilter: 'blur(14px)',
  WebkitBackdropFilter: 'blur(14px)',
} as const;

export function HeaderCell({
  children,
  width,
  align = 'left',
}: {
  children: React.ReactNode;
  width?: string;
  align?: 'left' | 'right';
}) {
  return (
    <div
      style={{
        flex: width ? `0 0 ${width}` : 1,
        textAlign: align,
        fontSize: 12,
        fontWeight: 600,
        letterSpacing: '0.04em',
        textTransform: 'uppercase',
        color: 'rgba(255,255,255,.4)',
      }}
    >
      {children}
    </div>
  );
}
