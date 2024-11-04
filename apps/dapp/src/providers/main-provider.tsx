'use client'
import React, { ReactNode } from 'react'
import { StoreProvider } from './store-provider'
import MySorobanReactProvider from './soroban-react-provider'
import { Provider } from '@/components/ui/provider'

export const Providers = ({ children }: { children: ReactNode }) => {
  return (
    <StoreProvider>
      <MySorobanReactProvider>
          <Provider>
            {children}
        </Provider>
      </MySorobanReactProvider>
    </StoreProvider>
  )
}

export default Providers
