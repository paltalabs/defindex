import React from "react"
import {
  Button,
  Container,
  Grid,
  GridItem,
  IconButton,
  Input,
  InputGroup,
  InputRightElement,
  Modal,
  ModalContent,
  ModalOverlay
} from "@chakra-ui/react"
import { SearchIcon } from "@chakra-ui/icons"
import AllVaults from "./AllVaults"
import { useState } from "react"
import { DeployVault } from "../DeployVault/DeployVault"
import { useAppDispatch } from "@/store/lib/storeHooks"
import { pushStrategy, resetStrategies } from "@/store/lib/features/vaultStore"
import { shortenAddress } from "@/helpers/shortenAddress"
import { DepositToVault } from "../DepositToVault/DepositToVault"
import { setSelectedVault } from "@/store/lib/features/walletStore"
import ConnectButton from "../Wallet/ConnectButton"
import { useSorobanReact } from "@soroban-react/core"

export const ManageVaultes = () => {
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
  const handleOpenDeployVault = async (method: string, args?: any) => {
    switch (method) {
      case 'create_vault':
        await dispatch(resetStrategies())
        setModalStatus({ ...modalStatus, deployVault: { isOpen: true } })
        break
      case 'edit_vault':
        await dispatch(resetStrategies())
        for (const item of args.strategies) {
          const newStrategy = {
            address: item.address,
            value: item.share,
            name: item.name ? item.name : shortenAddress(item.address)
          }
          await dispatch(pushStrategy(newStrategy))
        }
        setModalStatus({ ...modalStatus, deployVault: { isOpen: true } })
        break
    }
  }

  const handleOpenDeposit = async (method: string, args?: any) => {
    switch (method) {
      case 'deposit':
        setModalStatus({ ...modalStatus, deposit: { isOpen: true } })
        await dispatch(setSelectedVault({ ...args, method: 'deposit' }))
        console.log(args)
        break
      case 'withdraw':
        setModalStatus({ ...modalStatus, deposit: { isOpen: true } })
        await dispatch(setSelectedVault({ ...args, method: 'withdraw' }))
        console.log(args)
        break
    }
  }


  return (
    <>
      <Grid
        boxShadow='dark-lg'
        rounded={16}
        templateColumns='repeat(12, 1fr)'
        gap={4}
        maxW={{ sm: '100%', md: '90%', lg: '80%' }}
        py={6}
      >
        <GridItem
          colStart={2}
          colEnd={8}>
          <InputGroup>
            <Input
              placeholder='Vault address'
              boxShadow='md'
              rounded={18}
            />
            <InputRightElement>
              <IconButton
                rounded={32}
                size={'sm'}
                aria-label="search-Vault"
                colorScheme="green"
                variant={'ghost'}
                icon={<SearchIcon />} />
            </InputRightElement>
          </InputGroup>
        </GridItem>
        <GridItem colStart={8} colEnd={12} justifyItems={'start'}>
          <Container display={'flex'} flexDirection={'row'} justifyContent={'end'}>
            <ConnectButton />
            {!!address && <Button
              rounded={18}
              sx={{ px: 6 }}
              aria-label="add-Vault"
              colorScheme="green"
              onClick={() => handleOpenDeployVault('create_vault')}
            >
              Add Vault
            </Button>}
          </Container>
        </GridItem>
        <GridItem colSpan={12} colStart={1} colEnd={13}>
          <AllVaults handleOpenDeployVault={handleOpenDeployVault} handleOpenDeposit={handleOpenDeposit} />
        </GridItem>
      </Grid>
      <Modal
        isOpen={modalStatus.deployVault.isOpen}
        onClose={() => setModalStatus({ ...modalStatus, deployVault: { isOpen: false } })}
      >
        <ModalOverlay />
        <ModalContent minW={{ sm: '100%', md: '80%', lg: '60%', }}>
          <DeployVault />
        </ModalContent>
      </Modal>
      <Modal
        isOpen={modalStatus.deposit.isOpen}
        onClose={() => setModalStatus({ ...modalStatus, deposit: { isOpen: false } })}
      >
        <ModalOverlay />
        <ModalContent minW={{ sm: '100%', md: '80%', lg: '60%', }}>
          <DepositToVault />
        </ModalContent>
      </Modal>
    </>
  )
}

export default ManageVaultes