"use client";
import { Container, Grid } from '@chakra-ui/react'
import { useSorobanReact } from '@soroban-react/core'
import { DeployIndex } from "../src/components/DeployIndex/DeployIndex";
import { DepositToIndex } from "@/components/DepositToIndex/DepositToIndex";
import ConnectButton from "@/components/Wallet/ConnectButton";

export default function Home() {
  const { address } = useSorobanReact()
  return (
    <Container className='' centerContent alignItems={'center'}>
      <ConnectButton />
      {address && (
        <>
          <DeployIndex />
          <DepositToIndex />
        </>
      )}
    </Container>
  );
}
