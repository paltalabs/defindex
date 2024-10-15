import "../styles/main.css";
import type { Metadata } from "next";
import { Manrope, Familjen_Grotesk, Inter_Tight, Inter } from "next/font/google";

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

const fonts = [manrope, familjenGrotesk, interTight, inter].map((font) => font.variable).join(" ");

export const metadata: Metadata = {
    title: "Defindex",
    description: "Diversified DeFi strategies for your users.",
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
    return (
        <html lang="en">
            <body className={fonts}>{children}</body>
        </html>
    );
}
