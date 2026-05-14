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
    title: "DeFindex: Yield Infrastructure for Wallets, Neobanks & Fintech Apps",
    description: "DeFindex plugs stablecoin savings into your app via API in hours. 100% non-custodial. REST API only — no smart contract work needed. 7 partners live across LATAM, EMEA and APAC.",
    icons: {
        icon: favicon,
    },
    keywords: ["yield infrastructure", "stablecoin savings", "wallet API", "neobank", "fintech", "USDC yield", "non-custodial", "REST API", "remittance", "DeFi"],
    authors: [{ name: "DeFindex" }],
    creator: "DeFindex",
    publisher: "DeFindex",
    openGraph: {
        type: "website",
        locale: "en_US",
        url: "https://defindex.io",
        title: "DeFindex: Yield Infrastructure for Wallets, Neobanks & Fintech Apps",
        description: "Plug stablecoin savings into your app via API in hours. 100% non-custodial. 7 partners live across LATAM, EMEA and APAC.",
        siteName: "DeFindex",
        images: [
            {
                url: "https://defindex.io/images/glass-02.png",
                width: 1200,
                height: 630,
                alt: "DeFindex — Yield Infrastructure for Wallets, Neobanks & Fintech Apps",
            },
        ],
    },
    twitter: {
        card: "summary_large_image",
        title: "DeFindex: Yield Infrastructure for Wallets, Neobanks & Fintech Apps",
        description: "Plug stablecoin savings into your app via API in hours. 100% non-custodial. 7 partners live across LATAM, EMEA and APAC.",
        creator: "@defindex_",
        images: ["https://defindex.io/images/glass-02.png"],
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
