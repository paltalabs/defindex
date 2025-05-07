'use client'
import { ReactNode } from "react"
import MySorobanReactProvider from "./SorobanProvider"
import { ThemeProvider } from "@/components/ui/provider"

export const MainProvider = ({ children }: { children: ReactNode }) => {
  return (
    <MySorobanReactProvider>
      <ThemeProvider>
        {children}
      </ThemeProvider>
    </MySorobanReactProvider>
  )
}