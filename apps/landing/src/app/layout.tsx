import type { Metadata } from "next";
import { Familjen_Grotesk, Inter, Inter_Tight, Manrope } from "next/font/google";
import "../styles/main.css";

const manrope = Manrope({
    subsets: ["latin"],
    weight: ["400", "700", "800"],
    variable: "--font-manrope",
});

const familjenGrotesk = Familjen_Grotesk({
    subsets: ["latin"],
    weight: ["400", "500", "600", "700"],
    variable: "--font-familjen_grotesk",
});

const interTight = Inter_Tight({
    subsets: ["latin"],
    weight: ["400", "600", "800"],
    variable: "--font-inter_tight",
});

const inter = Inter({
    subsets: ["latin"],
    weight: ["400"],
    variable: "--font-inter",
});

const favicon = "/images/favicon.ico";

const fonts = [manrope, familjenGrotesk, interTight, inter].map((font) => font.variable).join(" ");

export const metadata: Metadata = {
    title: "Defindex",
    description: "Diversified DeFi strategies for your users.",
    icons: {
        icon: favicon,
    },
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
    return (
        <html lang="en">
            <body className={`${fonts} max-w-[100dvw]`}>{children}</body>
        </html>
    );
}
