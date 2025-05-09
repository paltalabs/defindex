"use client";
import { HStack } from '@chakra-ui/react'
import ManageVaults from '@/components/ManageVaults/ManageVaults';
import {
  ArcElement,
  Chart as ChartJS,
} from 'chart.js';

ChartJS.register(ArcElement);


export default function Home() {
  return (
    <HStack w={'full'} alignContent={'center'} justifyContent={'center'}>
      <ManageVaults />
    </HStack>
  );
}
