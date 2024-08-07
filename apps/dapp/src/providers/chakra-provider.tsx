import { ChakraProvider } from '@chakra-ui/react'

export const ThemeProvider = ({ children }: { children: React.ReactNode }) => {
  return <ChakraProvider>{children}</ChakraProvider>
}