'use client'
import { ReactNode } from "react"
import MySorobanReactProvider from "./SorobanProvider"
import { ThemeProvider } from "@/components/ui/provider"
import useMounted from "@/hooks/useMounted"
import { StrategiesProvider } from "./StrategiesProvider"

export const MainProvider = ({ children }: { children: ReactNode }) => {
  const mounted = useMounted();
  if (!mounted) return null;
  return (
    <MySorobanReactProvider>
      <ThemeProvider>
        <StrategiesProvider>
          {children}
        </StrategiesProvider>
      </ThemeProvider>
    </MySorobanReactProvider>
  )
}