import type { Metadata } from "next";
import { Box, Flex, Text, VStack } from "@chakra-ui/react";
import { RiShieldLine } from "react-icons/ri";

export const metadata: Metadata = {
  title: "Region Not Available | DeFindex",
  description: "This service is not available in your region.",
};

export default function BlockedPage() {
  return (
    <Flex
      minH="100dvh"
      w="100dvw"
      alignItems="center"
      justifyContent="center"
      p={4}
    >
      <VStack
        maxW="md"
        w="full"
        gap={6}
        p={{ base: 6, sm: 8 }}
        borderRadius="2xl"
        border="1px solid"
        borderColor="#d3ffb4"
        bg="rgba(26, 44, 34, 0.1)"
        backdropFilter="blur(10px)"
        textAlign="center"
      >
        <VStack gap={4}>
          <Box
            display="flex"
            alignItems="center"
            justifyContent="center"
            w={16}
            h={16}
            borderRadius="full"
            bg="rgba(211, 255, 180, 0.1)"
          >
            <RiShieldLine size={32} color="#d3ffb4" />
          </Box>
        </VStack>

        <VStack gap={2}>
          <Text
            fontSize="2xl"
            fontWeight="bold"
            color="white"
            fontFamily="var(--font-familjen-grotesk)"
          >
            Access Restricted
          </Text>
          <Text
            fontSize="md"
            color="rgba(255, 255, 255, 0.7)"
            fontFamily="var(--font-inter)"
          >
            This service is not available in your region due to regulatory
            restrictions.
          </Text>
        </VStack>
      </VStack>
    </Flex>
  );
}
