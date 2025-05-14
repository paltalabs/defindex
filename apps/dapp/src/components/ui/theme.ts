import { createSystem, defineConfig, defaultConfig } from "@chakra-ui/react"

const config = defineConfig({
  cssVarsRoot: ":where(html)",
  cssVarsPrefix: "defindex",
})

// Ensure defaultConfig is cast or transformed to match SystemConfig
export const system = createSystem(defaultConfig, config)