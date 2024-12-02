"use client";
import { HStack } from '@chakra-ui/react'
import { useSorobanReact } from '@soroban-react/core'
import ManageVaults from '@/components/ManageVaults/ManageVaults';
import { TestTokens } from '@/components/TestTokens';
import {
  ArcElement,
  Chart as ChartJS,
} from 'chart.js';

ChartJS.register(ArcElement);


export default function Home() {
  const { address } = useSorobanReact()
  return (
    <HStack w={'full'} alignContent={'center'} justifyContent={'center'}>
      <ManageVaults />
    </HStack>
  );
}
