'use client'
import React, { ReactNode } from 'react'
import { StoreProvider } from './store-provider'
import MySorobanReactProvider from './soroban-react-provider'
import { ThemeProvider } from './chakra-provider'
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
          <ThemeProvider>
            {children}
          </ThemeProvider>
        </MuiProvider>
      </MySorobanReactProvider>
    </StoreProvider>
  )
}

export default Providers
