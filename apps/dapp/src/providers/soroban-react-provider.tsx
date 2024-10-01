"use client";
import React from 'react'
import { SorobanReactProvider } from '@soroban-react/core';
import { futurenet, sandbox, standalone, testnet } from '@soroban-react/chains';
import { freighter } from '@soroban-react/freighter';
import type { ChainMetadata, Connector } from "@soroban-react/types";
import { lobstr } from '@soroban-react/lobstr'

const chains: ChainMetadata[] = [sandbox, standalone, futurenet, testnet];
export const connectors: Connector[] = [freighter(), lobstr()]


export default function MySorobanReactProvider({ children }: { children: React.ReactNode }) {

  return (
    <SorobanReactProvider
      chains={chains}
      appName={"Defindex"}
      activeChain={testnet}
      connectors={connectors}>
      {children}
    </SorobanReactProvider>
  )
}