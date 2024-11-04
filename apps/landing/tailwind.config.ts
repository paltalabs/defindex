import type { Config } from "tailwindcss";
import { fontFamily } from "tailwindcss/defaultTheme";

const config: Config = {
    content: [
        "./src/pages/**/*.{js,ts,jsx,tsx,mdx}",
        "./src/components/**/*.{js,ts,jsx,tsx,mdx}",
        "./src/app/**/*.{js,ts,jsx,tsx,mdx}",
    ],
    theme: {
        extend: {
            colors: {
                gray: { "50": "#ffffff" },
                orange: { "500": "#FC5B31" },
                lime: { "200": "#D3FFB4" },
                cyan: { "950": "#014751", "900": "#033036" },
                blue: { "100": "#D3FBFF" },
            },
        },
        fontSize: {
            xs: ["16px", { lineHeight: "3.75em" }],
            sm: ["18px", { lineHeight: "1.67em" }],
            md: ["20px", { lineHeight: "1.5em" }],
            base: ["22px", { lineHeight: "1.27em" }],
            lg: ["24px", { lineHeight: "1.5em" }],
            xl: ["64px", { lineHeight: "1em" }],
            "3xl": ["72px", { lineHeight: "1.11em" }],
            "4xl": ["140px", { lineHeight: "0.86em" }],
        },
        boxShadow: {
            sm: "37px 37px 37px",
            md: "0px 5px 16px rgba(8.24, 15.25, 52.06, 0.06)",
            lg: "0px 3px 10px rgba(0, 0, 0, 0.15)",
            xl: "-16px 18px 39px rgba(218, 242, 236, 0.16)",
        },
        backgroundImage: {
            linear: "linear-gradient(91deg, #FFF -5.52%, #DEC9F4 84.31%, #024852 101.37%)",
            "item-linear":
                "linear-gradient(180deg, rgba(255, 220, 211, 0.4) 0%, rgba(255, 255, 255, 0.4) 148.8%)",
            "item-linear-green":
                "linear-gradient(176deg, rgb(211 255 180 / 40%) 3.5%, rgb(255 255 255 / 40%) 113.2%)",
        },
        fontFamily: {
            manrope: ["var(--font-manrope)", "serif"],
            "familjen-grotesk": ["var(--font-familjen_grotesk)", "serif"],
            "inter-tight": ["var(--font-inter_tight)", "serif"],
            inter: ["var(--font-inter)", "serif"],
        },
        container: {
            center: true,
            padding: "1rem",
        },
    },
    plugins: [],
};
export default config;
