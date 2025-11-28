'use client'
import { ReactNode } from "react"
import { UserProvider } from "@/contexts/UserContext"
import { ThemeProvider } from "@/components/ui/provider"
import useMounted from "@/hooks/useMounted"
import { StrategiesProvider } from "./StrategiesProvider"
import { VaultProvider } from "./VaultProvider"


export const MainProvider = ({ children }: { children: ReactNode }) => {
  const mounted = useMounted();
  if (!mounted) return null;
  return (
    <UserProvider>
      <ThemeProvider forcedTheme="dark">
        <StrategiesProvider>
          <VaultProvider>
            {children}
          </VaultProvider>
        </StrategiesProvider>
      </ThemeProvider>
    </UserProvider>
  )
}