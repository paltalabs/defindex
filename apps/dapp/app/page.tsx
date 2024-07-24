"use client";
import { Container, Grid } from '@chakra-ui/react'
import { useSorobanReact } from '@soroban-react/core'
import { DeployIndex } from "../src/components/DeployIndex/DeployIndex";
import { DepositToIndex } from "@/components/DepositToIndex/DepositToIndex";
import ConnectButton from "@/components/Wallet/ConnectButton";
import AllIndexes from '@/components/ManageIndexes/AllIndexes';

export default function Home() {
  const { address } = useSorobanReact()
  return (
    <Container textAlign={'center'} mt={16} mx={0} px={0} minW={'100vw'}>
      <ConnectButton />
      {address && (
        <>
          <AllIndexes />
          <DeployIndex />
          {/*  <DepositToIndex /> */}
        </>
      )}
    </Container>
  );
}
