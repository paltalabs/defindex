import type { Metadata } from "next";
import { Inter, Familjen_Grotesk } from "next/font/google";
import "./globals.css";
import { MainProvider } from "@/providers/MainProvider";
import NavBar from "@/components/NavBar/NavBar";
import { Stack } from "@chakra-ui/react";
import { Toaster } from "@/components/ui/toaster";
import './background.css';

const familjen_Grotesk = Familjen_Grotesk({
  variable: "--font-familjen-grotesk",
  subsets: ["latin"],
});

const inter = Inter({
  variable: "--font-inter",
  subsets: ["latin"],
});

const backgroundColor = "#022227";

const customFontClass = `${familjen_Grotesk.variable} ${inter.variable}`; 

export const metadata: Metadata = {
  title: "DeFindex Dapp",
  description: "A GUI for the DeFindex protocol",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" suppressHydrationWarning>
      <body className={customFontClass} style={{ backgroundColor }}>

        <MainProvider>
          <Stack w={"100dvw"} h="100dvh">

            <NavBar />
            <Toaster />
            {children}
          </Stack>
        </MainProvider>
      </body>
    </html>
  );
}
