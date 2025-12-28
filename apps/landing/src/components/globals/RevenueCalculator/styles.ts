/**
 * Centralized styles for Revenue Calculator components
 * Modify values here to update styles across all components
 */

// =============================================================================
// COLORS
// =============================================================================

export const colors = {
  // Primary accent color (lime green)
  primary: '#D3FFB4',
  primaryRgb: '211, 255, 180',

  // Secondary accent (purple)
  secondary: '#C084FC',
  secondaryRgb: '192, 132, 252',

  // Background teal tones
  bgDark: 'rgba(3, 48, 54, 1)',
  bgMedium: 'rgba(4, 74, 84, 1)',
  bgLight: 'rgba(1, 71, 81, 1)',

  // Border colors
  border: 'rgba(8, 145, 165, 0.5)', // cyan-800/50
  borderHover: 'rgba(8, 145, 165, 0.7)',

  // Text colors
  textPrimary: 'white',
  textMuted: 'rgba(255, 255, 255, 0.75)',
  textSubtle: 'rgba(255, 255, 255, 0.60)',
  textFaint: 'rgba(255, 255, 255, 0.50)',

  // Status colors
  error: '#fb923c', // orange-400
} as const;

// =============================================================================
// GRADIENTS
// =============================================================================

export const gradients = {
  // Card background - standard
  card: 'linear-gradient(135deg, rgba(3, 48, 54, 0.8) 0%, rgba(1, 71, 81, 0.5) 100%)',

  // Card background - darker (for forms)
  cardDark: 'linear-gradient(135deg, rgba(3, 48, 54, 0.9) 0%, rgba(1, 71, 81, 0.6) 100%)',

  // Calculator inputs panel
  inputsPanel: 'linear-gradient(115deg, rgba(4, 74, 84, 1) 0%, rgba(3, 48, 54, 1) 100%)',

  // Integration cost input
  costInput: 'linear-gradient(135deg, rgba(6, 78, 88, 0.6) 0%, rgba(4, 58, 68, 0.8) 100%)',

  // Selected projection card
  selectedCard: 'linear-gradient(145deg, rgba(79, 121, 102, 0.8) 0%, rgba(45, 85, 75, 0.6) 50%, rgba(30, 60, 55, 0.7) 100%)',
} as const;

// =============================================================================
// SHADOWS
// =============================================================================

export const shadows = {
  // Primary glow (lime)
  primaryGlow: `0 0 12px rgba(${colors.primaryRgb}, 0.5), 0 0 24px rgba(${colors.primaryRgb}, 0.25)`,
  primaryGlowSm: `0 0 6px rgba(${colors.primaryRgb}, 0.6)`,
  primaryGlowMd: `0 0 10px rgba(${colors.primaryRgb}, 0.6), 0 0 20px rgba(${colors.primaryRgb}, 0.3)`,
  primaryGlowLg: `0 0 15px rgba(${colors.primaryRgb}, 0.8), 0 0 30px rgba(${colors.primaryRgb}, 0.4)`,

  // Secondary glow (purple)
  secondaryGlow: `0 0 6px rgba(${colors.secondaryRgb}, 0.6)`,

  // Card selected shadow
  cardSelected: `0 0 15px rgba(${colors.primaryRgb}, 0.3)`,

  // Subtle cyan glow
  cyanGlow: '0 0 15px rgba(6, 182, 212, 0.1)',
} as const;

// =============================================================================
// CSS CLASSES (Tailwind)
// =============================================================================

export const classes = {
  // Card base
  card: 'rounded-2xl border border-cyan-800/50',
  cardPadding: 'p-6 md:p-8',

  // Text styles
  heading: 'font-familjen-grotesk font-bold text-white',
  headingLg: 'text-lg font-familjen-grotesk font-semibold text-white',
  label: 'text-lg font-manrope font-bold text-white',
  labelSm: 'text-sm font-medium text-white/75',
  textMuted: 'text-white/60',
  textSubtle: 'text-white/50',
  textXs: 'text-xs text-white/50',

  // Input styles
  input: 'w-full h-12 px-4 bg-cyan-950 border border-cyan-800 rounded-lg text-white placeholder-white/40 focus:outline-none focus:ring-2 focus:ring-lime-200',
  inputError: 'border-orange-400',

  // Button styles
  buttonPrimary: 'w-full h-12 bg-lime-200 text-cyan-900 font-semibold rounded-lg hover:bg-lime-300 transition-colors disabled:opacity-50 disabled:cursor-not-allowed',
  buttonSecondary: 'bg-cyan-950 text-white border border-cyan-800 hover:bg-cyan-900 hover:border-cyan-600',

  // Accent text
  accentText: 'text-lime-200',
} as const;

// =============================================================================
// SLIDER STYLES (CSS-in-JS)
// =============================================================================

export const sliderStyles = `
  .custom-slider {
    -webkit-appearance: none;
    appearance: none;
    width: 100%;
    height: 6px;
    border-radius: 9999px;
    outline: none;
    cursor: pointer;
  }

  .custom-slider::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    width: 20px;
    height: 20px;
    border-radius: 50%;
    background: ${colors.primary};
    cursor: pointer;
    box-shadow: ${shadows.primaryGlowMd};
    border: 2px solid rgba(255, 255, 255, 0.2);
    transition: box-shadow 0.2s ease;
  }

  .custom-slider::-webkit-slider-thumb:hover {
    box-shadow: ${shadows.primaryGlowLg};
  }

  .custom-slider::-moz-range-thumb {
    width: 20px;
    height: 20px;
    border-radius: 50%;
    background: ${colors.primary};
    cursor: pointer;
    box-shadow: ${shadows.primaryGlowMd};
    border: 2px solid rgba(255, 255, 255, 0.2);
    transition: box-shadow 0.2s ease;
  }

  .custom-slider::-moz-range-thumb:hover {
    box-shadow: ${shadows.primaryGlowLg};
  }

  .custom-slider::-moz-range-track {
    height: 6px;
    border-radius: 9999px;
  }
`;

// =============================================================================
// HELPER FUNCTIONS
// =============================================================================

/**
 * Calculate slider fill background gradient
 */
export const getSliderBackground = (value: number, min: number, max: number): string => {
  const percentage = ((value - min) / (max - min)) * 100;
  return `linear-gradient(90deg, ${colors.primary} 0%, ${colors.primary} ${percentage}%, rgba(6, 95, 105, 0.8) ${percentage}%, rgba(8, 75, 85, 0.6) 100%)`;
};

/**
 * Get styles for selected button/card
 */
export const getSelectedStyles = () => ({
  background: colors.primary,
  boxShadow: shadows.primaryGlow,
});

/**
 * Get styles for selected projection card
 */
export const getSelectedCardStyles = () => ({
  background: gradients.selectedCard,
  boxShadow: shadows.cardSelected,
});

/**
 * Get indicator dot styles
 */
export const getIndicatorDot = (color: 'primary' | 'secondary') => ({
  background: color === 'primary' ? colors.primary : colors.secondary,
  boxShadow: color === 'primary' ? shadows.primaryGlowSm : shadows.secondaryGlow,
});
