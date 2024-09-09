"use client";
import { Container } from '@chakra-ui/react'
import { useSorobanReact } from '@soroban-react/core'
import { DeployIndex } from "../src/components/DeployIndex/DeployIndex";
import ConnectButton from "@/components/Wallet/ConnectButton";
import ManageIndexes from '@/components/ManageIndexes/ManageIndexes';

export default function Home() {
  const { address } = useSorobanReact()
  return (
    <Container textAlign={'center'} mt={16} mx={0} px={0} minW={'100vw'}>
      <ConnectButton />
      {address && (
        <Container centerContent minW={'100vw'}>
          <ManageIndexes />
          {/*   <DeployIndex /> */}
        </Container>
      )}
    </Container>
  );
}
