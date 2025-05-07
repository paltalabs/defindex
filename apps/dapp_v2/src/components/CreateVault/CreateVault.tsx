import React from 'react'
import BackgroundCard from '../ui/BackgroundCard'
import { Field, Fieldset, Select, HStack, Input, createListCollection, Portal, Stack, Box, Button, Flex } from '@chakra-ui/react'


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

const basePadding = 2
const baseMargin = 6

function SelectAssets() {
  return (
    <Select.Root multiple collection={vaultAssets}>
      <Select.HiddenSelect />
      <Select.Label>Assets</Select.Label>
      <Select.Control>
        <Select.Trigger px={basePadding}>
          <Select.ValueText placeholder='Select assets' />
        </Select.Trigger>
        <Select.IndicatorGroup p={basePadding}>
          <Select.Indicator />
        </Select.IndicatorGroup>
      </Select.Control>
      <Portal>
        <Select.Positioner>
          <Select.Content>
            {vaultAssets.items.map((item) => (
              <Select.Item key={item.value} item={item}>
                {item.label}
                <Select.ItemIndicator />
              </Select.Item>
            ))}
          </Select.Content>
        </Select.Positioner>
      </Portal>
    </Select.Root>
  )
}

function SelectStrategies() {
  return (
    <Select.Root multiple collection={vaultStrategies}>
      <Select.HiddenSelect />
      <Select.Label>Strategies</Select.Label>
      <Select.Control>
        <Select.Trigger px={basePadding}>
          <Select.ValueText placeholder='Select strategies' />
        </Select.Trigger>
        <Select.IndicatorGroup p={basePadding}>
          <Select.Indicator />
        </Select.IndicatorGroup>
      </Select.Control>
      <Portal>
        <Select.Positioner>
          <Select.Content>
            {vaultStrategies.items.map((item) => (
              <Select.Item key={item.value} item={item}>
                {item.label}
                <Select.ItemIndicator />
              </Select.Item>
            ))}
          </Select.Content>
        </Select.Positioner>
      </Portal>
    </Select.Root>
  )
}

function AddStrategies() {
  return (
    <BackgroundCard title='Add Strategies' titleFontWeight='bold' titleFontSize='xl'>
      <HStack>
        {vaultAssets.items.map((item) => (
          <Stack key={item.value} w={'full'} alignContent={'center'} justifyContent={'center'} mt={baseMargin} gap={4}>
            <Field.Root>
              <Field.Label>{item.label}</Field.Label>
              <Input placeholder='Initial deposit' px={basePadding} />
            </Field.Root>
            <SelectStrategies />
          </Stack>
        ))}
      </HStack>
    </BackgroundCard>
  )
}

function VaultConfig() {
  return (
    <BackgroundCard title='Creating a Vault' titleFontWeight='bold' titleFontSize='2xl'>
      <Fieldset.Root mt={baseMargin}>
        <Fieldset.Content>
          <HStack gap={4} w={'full'} alignContent={'center'} justifyContent={'center'}>
            <Field.Root>
              <Field.Label>Vault Name</Field.Label>
              <Input placeholder='Vault name' px={basePadding} />
            </Field.Root>
            <Field.Root>
              <Field.Label>Tag for the vault</Field.Label>
              <Input placeholder='Tag name' px={basePadding} />
            </Field.Root>
            <SelectAssets />
          </HStack>
        </Fieldset.Content>
      </Fieldset.Root>
    </BackgroundCard>
  )
}

function ManagerConfig() {
  return (
    <BackgroundCard title='Manager Config' titleFontWeight='bold' titleFontSize='xl'>
      <Fieldset.Root mt={6}>
        <Fieldset.Content>
          <HStack gap={4} w={'full'} alignContent={'center'} justifyContent={'center'}>
            <Field.Root>
              <Field.Label>Manager</Field.Label>
              <Input placeholder='Manager address...' px={basePadding} />
            </Field.Root>
            <Field.Root>
              <Field.Label>Emergency Manager</Field.Label>
              <Input placeholder='Emergency manager address...' px={basePadding} />
            </Field.Root>
            <Field.Root>
              <Field.Label>Rebalance manager</Field.Label>
              <Input placeholder='Rebalance manager address...' px={basePadding} />
            </Field.Root>
          </HStack>
        </Fieldset.Content>
      </Fieldset.Root>
    </BackgroundCard>
  )
}

function FeeConfig() {
  return (
    <BackgroundCard title='Fee Config' titleFontWeight='bold' titleFontSize='xl'>
      <Fieldset.Root mt={6}>
        <Fieldset.Content>
          <HStack gap={4} w={'full'} alignContent={'center'} justifyContent={'center'}>
            <Field.Root>
              <Field.Label>Fee receiver</Field.Label>
              <Input placeholder='Fee receiver address...' px={basePadding} />
            </Field.Root>
            <Field.Root>
              <Field.Label>Fee percentage</Field.Label>
              <Input placeholder='Percentage...' px={basePadding} />
            </Field.Root>
          </HStack>
        </Fieldset.Content>
      </Fieldset.Root>
    </BackgroundCard>
  )
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
