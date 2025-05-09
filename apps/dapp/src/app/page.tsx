'use client'
import CreateVault from "@/components/CreateVault/CreateVault";
import styles from "./page.module.css";
import ConnectButton from "@/components/web3/ConnectWallet";
import { Stack } from "@chakra-ui/react";

export default function Home() {
  return (
    <Stack h={'full'} w={'full'} alignContent={'center'} justifyContent={'center'} px={16}>
      <CreateVault />
    </Stack>
  );
}
