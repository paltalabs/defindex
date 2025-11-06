// Design constants for consistent theming

export const SCROLL_OFFSET = -150;

export const CONTAINER_MAX_WIDTH = "calc(100vw - 2rem)";

export const COLORS = {
    primary: "#D3FFB4",
    dark: "#033036",
    darkCyan: "#014751",
    purple: "#DEC9F4",
    lightCyan: "#D3FBFF",
    white: "#FFFFFF",
} as const;

export const GRADIENTS = {
    primary: "linear-gradient(137deg, #FFFFFF 7%, #DEC9F4 100%)",
    secondary: "linear-gradient(91deg, #FFF -5.52%, #DEC9F4 84.31%, #024852 101.37%)",
    purple: "linear-gradient(137deg, #DEC9F4 7%, #024852 100%)",
    green: "linear-gradient(137deg, #D3FFB4 7%, #024852 100%)",
    cardGreen: "linear-gradient(135deg, #D3FFB4 0%, #024852 100%)",
    cardPurple: "linear-gradient(135deg, rgba(211, 255, 180, 0.3) 0%, rgba(255, 255, 255, 1) 100%)",
    cardOrange: "linear-gradient(135deg, rgba(255, 220, 211, 1) 0%, rgba(255, 255, 255, 1) 100%)",
} as const;

export const ICON_FILTER = {
    dark: "brightness(0) saturate(100%) invert(15%) sepia(82%) saturate(1234%) hue-rotate(181deg) brightness(94%) contrast(101%)",
    white: "brightness(0) saturate(100%) invert(100%) sepia(0%) saturate(0%) hue-rotate(0deg) brightness(100%) contrast(100%)",
    green: "brightness(0) saturate(100%) invert(49%) sepia(96%) saturate(445%) hue-rotate(75deg) brightness(108%) contrast(101%)",
} as const;
