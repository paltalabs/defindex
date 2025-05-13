'use client'
import CreateVault from "@/components/CreateVault/CreateVault";
import { Stack } from "@chakra-ui/react";

export default function Home() {
  return (
    <Stack w={'full'} alignContent={'center'} justifyContent={'center'} px={16}>
      <CreateVault />
    </Stack>
  );
}
