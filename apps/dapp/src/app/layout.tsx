import type { Metadata } from "next";
import { Geist, Geist_Mono } from "next/font/google";
import "./globals.css";
import { MainProvider } from "@/providers/MainProvider";
import NavBar from "@/components/NavBar/NavBar";
import { Stack } from "@chakra-ui/react";

const geistSans = Geist({
  variable: "--font-geist-sans",
  subsets: ["latin"],
});

const geistMono = Geist_Mono({
  variable: "--font-geist-mono",
  subsets: ["latin"],
});

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
    <html lang="en">
      <body className={`${geistSans.variable} ${geistMono.variable}`}>
        <MainProvider>
          <Stack w={"full"} h="full">
            <NavBar />
            {children}
          </Stack>
        </MainProvider>
      </body>
    </html>
  );
}
