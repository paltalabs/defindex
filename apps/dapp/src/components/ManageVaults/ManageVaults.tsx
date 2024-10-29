import React from "react"
import {
  Box,
  Button,
  Container,
  DialogBackdrop,
  DialogContent,
  DialogRoot,
  DialogTrigger,
  Grid,
  GridItem,
  IconButton,
  Input,
  Stack,
} from "@chakra-ui/react"
import { SearchIcon } from "@chakra-ui/icons"
import AllVaults from "./AllVaults"
import { useState } from "react"
import { DeployVault } from "../DeployVault/DeployVault"
import { useAppDispatch } from "@/store/lib/storeHooks"
import { pushStrategy, resetStrategies } from "@/store/lib/features/vaultStore"
import { shortenAddress } from "@/helpers/shortenAddress"
import { InteractWithVault } from "../InteractWithVault/InteractWithVault"
import { setSelectedVault } from "@/store/lib/features/walletStore"
import ConnectButton from "../Wallet/ConnectButton"
import { useSorobanReact } from "@soroban-react/core"
import { VaultMethod } from "@/hooks/useVault"
import { InputGroup } from "../ui/input-group"

export const ManageVaults = () => {
  const { address } = useSorobanReact()
  const [modalStatus, setModalStatus] = useState<{
    deployVault: {
      isOpen: boolean
    },
    deposit: {
      isOpen: boolean
    }
  }>({
    deployVault: {
      isOpen: false
    },
    deposit: {
      isOpen: false
    }
  })
  const dispatch = useAppDispatch()
  const handleOpenDeployVault = async (method: string, value: boolean, args?: any) => {
    switch (method) {
      case 'create_vault':
        await dispatch(resetStrategies())
        setModalStatus({ ...modalStatus, deployVault: { isOpen: value } })
        break
      case 'edit_vault':
        await dispatch(resetStrategies())
        for (const item of args.strategies) {
          const newStrategy = {
            address: item.address,
            share: item.share,
            name: item.name ? item.name : shortenAddress(item.address),
            index: item.index
          }
          await dispatch(pushStrategy(newStrategy))
        }
        setModalStatus({ ...modalStatus, deployVault: { isOpen: value } })
        break
    }
  }

  const handleOpenDeposit = async (method: string, value: boolean, args?: any) => {
    switch (method) {
      case VaultMethod.DEPOSIT:
        setModalStatus({ ...modalStatus, deposit: { isOpen: value } })
        await dispatch(setSelectedVault({ ...args, method: VaultMethod.DEPOSIT }))
        console.log(args)
        break
      case VaultMethod.WITHDRAW:
        setModalStatus({ ...modalStatus, deposit: { isOpen: value } })
        await dispatch(setSelectedVault({ ...args, method: VaultMethod.WITHDRAW }))
        console.log(args)
        break
      case VaultMethod.EMERGENCY_WITHDRAW:
        setModalStatus({ ...modalStatus, deposit: { isOpen: value } })
        await dispatch(setSelectedVault({ ...args, method: VaultMethod.EMERGENCY_WITHDRAW }))
        console.log(args)
        break
    }
  }


  return (
    <>
      <Grid
        boxShadow='dark-lg'
        rounded={16}
        templateColumns={{ base: 'repeat(1, 1fr)', md: 'repeat(12, 1fr)' }}
        gap={4}
        maxW={{ sm: '100%', md: '90%', lg: '80%' }}
        py={6}
      >
        <GridItem
          colStart={{ base: 1, md: 2 }}
          colEnd={{ base: 13, md: 8 }}>
          <Stack>
            <InputGroup
              endElement={
                <IconButton
                  rounded={32}
                  size={'sm'}
                  aria-label="search-Vault"
                  colorScheme="green"
                  variant={'ghost'}>
                  <SearchIcon />
                </IconButton>}
            >
              <Input
                placeholder='Vault address'
                boxShadow='md'
                rounded={18}
              />
            </InputGroup>
          </Stack>
        </GridItem>
        <GridItem
          colStart={{ base: 1, md: 8 }}
          colEnd={{ base: 13, md: 12 }}
          justifyItems={'start'}
          display={'flex'}
        >
          <ConnectButton />
          {!!address &&
            <DialogRoot open={modalStatus.deployVault.isOpen} onOpenChange={(e) => { handleOpenDeployVault('create_vault', e.open) }} size={'lg'} placement={'center'}>
              <DialogBackdrop backdropFilter='blur(1px)' />
              <DialogTrigger asChild>
                <Container>
                  <Button
                    rounded={18}
                    aria-label="add-Vault"
                    colorScheme="green"
                  >
                    Add Vault
                  </Button>
                </Container>
              </DialogTrigger>
              <DeployVault />
            </DialogRoot>}
        </GridItem>
        <GridItem colSpan={12} colStart={1} colEnd={13}>
          <AllVaults handleOpenDeployVault={handleOpenDeployVault} handleOpenDeposit={handleOpenDeposit} />
        </GridItem>
      </Grid>

      <DialogRoot
        open={modalStatus.deposit.isOpen}
      >
        <DialogBackdrop />
        <DialogContent>
          <InteractWithVault />
        </DialogContent>
      </DialogRoot>
    </>
  )
}

export default ManageVaults