"use client"

import { ChakraProvider, defaultSystem, createSystem, defaultConfig } from "@chakra-ui/react"
import { ColorModeProvider } from "./color-mode"

export const system = createSystem(defaultConfig, {
  theme: {
    tokens: {
      fonts: {
        heading: { value: `'Figtree', sans-serif` },
        body: { value: `'Figtree', sans-serif` },
      },
    },
  },
})

export function Provider(props: React.PropsWithChildren) {
  return (
    <ChakraProvider value={system}>
      <ColorModeProvider>{props.children}</ColorModeProvider>
    </ChakraProvider>
  )
}
