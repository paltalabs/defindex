import React from 'react';

export interface GlassIconsItem {
  icon?: React.ReactElement;
  text?: string;
  color: string;
  label: string;
  customClass?: string;
}

export interface GlassIconsProps {
  items: GlassIconsItem[];
  className?: string;
}

const gradientMapping: Record<string, string> = {
  blue: 'linear-gradient(hsl(223, 90%, 50%), hsl(208, 90%, 50%))',
  purple: 'linear-gradient(hsl(283, 90%, 50%), hsl(268, 90%, 50%))',
  red: 'linear-gradient(hsl(3, 90%, 50%), hsl(348, 90%, 50%))',
  indigo: 'linear-gradient(hsl(253, 90%, 50%), hsl(238, 90%, 50%))',
  orange: 'linear-gradient(hsl(43, 90%, 50%), hsl(28, 90%, 50%))',
  green: 'linear-gradient(rgba(211, 255, 180, 0.8), rgba(3, 48, 54, 0.1) 80%)'
};

const GlassIcons: React.FC<GlassIconsProps> = ({ items, className }) => {
  const getBackgroundStyle = (color: string): React.CSSProperties => {
    if (gradientMapping[color]) {
      return { background: gradientMapping[color] };
    }
    return { background: color };
  };

  return (
    <div className={`${items.length === 1 ? 'flex' : 'grid gap-[5em] grid-cols-2 md:grid-cols-3'} mx-auto py-[3em] overflow-visible ${className || ''}`}>
      {items.map((item, index) => (
        <button
          key={index}
          type="button"
          aria-label={item.label}
          className={`relative bg-transparent outline-none ${item.text ? 'w-[10em] h-[2.5em]' : 'w-[4.5em] h-[4.5em]'} [perspective:24em] [transform-style:preserve-3d] [-webkit-tap-highlight-color:transparent] group ${
            item.customClass || ''
          }`}
        >
          <span
            className="absolute top-0 left-0 w-full h-full rounded-[1.25em] block transition-[opacity,transform] duration-300 ease-[cubic-bezier(0.83,0,0.17,1)] origin-[100%_100%] rotate-[1deg] group-hover:[transform:rotate(1deg)_translate3d(-0.2em,-0.2em,0.2em)]"
            style={{
              ...getBackgroundStyle(item.color),
              boxShadow: '0.3em -0.3em 0.5em hsla(223, 10%, 10%, 0.15)'
            }}
          ></span>

          <span
            className="absolute top-0 left-0 w-full h-full rounded-[1.25em] bg-[hsla(0,0%,100%,0.15)] transition-[opacity,transform] duration-300 ease-[cubic-bezier(0.83,0,0.17,1)] origin-[80%_50%] flex items-center justify-center backdrop-blur-[0.75em] [-webkit-backdrop-filter:blur(0.75em)] transform group-hover:[transform:translateZ(0.5em)]"
            style={{
              boxShadow: '0 0 0 0.1em hsla(0, 0%, 100%, 0.3) inset'
            }}
          >
            <span className="flex items-center justify-center px-2" aria-hidden="true">
              {item.icon ? (
                <span className="w-[1.5em] h-[1.5em] flex items-center justify-center">{item.icon}</span>
              ) : (
                <span className="text-white font-bold text-xs whitespace-nowrap leading-none" style={{textShadow: '0 2px 4px rgba(0,0,0,0.2), 0 1px 2px rgba(0,0,0,0.2)'}}>{item.text}</span>
              )}
            </span>
          </span>
        </button>
      ))}
    </div>
  );
};

export default GlassIcons;
