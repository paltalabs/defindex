import { Stack } from '@chakra-ui/react';
import { navBarHeight } from '@/components/ui/Common';
import { VaultDetailsBanner } from '@/components/VaultDetails/VaultDetails';

export default async function VaultPage({ params }: { params: Promise<{ id: string }> }) {
  const { id: vaultAddress } = await params;
  return (
    <Stack alignItems={'center'} justifyContent={'flex-start'} mt={navBarHeight}>
      <VaultDetailsBanner vaultAddress={vaultAddress} />
    </Stack>
  );
} 