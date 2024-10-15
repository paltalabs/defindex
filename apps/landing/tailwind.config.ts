import type { Config } from "tailwindcss";
import { fontFamily } from "tailwindcss/defaultTheme";

import tailwindcss_animate from "tailwindcss-animate";

const config: Config = {
    darkMode: ["class"],
    content: [
        "./src/pages/**/*.{js,ts,jsx,tsx,mdx}",
        "./src/components/**/*.{js,ts,jsx,tsx,mdx}",
        "./src/app/**/*.{js,ts,jsx,tsx,mdx}",
    ],
    theme: {
    	extend: {
    		colors: {
    			gray: {
    				'50': '#ffffff'
    			},
    			orange: {
    				'500': '#FC5B31'
    			},
    			lime: {
    				'200': '#D3FFB4'
    			},
    			cyan: {
    				'900': '#033036',
    				'950': '#014751'
    			},
    			blue: {
    				'100': '#D3FBFF'
    			},
    			background: 'hsl(var(--background))',
    			foreground: 'hsl(var(--foreground))',
    			card: {
    				DEFAULT: 'hsl(var(--card))',
    				foreground: 'hsl(var(--card-foreground))'
    			},
    			popover: {
    				DEFAULT: 'hsl(var(--popover))',
    				foreground: 'hsl(var(--popover-foreground))'
    			},
    			primary: {
    				DEFAULT: 'hsl(var(--primary))',
    				foreground: 'hsl(var(--primary-foreground))'
    			},
    			secondary: {
    				DEFAULT: 'hsl(var(--secondary))',
    				foreground: 'hsl(var(--secondary-foreground))'
    			},
    			muted: {
    				DEFAULT: 'hsl(var(--muted))',
    				foreground: 'hsl(var(--muted-foreground))'
    			},
    			accent: {
    				DEFAULT: 'hsl(var(--accent))',
    				foreground: 'hsl(var(--accent-foreground))'
    			},
    			destructive: {
    				DEFAULT: 'hsl(var(--destructive))',
    				foreground: 'hsl(var(--destructive-foreground))'
    			},
    			border: 'hsl(var(--border))',
    			input: 'hsl(var(--input))',
    			ring: 'hsl(var(--ring))',
    			chart: {
    				'1': 'hsl(var(--chart-1))',
    				'2': 'hsl(var(--chart-2))',
    				'3': 'hsl(var(--chart-3))',
    				'4': 'hsl(var(--chart-4))',
    				'5': 'hsl(var(--chart-5))'
    			}
    		},
    		borderRadius: {
    			lg: 'var(--radius)',
    			md: 'calc(var(--radius) - 2px)',
    			sm: 'calc(var(--radius) - 4px)'
    		},
    		animation: {
    			orbit: 'orbit calc(var(--duration)*1s) linear infinite',
    			'border-beam': 'border-beam calc(var(--duration)*1s) infinite linear'
    		},
    		keyframes: {
    			orbit: {
    				'0%': {
    					transform: 'rotate(0deg) translateY(calc(var(--radius) * 1px)) rotate(0deg)'
    				},
    				'100%': {
    					transform: 'rotate(360deg) translateY(calc(var(--radius) * 1px)) rotate(-360deg)'
    				}
    			},
    			'border-beam': {
    				'100%': {
    					'offset-distance': '100%'
    				}
    			}
    		}
    	},
    	fontSize: {
    		xs: ["16px", { lineHeight: "3.75em" }],
    		sm: ["18px", { lineHeight: "1.67em" }],
    		md: ["20px", { lineHeight: "1.5em" }],
    		base: ["22px", { lineHeight: "1.27em" }],
    		lg: ["24px", { lineHeight: "1.5em" }],
    		xl: ["64px", { lineHeight: "1em" }],
    		'3xl': ["72px", { lineHeight: "1.11em" }],
    		'4xl': ["140px", { lineHeight: "0.86em" }]
    	},
    	boxShadow: {
    		sm: '37px 37px 37px',
    		md: '0px 5px 16px rgba(8.24, 15.25, 52.06, 0.06)',
    		lg: '0px 3px 10px rgba(0, 0, 0, 0.15)',
    		xl: '-16px 18px 39px rgba(218, 242, 236, 0.16)'
    	},
    	backgroundImage: {
    		linear: 'linear-gradient(91deg, #FFF -5.52%, #DEC9F4 84.31%, #024852 101.37%)',
    		'item-linear': 'linear-gradient(180deg, rgba(255, 220, 211, 0.4) 0%, rgba(255, 255, 255, 0.4) 148.8%)',
    		'item-linear-green': 'linear-gradient(176deg, rgb(211 255 180 / 40%) 3.5%, rgb(255 255 255 / 40%) 113.2%)'
    	},
    	fontFamily: {
    		manrope: ["var(--font-manrope)", ...fontFamily.sans],
    		'familjen-grotesk': ["var(--font-familjen_grotesk)", ...fontFamily.sans],
    		'inter-tight': ["var(--font-inter_tight)", ...fontFamily.sans],
    		inter: ["var(--font-inter)", ...fontFamily.sans]
    	},
    	container: {
    		center: 'true',
    		padding: '1rem'
    	}
    },
    plugins: [tailwindcss_animate],
};
export default config;
