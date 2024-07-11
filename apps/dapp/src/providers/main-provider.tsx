import React from 'react'
import { StoreProvider } from './store-provider'
import MySorobanReactProvider from './soroban-react-provider'
import { ThemeProvider } from './chakra-provider'

export const Providers = ({ children }: { children: React.ReactNode }) => {
  return (
    <StoreProvider>
      <MySorobanReactProvider>
        <ThemeProvider>
          {children}
        </ThemeProvider>
      </MySorobanReactProvider>
    </StoreProvider>
  )
}

export default Providers
