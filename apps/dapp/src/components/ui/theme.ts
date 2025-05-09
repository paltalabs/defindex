import { createSystem, defineConfig, defaultBaseConfig, defaultConfig } from "@chakra-ui/react"


const config = defineConfig({
  theme: {
    tokens: {
      colors: {
        defindex: {
          100: { value: "#000" },
          200: { value: "#ffffff" },
          300: { value: "#0056b3" },
          400: { value: "#0056b3" },
          500: { value: "#0056b3" },
          600: { value: "#0056b3" },
          700: { value: "#0056b3" },
        },
      },
    },
    semanticTokens: {
      colors: {
        defindex: {
          solid: { value: "{colors.defindex.300}" },
          contrast: { value: "{colors.defindex.400}" },
          fg: { value: "{colors.defindex.500}" },
          muted: { value: "{colors.defindex.600}" },
          subtle: { value: "{colors.defindex.700}" },
          emphasized: { value: "{colors.defindex.300}" },
          focusRing: { value: "{colors.defindex.700}" },
        },
      },
    },
  },
})

// Ensure defaultConfig is cast or transformed to match SystemConfig
export const system = createSystem(defaultConfig, config)