import { useSorobanReact } from "@soroban-react/core"
import { useContext } from "react"

import { ModalContext } from "@/contexts"
import { VaultMethod } from "@/hooks/useVault"

import { openEditVault, resetAssets } from "@/store/lib/features/vaultStore"
import { setSelectedVault } from "@/store/lib/features/walletStore"
import { useAppDispatch, useAppSelector } from "@/store/lib/storeHooks"
import { VaultData } from "@/store/lib/types"

import {
  Button,
  Grid,
  GridItem,
  IconButton,
  Input,
  Stack,
} from "@chakra-ui/react"
import { CiSearch } from "react-icons/ci"
import { DeployVault } from "../DeployVault/DeployVault"
import { EditVaultModal } from "../InteractWithVault/EditVault"
import { InteractWithVault } from "../InteractWithVault/InteractWithVault"
import { InvestStrategies } from "../InteractWithVault/InvestStrategies"
import RebalanceVault from "../InteractWithVault/RebalanceVault"
import { DialogBackdrop, DialogRoot, DialogTrigger } from "../ui/dialog"
import { InputGroup } from "../ui/input-group"
import ConnectButton from "../Web3/ConnectButton"
import { TransactionStatusModal } from "../Web3/TransactionStatusModal"
import AllVaults from "./AllVaults"
import { InspectVault } from "./InspectVault"

export const ManageVaults = () => {
  const { address, activeChain } = useSorobanReact()
  const {
    inspectVaultModal: inspectModal,
    deployVaultModal: deployModal,
    interactWithVaultModal: interactModal,
    transactionStatusModal: txModal,
    editVaultModal: editModal,
    rebalanceVaultModal: rebalanceModal,
    investStrategiesModal: investModal,
  } = useContext(ModalContext)
  const dispatch = useAppDispatch()
  const modalContext = useContext(ModalContext)
  const vaults: VaultData[] = useAppSelector(state => state.wallet.vaults.createdVaults)
  const handleInspectVault = async (value: boolean, args?: any) => {
    await dispatch(setSelectedVault({ ...args }))
    inspectModal.setIsOpen(value)
  }
  const handleOpenDeployVault = async (method: string, value: boolean, args?: any) => {
    switch (method) {
      case 'create_vault':
        await dispatch(resetAssets())
        deployModal.setIsOpen(value)
        break
      case 'edit_vault':
        await dispatch(resetAssets())
        const selectedVault = vaults.find(vault => vault.address === args.address)
        if (!selectedVault) return;
        await dispatch(openEditVault(selectedVault))
        deployModal.setIsOpen(value)
        break
    }
  }

  const handleOpenInteract = async (method: string, args?: any) => {
    switch (method) {
      case VaultMethod.DEPOSIT:
        interactModal.setIsOpen(true)
        await dispatch(setSelectedVault({ ...args, method: VaultMethod.DEPOSIT }))
        console.log(args)
        break
      case VaultMethod.WITHDRAW:
        interactModal.setIsOpen(true)
        await dispatch(setSelectedVault({ ...args, method: VaultMethod.WITHDRAW }))
        console.log(args)
        break
      case VaultMethod.RESCUE:
        interactModal.setIsOpen(true)
        await dispatch(setSelectedVault({ ...args, method: VaultMethod.RESCUE }))
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
                  <CiSearch />
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

          {!!address && <DialogRoot
            open={deployModal.isOpen}
            onOpenChange={(e) => { handleOpenDeployVault('create_vault', e.open) }}
            size={'lg'}
            placement={'center'}>
            <DialogBackdrop backdropFilter='blur(1px)' />
            <DialogTrigger asChild>
              <Button
                rounded={18}
              >
                Add Vault
              </Button>
            </DialogTrigger>
            <DeployVault />
          </DialogRoot>}
        </GridItem>

        {/* Interact with vault */}
        <GridItem colSpan={12} colStart={1} colEnd={13} zIndex={'base'}>
          <DialogRoot
            open={interactModal.isOpen}
            onOpenChange={(e) => { interactModal.setIsOpen(e.open) }}
            size={'lg'}
            placement={'center'}
          >
            <DialogBackdrop backdropFilter='blur(1px)' />
            <InteractWithVault />
          </DialogRoot>
          <AllVaults handleOpenInspect={handleInspectVault} />
        </GridItem>

        {/* Inspect vault */}
        <DialogRoot
          open={inspectModal.isOpen}
          onOpenChange={(e) => { inspectModal.setIsOpen(e.open) }}
          size={'xl'}
          placement={'center'}
        >
          <DialogBackdrop backdropFilter='blur(1px)' />
          <InspectVault
            handleOpenDeployVault={handleOpenDeployVault}
            handleOpenInteract={handleOpenInteract}
            onClose={() => { inspectModal.setIsOpen(false) }}
          />
        </DialogRoot>

        {/* Edit vault */}
        <DialogRoot
          open={editModal.isOpen}
          onOpenChange={(e) => { editModal.setIsOpen(e.open) }}
          size={'lg'}
          placement={'center'}
        >
          <DialogBackdrop backdropFilter='blur(1px)' />
          <EditVaultModal />
        </DialogRoot>

        {/* Transaction status modal */}
        <DialogRoot
          open={modalContext.transactionStatusModal.isOpen}
          onOpenChange={(e) => { txModal.setIsOpen(e.open) }}
          size={'lg'}
          placement={'center'}
        >
          <DialogBackdrop backdropFilter='blur(1px)' />
          <TransactionStatusModal />
        </DialogRoot>
        <DialogRoot
          open={rebalanceModal.isOpen}
          onOpenChange={(e) => { rebalanceModal.setIsOpen(e.open) }}
          size={'lg'}
          placement={'center'}
        >
          <DialogBackdrop backdropFilter='blur(1px)' />
          <RebalanceVault />
        </DialogRoot>
        <DialogRoot
          open={investModal.isOpen}
          onOpenChange={(e) => { investModal.setIsOpen(e.open) }}
          size={'lg'}
          placement={'center'}
        >
          <DialogBackdrop backdropFilter='blur(1px)' />
          <InvestStrategies />
        </DialogRoot>
      </Grid>
    </>
  )
}

export default ManageVaults