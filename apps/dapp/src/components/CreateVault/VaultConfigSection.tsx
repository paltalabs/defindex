import { Fieldset, HStack } from '@chakra-ui/react'
import React from 'react'
import BackgroundCard from '../ui/BackgroundCard'
import { baseMargin } from '../ui/Common'

interface VaultConfigSectionProps {
  title: string;
  children: React.ReactNode;
}

export function VaultConfigSection({ title, children }: VaultConfigSectionProps) {
  return (
    <BackgroundCard title={title} titleFontWeight='bold' titleFontSize='xl'>
      <Fieldset.Root mt={baseMargin}>
        <Fieldset.Content>
          <HStack gap={4} w={'full'} alignContent={'center'} justifyContent={'center'} alignItems={'center'} justifyItems={'center'}>
            {children}
          </HStack>
        </Fieldset.Content>
      </Fieldset.Root>
    </BackgroundCard>
  );
}
