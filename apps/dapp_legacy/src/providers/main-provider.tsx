'use client'
import React, { ReactNode } from 'react'
import { StoreProvider } from './store-provider'
import MySorobanReactProvider from './soroban-react-provider'
import { Provider } from '@/components/ui/provider'
import { ModalProvider } from './modal-provider'
export const Providers = ({ children }: { children: ReactNode }) => {

  return (
    <StoreProvider>
      <MySorobanReactProvider>
        <Provider>
          <ModalProvider>
            {children}
          </ModalProvider>
        </Provider>
      </MySorobanReactProvider>
    </StoreProvider>
  )
}

export default Providers
