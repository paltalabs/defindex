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
  const [defaultStrategies, setDefaultStrategies] = useState<DefaultStrategy[]>([])
  const [newStrategy, setNewStrategy] = useState<Strategy>()
  const [newAddress, setNewAddress] = useState<string>()
  const [newName, setNewName] = useState<string>()
  const [isInputVisible, setIsInputVisible] = useState<boolean>(false)
  const [selectValue, setSelectValue] = useState<string>('')


  useEffect(() => {
    const tempStrategies = getDefaultStrategies(activeChain?.networkPassphrase || 'Test SDF Network ; September 2015')
    if (!tempStrategies || tempStrategies.length === 0) return;
    setDefaultStrategies(tempStrategies[0]?.strategies as DefaultStrategy[])
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
    setIsInputVisible(false)
  }

  const handleInputSelect = async (e: any) => {
    const value = e.target.value
    setSelectValue(value)
    const isDefaultStrategy = await defaultStrategies.find(Strategy => Strategy.address === value)
    if (!!isDefaultStrategy) {
      setIsInputVisible(false)
      setNewStrategy(isDefaultStrategy)
    } else if (value === 'custom') {
      setSelectValue(value)
      setNewStrategy({ address: '', name: '', value: 0 })
      setNewAddress('')
      setNewName('')
      setIsInputVisible(true)
    }
  }

  const handleInput = (e: any) => {
    const id = e.target.id
    const value = e.target.value
    if (id === 'address') {
      setNewAddress(value)
      setNewStrategy({ address: value, name: newName!, value: 0 })
    } else if (id === 'name') {
      setNewName(value)
      setNewStrategy({ address: newAddress!, name: value, value: 0 })
    }
  }

  const addStrategy = async () => {
    const isDefaultStrategy = await defaultStrategies.find(Strategy => Strategy.address === newStrategy?.address)
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
      <Button colorScheme="green" size="md" onClick={handleOpenModal} textAlign={'end'}>
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
              <option value={'custom'}>Custom</option>
            </Select>
            {isInputVisible &&
              <FormControl>
                <Input mt={4} id='address' type='text' placeholder='Address' onChange={handleInput} value={newAddress} />
                <Input mt={4} id='name' type='text' placeholder='Name' onChange={handleInput} value={newName} />
              </FormControl>
            }
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