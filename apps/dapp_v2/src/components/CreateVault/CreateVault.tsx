import React from 'react'
import BackgroundCard from '../ui/BackgroundCard'
import { Fieldset, HStack, createListCollection, Stack, Button, Flex } from '@chakra-ui/react'
import { CustomSelect, FormField } from '../ui/CustomInputFields'
import { baseMargin } from '../ui/Common'

const vaultAssets = createListCollection({
  items: [
    { label: 'USDC', value: 'usdc' },
    { label: 'XLM', value: 'xlm' },
  ]
})

const vaultStrategies = createListCollection({
  items: [
    { label: 'Strategy 1', value: 'strategy1' },
    { label: 'Strategy 2', value: 'strategy2' },
  ]
})
interface ConfigSectionProps {
  title: string;
  children: React.ReactNode;
}

function ConfigSection({ title, children }: ConfigSectionProps) {
  return (
    <BackgroundCard title={title} titleFontWeight='bold' titleFontSize='xl'>
      <Fieldset.Root mt={baseMargin}>
        <Fieldset.Content>
          <HStack gap={4} w={'full'} alignContent={'center'} justifyContent={'center'}>
            {children}
          </HStack>
        </Fieldset.Content>
      </Fieldset.Root>
    </BackgroundCard>
  );
}

function SelectAssets() {
  return (
    <CustomSelect
      collection={vaultAssets}
      label="Assets"
      placeholder="Select assets"
    />
  )
}

function SelectStrategies() {
  return (
    <CustomSelect
      collection={vaultStrategies}
      label="Strategies"
      placeholder="Select strategies"
    />
  )
}

function AddStrategies() {
  return (
    <BackgroundCard title='Add Strategies' titleFontWeight='bold' titleFontSize='xl'>
      <HStack>
        {vaultAssets.items.map((item) => (
          <Stack key={item.value} w={'full'} alignContent={'center'} justifyContent={'center'} mt={baseMargin} gap={4}>
            <FormField label={item.label} placeholder="Initial deposit" type="number" />
            <SelectStrategies />
          </Stack>
        ))}
      </HStack>
    </BackgroundCard>
  )
}

function VaultConfig() {
  return (
    <ConfigSection title="Creating a Vault">
      <FormField label="Vault Name" placeholder="Vault name" />
      <FormField label="Tag for the vault" placeholder="Tag name" />
      <SelectAssets />
    </ConfigSection>
  );
}

function ManagerConfig() {
  return (
    <ConfigSection title="Manager Config">
      <FormField label="Manager" placeholder="Manager address..." />
      <FormField label="Emergency Manager" placeholder="Emergency manager address..." />
      <FormField label="Rebalance manager" placeholder="Rebalance manager address..." />
    </ConfigSection>
  );
}

function FeeConfig() {
  return (
    <ConfigSection title="Fee Config">
      <FormField label="Fee receiver" placeholder="Fee receiver address..." />
      <FormField label="Fee percentage" placeholder="Percentage..." />
    </ConfigSection>
  );
}

function CreateVaultButton() {
  return (
    <Flex w={'full'} h={'full'} alignItems={'center'} justifyContent={'end'}>
      <Button
        px={4}
        rounded={15}
        variant={'outline'}
        size={'lg'}
        mb={baseMargin}
        colorPalette={'green'}>
        Launch Vault
      </Button>
    </Flex>
  )
}

function CreateVault() {
  return (

    <Stack h={'full'} w={'full'} alignContent={'center'} justifyContent={'center'} gap={6}>
      <VaultConfig />
      <AddStrategies />
      <ManagerConfig />
      <FeeConfig />
      <CreateVaultButton />
    </Stack>
  )
}

export default CreateVault
