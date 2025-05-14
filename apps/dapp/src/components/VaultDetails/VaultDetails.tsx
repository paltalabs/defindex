import { Heading, HStack, Image, Stack, Text } from "@chakra-ui/react";
import './VaultDetails.css';
import BackgroundCard from "../ui/BackgroundCard";
import AddressToClipboard from "../ui/AddressToClipboard";

const vaultAmounts = [
  { title: 'Available', amount: '$100,000' },
  { title: 'Holdings', amount: '$100,000' },
  { title: 'Deposits', amount: '$100,000' },
]

const strategyLogo = 'https://cdn.prod.website-files.com/63ff7d58715f3d565376d770/642360bb2f8ab626c8d491f3_Blend%20Logo%20bigboi.svg'
export function VaultDetailsAmounts({ title, amount }: { title: string, amount: string }) {
  return (
    <Stack gap={1}>
      <Text truncate fontSize={'md'} className='vault-details__amount-title'>{title}</Text>
      <Text fontSize={'xl'} className='vault-details__amount'>{amount}</Text>
    </Stack>
  )
}

export function VaultDetailsBanner({ vaultAddress }: { vaultAddress: string }) {
  return (
    <BackgroundCard>
      <Stack gap={4} w={'80dvw'} h={'full'} alignItems={'start'} justifyContent={'center'}>
        <HStack>
          <Image
            src={strategyLogo}
            boxSize={'5dvh'}
            borderRadius="full"
            fit="cover"
            alt="Strategy Image Placeholder"
            mr={4}
          />
          <Heading textAlign={'left'} fontSize={'4xl'}>Blend Vault</Heading>
        </HStack>
        <HStack justify={'space-between'} w={'100%'}>
          {vaultAmounts.map((vaultAmount, index) => (
            <VaultDetailsAmounts key={index} title={vaultAmount.title} amount={vaultAmount.amount} />
          ))}
        </HStack>
        <AddressToClipboard label={'Vault Contract Address'} vaultAddress={vaultAddress} />
      </Stack>
    </BackgroundCard>
  )
}