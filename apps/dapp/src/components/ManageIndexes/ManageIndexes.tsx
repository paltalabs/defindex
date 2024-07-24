import {
  Button,
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
import { AddIcon, SearchIcon } from "@chakra-ui/icons"
import AllIndexes from "./AllIndexes"
import { useState } from "react"
import { DeployIndex } from "../DeployIndex/DeployIndex"

export const ManageIndexes = () => {
  const [isOpen, setIsOpen] = useState(false)
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
        <GridItem colStart={11} justifyItems={'start'}>
          <Button
            rounded={18}
            aria-label="add-index"
            colorScheme="green"
            onClick={() => setIsOpen(true)}
          >
            Add Index
          </Button>
          {/* <IconButton aria-label="add-index" colorScheme="green" icon={<AddIcon />} /> */}
        </GridItem>
        <GridItem colSpan={12} colStart={1} colEnd={13}>
          <AllIndexes />
        </GridItem>
      </Grid>
      <Modal isOpen={isOpen} onClose={() => setIsOpen(!isOpen)}>
        <ModalOverlay />
        <ModalContent>
          <DeployIndex />
        </ModalContent>
      </Modal>
    </>
  )
}

export default ManageIndexes