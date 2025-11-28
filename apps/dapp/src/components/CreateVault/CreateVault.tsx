import { Stack } from '@chakra-ui/react'
import { AddStrategies } from './AddStrategies'
import './CreateVault.css'
import { CreateVaultButton } from './CreateVaultButton'
import { FeeConfig } from './FeeConfig'
import { ManagerConfig } from './ManagerConfig'
import { VaultConfig } from './VaultConfig'
function CreateVault() {
  return (
    <Stack alignContent={'center'} justifyContent={'center'} gap={6} mt={'10dvh'}>
      <VaultConfig />
      <AddStrategies />
      <ManagerConfig />
      <FeeConfig />
      <CreateVaultButton />
    </Stack>
  )
}

export default CreateVault
