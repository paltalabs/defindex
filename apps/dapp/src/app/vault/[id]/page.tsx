import { Stack, useConst } from '@chakra-ui/react';
import { navBarHeight } from '@/components/ui/Common';
import VaultDetailsPage from '@/components/VaultDetails/VaultDetailsPage';

export default async function VaultPage({ params }: { params: Promise<{ id: string }> }) {
  const { id: vaultAddress } = await params;

  return (
    <Stack alignContent={'center'} justifyItems={'center'} w={'100dvw'} h={'full'} px={8} justifyContent={'flex-start'} mt={navBarHeight}>
      <VaultDetailsPage vaultAddress={vaultAddress} />
    </Stack>
  );
} 