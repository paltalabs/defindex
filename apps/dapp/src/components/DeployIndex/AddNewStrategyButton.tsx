import React from 'react'
import { useEffect, useState } from 'react'
import {
  Button,
  Modal,
  ModalOverlay,
  ModalContent,
  ModalHeader,
  ModalFooter,
  ModalBody,
  ModalCloseButton,
  useDisclosure,
  IconButton,
  FormControl,
  Select,
  Input
} from '@chakra-ui/react'
import { AddIcon } from '@chakra-ui/icons'
import { useAppDispatch, useAppSelector } from '@/store/lib/storeHooks'
import { pushStrategy, getDefaultStrategies, Strategy } from '@/store/lib/features/strategiesStore'
import { useSorobanReact } from '@soroban-react/core'

interface DefaultStrategy {
  name: string;
  address: string;
  value: number;
}


function AddNewStrategyButton() {
  const strategies = useAppSelector(state => state.strategies.strategies)
  const dispatch = useAppDispatch();
  const { activeChain } = useSorobanReact()
  const [defaultStrategies, setDefaultStrategies] = useState<Strategy[]>([])
  const [newStrategy, setNewStrategy] = useState<Strategy>()
  const [newAddress, setNewAddress] = useState<string>()
  const [newName, setNewName] = useState<string>()
  const [selectValue, setSelectValue] = useState<string>('')


  useEffect(() => {
    const fetchStragegies = async () => {
      const tempStrategies = await getDefaultStrategies(activeChain?.name?.toLowerCase() || 'testnet')
      setDefaultStrategies(tempStrategies)
    }
    fetchStragegies()
  }, [activeChain?.networkPassphrase])

  const { isOpen, onOpen, onClose } = useDisclosure()
  const handleOpenModal = () => {
    isOpen ? onClose() : onOpen()
  }

  const resetForm = () => {
    setNewStrategy({ address: '', name: '', value: 0 })
    setNewAddress('')
    setNewName('')
    setSelectValue('')
  }

  const handleInputSelect = async (e: any) => {
    const value = e.target.value
    setSelectValue(value)
    const isDefaultStrategy = await defaultStrategies.find(strategy => strategy.address === value)
    if (!!isDefaultStrategy) {
      setNewStrategy(isDefaultStrategy)
    }
  }

  const addStrategy = async () => {
    const isDefaultStrategy = await defaultStrategies.find(strategy => strategy.address === newStrategy?.address)
    const hasEmptyFields = newStrategy?.address === '' || newStrategy?.name === '' || newName === '' || newAddress === ''
    const strategyExists = strategies.find((strategy: Strategy) => strategy.address === newStrategy?.address)
    if (strategyExists) {
      console.error('Strategy already exists')
      return false
    }
    if (hasEmptyFields && !isDefaultStrategy) {
      console.error('Please fill all fields')
      return false
    }
    await dispatch(pushStrategy(newStrategy!))
    resetForm()
    isOpen ? onClose() : onOpen()
  }
  return (
    <>
      <Button colorScheme="green" size="md" onClick={handleOpenModal} textAlign={'end'} isDisabled={defaultStrategies.length === 0}>
        Add new strategy
      </Button>
      <Modal isOpen={isOpen} onClose={onClose} isCentered>
        <ModalOverlay backdropFilter='blur(5px)' />
        <ModalContent >
          <ModalHeader>Add new strategy</ModalHeader>
          <ModalCloseButton />
          <ModalBody>

            <Select placeholder='Select option' onChange={handleInputSelect} value={selectValue}>
              {defaultStrategies.map((strategy, index) => (
                <option key={strategy.name} value={strategy.address}>{(strategy.name != '') ? strategy.name : strategy.address}</option>
              ))}
            </Select>
          </ModalBody>

          <ModalFooter>
            <Button variant='ghost' mr={3} onClick={onClose}>
              Close
            </Button>
            <IconButton
              aria-label='add_strategy'
              colorScheme='green'
              icon={<AddIcon />}
              onClick={addStrategy}
            />
          </ModalFooter>
        </ModalContent>
      </Modal>
    </>
  )
}

export default AddNewStrategyButton