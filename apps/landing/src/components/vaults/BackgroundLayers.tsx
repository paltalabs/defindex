export default function BackgroundLayers() {
  return (
    <>
      {/* Flat teal base */}
      <div
        style={{
          position: 'fixed',
          inset: 0,
          zIndex: 0,
          pointerEvents: 'none',
          background: '#033036',
        }}
      />

      {/* Top vignette */}
      <div
        style={{
          position: 'fixed',
          inset: 0,
          zIndex: 0,
          pointerEvents: 'none',
          background:
            'radial-gradient(ellipse 1200px 700px at 50% -10%, rgba(1,71,81,0.6) 0%, rgba(3,48,54,0) 70%)',
        }}
      />

      {/* Ambient glows */}
      <div
        style={{
          position: 'fixed',
          inset: 0,
          zIndex: 0,
          pointerEvents: 'none',
          overflow: 'hidden',
        }}
      >
        {/* Glow A — top-right lime */}
        <div
          style={{
            position: 'absolute',
            top: -200,
            right: -120,
            width: 760,
            height: 760,
            borderRadius: '50%',
            background:
              'radial-gradient(circle, rgba(165,210,159,.14) 0%, rgba(165,210,159,0) 65%)',
            filter: 'blur(60px)',
          }}
        />
        {/* Glow B — mid-left lavender */}
        <div
          style={{
            position: 'absolute',
            top: 80,
            left: -240,
            width: 600,
            height: 600,
            borderRadius: '50%',
            background:
              'radial-gradient(circle, rgba(222,201,244,.10) 0%, rgba(222,201,244,0) 65%)',
            filter: 'blur(60px)',
          }}
        />
      </div>

      {/* Concentric rings — bottom-left motif */}
      <svg
        viewBox="0 0 1200 1200"
        preserveAspectRatio="xMidYMid meet"
        style={{
          position: 'fixed',
          left: -200,
          bottom: -300,
          width: 1200,
          height: 1200,
          zIndex: 0,
          pointerEvents: 'none',
          opacity: 0.18,
        }}
      >
        <g transform="translate(600 600)">
          {[120, 220, 320, 420, 520, 600].map(r => (
            <circle key={r} r={r} fill="none" stroke="#5fa39a" strokeWidth="1" />
          ))}
        </g>
      </svg>
    </>
  );
}
