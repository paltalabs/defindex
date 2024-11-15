"use client";
import ManageVaults from '@/components/ManageVaults/ManageVaults';
import { TestTokens } from '@/components/TestTokens';
import { Container } from '@chakra-ui/react';
import { useSorobanReact } from '@soroban-react/core';
import {
  ArcElement,
  Chart as ChartJS,
} from 'chart.js';

ChartJS.register(ArcElement);


export default function Home() {
  const { address } = useSorobanReact()
  return (
    <Container mt={16} mx={0} px={0} minW={'100vw'}>
      <Container centerContent textAlign={'center'} minW={'100vw'}>
        {address && (<TestTokens />)}
        <ManageVaults />
      </Container>
    </Container>
  );
}
