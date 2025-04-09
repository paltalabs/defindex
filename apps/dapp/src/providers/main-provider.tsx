'use client'
import React, { ReactNode } from 'react'
import { StoreProvider } from './store-provider'
import MySorobanReactProvider from './soroban-react-provider'
import { Provider } from '@/components/ui/provider'
import { ModalProvider } from './modal-provider'
export const Providers = ({ children }: { children: ReactNode }) => {

  return (
      <MySorobanReactProvider>
      <StoreProvider>
        <Provider>
          <ModalProvider>
            {children}
          </ModalProvider>
        </Provider>
      </StoreProvider>
    </MySorobanReactProvider>
  )
}

export default Providers
