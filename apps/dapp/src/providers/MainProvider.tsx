'use client'
import { ReactNode } from "react"
import MySorobanReactProvider from "./SorobanProvider"
import { ThemeProvider } from "@/components/ui/provider"
import useMounted from "@/hooks/useMounted"
import { StrategiesProvider } from "./StrategiesProvider"
import { VaultProvider } from "./VaultProvider"


export const MainProvider = ({ children }: { children: ReactNode }) => {
  const mounted = useMounted();
  if (!mounted) return null;
  return (
    <MySorobanReactProvider>
      <ThemeProvider forcedTheme="dark">
        <StrategiesProvider>
          <VaultProvider>
            {children}
          </VaultProvider>
        </StrategiesProvider>
      </ThemeProvider>
    </MySorobanReactProvider>
  )
}