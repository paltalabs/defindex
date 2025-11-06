// Design System - Single Source of Truth
// This file defines all design tokens used across the application

// ============================================================================
// LAYOUT CONSTANTS
// ============================================================================

export const SCROLL_OFFSET = -150;

export const CONTAINER_MAX_WIDTH = "calc(100vw - 2rem)";

export const BREAKPOINTS = {
  sm: "640px",
  md: "768px",
  lg: "1024px",
  xl: "1280px",
  "2xl": "1536px",
} as const;

// ============================================================================
// COLOR SYSTEM
// ============================================================================

export const COLORS = {
  // Brand colors
  primary: "#D3FFB4",
  primaryHover: "#E5FFCF",
  primaryDark: "#afd395",

  // Dark shades
  dark: "#033036",
  darkCyan: "#014751",
  darkCyan2: "#024852",

  // Light shades
  lightCyan: "#D3FBFF",
  lightCyanText: "#b3d5d8",
  white: "#FFFFFF",

  // Accent colors
  purple: "#DEC9F4",
  orange: "#FC5B31",
  orangeLight: "#FFDCD3",

  // Grays (for text and backgrounds)
  gray: {
    50: "#FFFFFF",
    100: "#F9FAFB",
    200: "#F3F4F6",
    300: "#E5E7EB",
    400: "#D1D5DB",
    500: "#9CA3AF",
    600: "#6B7280",
    700: "#4B5563",
    800: "#374151",
    900: "#1F2937",
  },
} as const;

// Semantic color mapping for easier usage
export const SEMANTIC_COLORS = {
  background: {
    primary: COLORS.dark,
    secondary: COLORS.darkCyan,
    light: COLORS.white,
    accent: COLORS.primary,
  },
  text: {
    primary: COLORS.white,
    secondary: COLORS.lightCyanText,
    dark: COLORS.dark,
    accent: COLORS.primary,
  },
  border: {
    primary: COLORS.primaryDark,
    light: COLORS.lightCyan,
    dark: COLORS.darkCyan,
  },
} as const;

// ============================================================================
// GRADIENTS
// ============================================================================

export const GRADIENTS = {
  // Primary gradients
  primary: "linear-gradient(137deg, #FFFFFF -20%, #DEC9F4 40%)",
  secondary: "linear-gradient(91deg, #FFF -20%, #DEC9F4 70%, #024852 101.37%)",
  tertiary: "linear-gradient(135deg, #D3FFB4 0%, #DEC9F4 50%, #024852 100%)",

  // Color-specific gradients
  purple: "linear-gradient(137deg, #DEC9F4 -25%, #024852 100%)",
  green: "linear-gradient(137deg, #D3FFB4 7%, #024852 100%)",
  orange: "linear-gradient(135deg, #FC5B31 0%, #FF8A6C 100%)",

  // Card gradients
  cardGreen: "linear-gradient(135deg, #D3FFB4 0%, #024852 100%)",
  cardPurple: "linear-gradient(135deg, rgba(211, 255, 180, 0.3) 0%, rgba(255, 255, 255, 1) 100%)",
  cardOrange: "linear-gradient(135deg, rgba(255, 220, 211, 1) 0%, rgba(255, 255, 255, 1) 100%)",

  // Background gradients
  darkRadial: "radial-gradient(circle at center, #014751 0%, #033036 100%)",
  lightRadial: "radial-gradient(circle at center, #D3FBFF 0%, #FFFFFF 100%)",
} as const;

// CSS-in-JS gradient objects for programmatic use
export const GRADIENT_OBJECTS = {
  primary: {
    background: GRADIENTS.primary,
    backgroundClip: "text",
    WebkitBackgroundClip: "text",
    WebkitTextFillColor: "transparent",
  },
  secondary: {
    background: GRADIENTS.secondary,
    backgroundClip: "text",
    WebkitBackgroundClip: "text",
    WebkitTextFillColor: "transparent",
  },
  tertiary: {
    background: GRADIENTS.tertiary,
    backgroundClip: "text",
    WebkitBackgroundClip: "text",
    WebkitTextFillColor: "transparent",
  },
  purple: {
    background: GRADIENTS.purple,
    backgroundClip: "text",
    WebkitBackgroundClip: "text",
    WebkitTextFillColor: "transparent",
  },
  green: {
    background: GRADIENTS.green,
    backgroundClip: "text",
    WebkitBackgroundClip: "text",
    WebkitTextFillColor: "transparent",
  },
} as const;

// ============================================================================
// TYPOGRAPHY
// ============================================================================

export const FONT_SIZES = {
  xs: { size: "16px", lineHeight: "3.75em" },
  sm: { size: "18px", lineHeight: "1.67em" },
  md: { size: "20px", lineHeight: "1.5em" },
  base: { size: "22px", lineHeight: "1.27em" },
  lg: { size: "24px", lineHeight: "1.5em" },
  xl: { size: "32px", lineHeight: "1.25em" },
  "2xl": { size: "48px", lineHeight: "1.2em" },
  "3xl": { size: "60px", lineHeight: "1.15em" },
  "4xl": { size: "72px", lineHeight: "1.11em" },
  "5xl": { size: "80px", lineHeight: "1.05em" },
  "6xl": { size: "96px", lineHeight: "1em" },
  "7xl": { size: "140px", lineHeight: "0.86em" },
} as const;

export const FONT_WEIGHTS = {
  normal: 400,
  medium: 500,
  semibold: 600,
  bold: 700,
  extrabold: 800,
} as const;

export const FONT_FAMILIES = {
  manrope: '"Manrope", sans-serif',
  familjenGrotesk: '"Familjen Grotesk", sans-serif',
  interTight: '"Inter Tight", sans-serif',
  inter: '"Inter", sans-serif',
} as const;

export const LETTER_SPACING = {
  tighter: "-0.05em",
  tight: "-0.025em",
  normal: "0em",
  wide: "0.025em",
  wider: "0.05em",
  widest: "0.1em",
} as const;

// ============================================================================
// SPACING SYSTEM
// ============================================================================

export const SPACING = {
  // Component spacing
  xs: "0.25rem",    // 4px
  sm: "0.5rem",     // 8px
  md: "1rem",       // 16px
  lg: "1.5rem",     // 24px
  xl: "2rem",       // 32px
  "2xl": "3rem",    // 48px
  "3xl": "4rem",    // 64px
  "4xl": "6rem",    // 96px
  "5xl": "8rem",    // 128px

  // Semantic spacing
  section: "4rem",
  sectionMobile: "2rem",
  container: "2rem",
  card: "1.5rem",
  button: "1rem",
} as const;

// ============================================================================
// BORDER RADIUS
// ============================================================================

export const RADIUS = {
  none: "0",
  sm: "0.5rem",     // 8px
  md: "1rem",       // 16px
  lg: "1.5rem",     // 24px
  xl: "2rem",       // 32px
  "2xl": "2.5rem",  // 40px
  "3xl": "3rem",    // 48px
  full: "9999px",
} as const;

// ============================================================================
// SHADOWS
// ============================================================================

export const SHADOWS = {
  sm: "37px 37px 37px rgba(0, 0, 0, 0.1)",
  md: "0px 5px 16px rgba(8, 15, 52, 0.06)",
  lg: "0px 3px 10px rgba(0, 0, 0, 0.15)",
  xl: "-16px 18px 39px rgba(218, 242, 236, 0.16)",

  // Glass effect shadows
  glass: "0 8px 32px 0 rgba(31, 38, 135, 0.37)",
  glassInset: "inset 0 0 0 1px rgba(255, 255, 255, 0.18)",
} as const;

// ============================================================================
// TRANSITIONS & ANIMATIONS
// ============================================================================

export const TRANSITIONS = {
  fast: "150ms",
  normal: "200ms",
  slow: "300ms",
  slower: "500ms",
} as const;

export const EASINGS = {
  default: "ease",
  linear: "linear",
  in: "ease-in",
  out: "ease-out",
  inOut: "ease-in-out",
  bounce: "cubic-bezier(0.68, -0.55, 0.265, 1.55)",
  smooth: "cubic-bezier(0.4, 0, 0.2, 1)",
} as const;

// ============================================================================
// INTERACTION EFFECTS
// ============================================================================

export const EFFECTS = {
  hover: {
    scale: 1.05,
    scaleSmall: 1.02,
    scaleLarge: 1.1,
    opacity: 0.9,
    brightness: 1.1,
  },
  active: {
    scale: 0.95,
    scaleSmall: 0.98,
    opacity: 0.8,
  },
  tap: {
    scale: 0.97,
  },
  disabled: {
    opacity: 0.5,
    cursor: "not-allowed",
  },
} as const;

// ============================================================================
// COMPLEX STYLES (CSS-in-JS Objects)
// ============================================================================

// Glassmorphism effect
export const GLASS_STYLES = {
  default: {
    background: "rgba(255, 255, 255, 0.1)",
    backdropFilter: "blur(10px)",
    WebkitBackdropFilter: "blur(10px)",
    border: "1px solid rgba(255, 255, 255, 0.18)",
    boxShadow: SHADOWS.glass,
  },
  strong: {
    background: "rgba(255, 255, 255, 0.2)",
    backdropFilter: "blur(20px)",
    WebkitBackdropFilter: "blur(20px)",
    border: "1px solid rgba(255, 255, 255, 0.3)",
    boxShadow: SHADOWS.glass,
  },
  dark: {
    background: "rgba(3, 48, 54, 0.3)",
    backdropFilter: "blur(10px)",
    WebkitBackdropFilter: "blur(10px)",
    border: "1px solid rgba(211, 255, 180, 0.2)",
    boxShadow: SHADOWS.glass,
  },
} as const;

// Button variants
export const BUTTON_STYLES = {
  primary: {
    background: COLORS.primary,
    color: COLORS.dark,
    border: "none",
    transition: `all ${TRANSITIONS.normal} ${EASINGS.smooth}`,
    "&:hover": {
      background: COLORS.primaryHover,
      transform: `scale(${EFFECTS.hover.scale})`,
    },
    "&:active": {
      transform: `scale(${EFFECTS.active.scale})`,
    },
  },
  secondary: {
    background: "transparent",
    color: COLORS.white,
    border: `2px solid ${COLORS.primary}`,
    transition: `all ${TRANSITIONS.normal} ${EASINGS.smooth}`,
    "&:hover": {
      background: COLORS.primary,
      color: COLORS.dark,
      transform: `scale(${EFFECTS.hover.scale})`,
    },
    "&:active": {
      transform: `scale(${EFFECTS.active.scale})`,
    },
  },
  ghost: {
    background: "transparent",
    color: COLORS.primary,
    border: "none",
    transition: `all ${TRANSITIONS.normal} ${EASINGS.smooth}`,
    "&:hover": {
      background: "rgba(211, 255, 180, 0.1)",
      transform: `scale(${EFFECTS.hover.scale})`,
    },
    "&:active": {
      transform: `scale(${EFFECTS.active.scale})`,
    },
  },
} as const;

// Card styles
export const CARD_STYLES = {
  default: {
    background: COLORS.white,
    borderRadius: RADIUS.xl,
    padding: SPACING.xl,
    boxShadow: SHADOWS.md,
    transition: `all ${TRANSITIONS.normal} ${EASINGS.smooth}`,
  },
  interactive: {
    background: COLORS.white,
    borderRadius: RADIUS.xl,
    padding: SPACING.xl,
    boxShadow: SHADOWS.md,
    transition: `all ${TRANSITIONS.normal} ${EASINGS.smooth}`,
    cursor: "pointer",
    "&:hover": {
      transform: `translateY(-4px) scale(${EFFECTS.hover.scaleSmall})`,
      boxShadow: SHADOWS.xl,
    },
  },
  glass: {
    ...GLASS_STYLES.default,
    borderRadius: RADIUS.xl,
    padding: SPACING.xl,
  },
} as const;

// ============================================================================
// ICON FILTERS
// ============================================================================

export const ICON_FILTER = {
  dark: "brightness(0) saturate(100%) invert(15%) sepia(82%) saturate(1234%) hue-rotate(181deg) brightness(94%) contrast(101%)",
  white: "brightness(0) saturate(100%) invert(100%) sepia(0%) saturate(0%) hue-rotate(0deg) brightness(100%) contrast(100%)",
  green: "brightness(0) saturate(100%) invert(49%) sepia(96%) saturate(445%) hue-rotate(75deg) brightness(108%) contrast(101%)",
  purple: "brightness(0) saturate(100%) invert(85%) sepia(18%) saturate(1054%) hue-rotate(229deg) brightness(101%) contrast(93%)",
  orange: "brightness(0) saturate(100%) invert(49%) sepia(73%) saturate(2613%) hue-rotate(343deg) brightness(101%) contrast(98%)",
} as const;

// ============================================================================
// Z-INDEX SCALE
// ============================================================================

export const Z_INDEX = {
  base: 0,
  dropdown: 1000,
  sticky: 1020,
  fixed: 1030,
  modalBackdrop: 1040,
  modal: 1050,
  popover: 1060,
  tooltip: 1070,
} as const;

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/**
 * Get responsive font size styles
 */
export const getResponsiveFontSize = (
  mobile: keyof typeof FONT_SIZES,
  desktop: keyof typeof FONT_SIZES
) => ({
  fontSize: FONT_SIZES[mobile].size,
  lineHeight: FONT_SIZES[mobile].lineHeight,
  "@media (min-width: 768px)": {
    fontSize: FONT_SIZES[desktop].size,
    lineHeight: FONT_SIZES[desktop].lineHeight,
  },
});

/**
 * Get hover styles
 */
export const getHoverStyles = (scale: number = EFFECTS.hover.scale) => ({
  transition: `all ${TRANSITIONS.normal} ${EASINGS.smooth}`,
  "&:hover": {
    transform: `scale(${scale})`,
  },
  "&:active": {
    transform: `scale(${EFFECTS.active.scale})`,
  },
});

/**
 * Get focus ring styles
 */
export const getFocusRingStyles = (color: string = COLORS.primary) => ({
  outline: "none",
  "&:focus-visible": {
    outline: `2px solid ${color}`,
    outlineOffset: "2px",
  },
});
