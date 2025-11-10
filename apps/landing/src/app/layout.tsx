import { OrganizationSchema, SoftwareApplicationSchema, WebSiteSchema } from "@/components/SEO/JsonLd";
import { PostHogProvider } from "@/components/providers/provider";
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
    title: "DeFindex - Stellar Yield Infrastructure for Wallets & DeFi Apps | Stablecoin Vaults SDK",
    description: "Plug-and-play yield infrastructure for Stellar wallets and DeFi apps. Automated stablecoin vault SDKs built on Soroban with 80% revenue share. Integrate in hours, not months.",
    icons: {
        icon: favicon,
    },
    keywords: ["Stellar", "Soroban", "DeFi", "yield", "stablecoin", "vaults", "SDK", "wallet integration", "blockchain"],
    authors: [{ name: "DeFindex" }],
    creator: "DeFindex",
    publisher: "DeFindex",
    openGraph: {
        type: "website",
        locale: "en_US",
        url: "https://defindex.io",
        title: "DeFindex - Stellar Yield Infrastructure for Wallets & DeFi Apps",
        description: "Plug-and-play yield infrastructure for Stellar wallets and DeFi apps. Built on Soroban with 80% revenue share.",
        siteName: "DeFindex",
    },
    twitter: {
        card: "summary_large_image",
        title: "DeFindex - Stellar Yield Infrastructure",
        description: "Plug-and-play yield infrastructure for Stellar wallets and DeFi apps. Built on Soroban with 80% revenue share.",
        creator: "@defindex_",
    },
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
    return (
        <html lang="en">
            <head>
                <SoftwareApplicationSchema />
                <OrganizationSchema />
                <WebSiteSchema />
            </head>
            <body className={`${fonts} max-w-[100dvw]`}>
                <PostHogProvider>

                    {children}
                </PostHogProvider>
            </body>
        </html>
    );
}
