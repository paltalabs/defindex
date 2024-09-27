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
import AllIndexes from "./AllIndexes"
import { useState } from "react"
import { DeployIndex } from "../DeployIndex/DeployIndex"
import { useAppDispatch } from "@/store/lib/storeHooks"
import { pushStrategy, resetStrategies } from "@/store/lib/features/strategiesStore"
import { shortenAddress } from "@/helpers/shortenAddress"
import { DepositToIndex } from "../DepositToIndex/DepositToIndex"
import { setSelectedIndex } from "@/store/lib/features/walletStore"
import ConnectButton from "../Wallet/ConnectButton"
import { useSorobanReact } from "@soroban-react/core"

export const ManageIndexes = () => {
  const { address } = useSorobanReact()
  const [modalStatus, setModalStatus] = useState<{
    deployIndex: {
      isOpen: boolean
    },
    deposit: {
      isOpen: boolean
    }
  }>({
    deployIndex: {
      isOpen: false
    },
    deposit: {
      isOpen: false
    }
  })
  const dispatch = useAppDispatch()
  const handleOpenDeployIndex = async (method: string, args?: any) => {
    switch (method) {
      case 'create_defindex':
        await dispatch(resetStrategies())
        setModalStatus({ ...modalStatus, deployIndex: { isOpen: true } })
        break
      case 'edit_index':
        await dispatch(resetStrategies())
        for (const item of args.shares) {
          const newStrategy = {
            address: item.address,
            value: item.share,
            name: item.name ? item.name : shortenAddress(item.address)
          }
          await dispatch(pushStrategy(newStrategy))
        }
        setModalStatus({ ...modalStatus, deployIndex: { isOpen: true } })
        break
    }
  }

  const handleOpenDeposit = async (method: string, args?: any) => {
    switch (method) {
      case 'deposit':
        setModalStatus({ ...modalStatus, deposit: { isOpen: true } })
        await dispatch(setSelectedIndex({ ...args, method: 'deposit' }))
        console.log(args)
        break
      case 'withdraw':
        setModalStatus({ ...modalStatus, deposit: { isOpen: true } })
        await dispatch(setSelectedIndex({ ...args, method: 'withdraw' }))
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
          colEnd={9}>
          <InputGroup>
            <Input
              placeholder='Index address'
              boxShadow='md'
              rounded={18}
            />
            <InputRightElement>
              <IconButton
                rounded={32}
                size={'sm'}
                aria-label="search-index"
                colorScheme="green"
                variant={'ghost'}
                icon={<SearchIcon />} />
            </InputRightElement>
          </InputGroup>
        </GridItem>
        <GridItem colStart={9} colEnd={12} justifyItems={'start'}>
          <Container display={'flex'} flexDirection={'row'}>
            <ConnectButton />
            <Button
              rounded={18}
              sx={{ ml: 4 }}
              aria-label="add-index"
              colorScheme="green"
              onClick={() => handleOpenDeployIndex('create_defindex')}
            >
              Add Index
            </Button>
          </Container>
        </GridItem>
        <GridItem colSpan={12} colStart={1} colEnd={13}>
          <AllIndexes handleOpenDeployIndex={handleOpenDeployIndex} handleOpenDeposit={handleOpenDeposit} />
        </GridItem>
      </Grid>
      <Modal
        isOpen={modalStatus.deployIndex.isOpen}
        onClose={() => setModalStatus({ ...modalStatus, deployIndex: { isOpen: false } })}
      >
        <ModalOverlay />
        <ModalContent minW={{ sm: '100%', md: '80%', lg: '60%', }}>
          <DeployIndex />
        </ModalContent>
      </Modal>
      <Modal
        isOpen={modalStatus.deposit.isOpen}
        onClose={() => setModalStatus({ ...modalStatus, deposit: { isOpen: false } })}
      >
        <ModalOverlay />
        <ModalContent minW={{ sm: '100%', md: '80%', lg: '60%', }}>
          <DepositToIndex />
        </ModalContent>
      </Modal>
    </>
  )
}

export default ManageIndexes