"use client";
import { Container } from '@chakra-ui/react'
import ManageIndexes from '@/components/ManageIndexes/ManageIndexes';

export default function Home() {
  return (
    <Container mt={16} mx={0} px={0} minW={'100vw'}>
      <Container centerContent textAlign={'center'} minW={'100vw'}>
        <ManageIndexes />
      </Container>
    </Container>
  );
}
