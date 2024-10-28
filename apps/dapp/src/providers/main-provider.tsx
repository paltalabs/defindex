'use client'
import React, { ReactNode } from 'react'
import { StoreProvider } from './store-provider'
import MySorobanReactProvider from './soroban-react-provider'
import { Provider } from '@/components/ui/provider'
import { createTheme, ThemeProvider as MuiProvider } from '@mui/material/styles';

export const Providers = ({ children }: { children: ReactNode }) => {
  const theme = createTheme(
    {
      palette: {
        primary: { main: '#1976d2' },
      },
    },
  );
  return (
    <StoreProvider>
      <MySorobanReactProvider>
        <MuiProvider theme={theme}>
          <Provider>
            {children}
          </Provider>
        </MuiProvider>
      </MySorobanReactProvider>
    </StoreProvider>
  )
}

export default Providers
