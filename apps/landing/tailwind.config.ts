import type { Config } from "tailwindcss";
import {
  COLORS,
  FONT_SIZES,
  SHADOWS,
  GRADIENTS,
  RADIUS,
  SPACING,
  TRANSITIONS,
} from "./src/constants/design";

const config: Config = {
    content: [
        "./src/pages/**/*.{js,ts,jsx,tsx,mdx}",
        "./src/components/**/*.{js,ts,jsx,tsx,mdx}",
        "./src/app/**/*.{js,ts,jsx,tsx,mdx}",
    ],
    theme: {
        extend: {
            // Colors from design system
            colors: {
                // Brand colors with semantic aliases
                brand: {
                    primary: COLORS.primary,
                    "primary-hover": COLORS.primaryHover,
                    "primary-dark": COLORS.primaryDark,
                    dark: COLORS.dark,
                    "dark-cyan": COLORS.darkCyan,
                    "dark-cyan-2": COLORS.darkCyan2,
                    "light-cyan": COLORS.lightCyan,
                    "light-cyan-text": COLORS.lightCyanText,
                    purple: COLORS.purple,
                    orange: COLORS.orange,
                    "orange-light": COLORS.orangeLight,
                },
                // Legacy aliases for backward compatibility
                gray: {
                    ...COLORS.gray
                },
                orange: { "500": COLORS.orange },
                lime: { "200": COLORS.primary },
                cyan: {
                    "950": COLORS.darkCyan,
                    "900": COLORS.dark
                },
                blue: { "100": COLORS.lightCyan },
            },
            // Border radius from design system
            borderRadius: {
                ...RADIUS,
            },
            // Spacing scale
            spacing: {
                ...SPACING,
            },
            // Transitions
            transitionDuration: {
                fast: TRANSITIONS.fast,
                normal: TRANSITIONS.normal,
                slow: TRANSITIONS.slow,
                slower: TRANSITIONS.slower,
            },
        },
        // Font sizes from design system
        fontSize: {
            xs: [FONT_SIZES.xs.size, { lineHeight: FONT_SIZES.xs.lineHeight }],
            sm: [FONT_SIZES.sm.size, { lineHeight: FONT_SIZES.sm.lineHeight }],
            md: [FONT_SIZES.md.size, { lineHeight: FONT_SIZES.md.lineHeight }],
            base: [FONT_SIZES.base.size, { lineHeight: FONT_SIZES.base.lineHeight }],
            lg: [FONT_SIZES.lg.size, { lineHeight: FONT_SIZES.lg.lineHeight }],
            xl: [FONT_SIZES.xl.size, { lineHeight: FONT_SIZES.xl.lineHeight }],
            "2xl": [FONT_SIZES["2xl"].size, { lineHeight: FONT_SIZES["2xl"].lineHeight }],
            "3xl": [FONT_SIZES["3xl"].size, { lineHeight: FONT_SIZES["3xl"].lineHeight }],
            "4xl": [FONT_SIZES["4xl"].size, { lineHeight: FONT_SIZES["4xl"].lineHeight }],
            "5xl": [FONT_SIZES["5xl"].size, { lineHeight: FONT_SIZES["5xl"].lineHeight }],
            "6xl": [FONT_SIZES["6xl"].size, { lineHeight: FONT_SIZES["6xl"].lineHeight }],
            "7xl": [FONT_SIZES["7xl"].size, { lineHeight: FONT_SIZES["7xl"].lineHeight }],
        },
        // Shadows from design system
        boxShadow: {
            ...SHADOWS,
        },
        // Background images/gradients from design system
        backgroundImage: {
            // Primary gradients
            linear: GRADIENTS.secondary,
            "gradient-primary": GRADIENTS.primary,
            "gradient-secondary": GRADIENTS.secondary,
            "gradient-tertiary": GRADIENTS.tertiary,
            "gradient-purple": GRADIENTS.purple,
            "gradient-green": GRADIENTS.green,
            "gradient-orange": GRADIENTS.orange,

            // Card gradients
            "gradient-card-green": GRADIENTS.cardGreen,
            "gradient-card-purple": GRADIENTS.cardPurple,
            "gradient-card-orange": GRADIENTS.cardOrange,

            // Background gradients
            "gradient-dark-radial": GRADIENTS.darkRadial,
            "gradient-light-radial": GRADIENTS.lightRadial,

            // Legacy gradients for backward compatibility
            "item-linear":
                "linear-gradient(180deg, rgba(255, 220, 211, 0.4) 0%, rgba(255, 255, 255, 0.4) 148.8%)",
            "item-linear-green":
                "linear-gradient(176deg, rgb(211 255 180 / 40%) 3.5%, rgb(255 255 255 / 40%) 113.2%)",
        },
        // Font families (keep using CSS variables)
        fontFamily: {
            manrope: ["var(--font-manrope)", "serif"],
            "familjen-grotesk": ["var(--font-familjen_grotesk)", "serif"],
            "inter-tight": ["var(--font-inter_tight)", "serif"],
            inter: ["var(--font-inter)", "serif"],
        },
        // Container configuration
        container: {
            center: true,
            padding: "1rem",
            screens: {
                sm: '100%',
                md: '100%',
                lg: '100%',
                xl: '100%',
                '2xl': '100%',
            },
        },
    },
    plugins: [],
};
export default config;
