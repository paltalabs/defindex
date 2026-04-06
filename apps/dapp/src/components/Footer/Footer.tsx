"use client";

import { Flex, Link, Text } from "@chakra-ui/react";
import NextLink from "next/link";

export default function Footer() {
  return (
    <Flex
      as="footer"
      w="full"
      py={4}
      px={6}
      justifyContent="center"
      alignItems="center"
      borderTop="1px solid rgba(211, 255, 180, 0.2)"
    >
      <Text fontSize="sm" color="rgba(255, 255, 255, 0.5)">
        <Link
          as={NextLink}
          href="/tos"
          color="rgba(255, 255, 255, 0.6)"
          _hover={{ color: "#d3ffb4" }}
          transition="color 0.2s"
        >
          Terms of Service
        </Link>
      </Text>
    </Flex>
  );
}
