import React, { useEffect, useState } from 'react'
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
  FormLabel,
  FormErrorMessage,
  FormHelperText,
  Select,
  Input
} from '@chakra-ui/react'
import { useAppDispatch, useAppSelector } from '@/store/lib/storeHooks'
import { pushAdapter, getDefaultAdapters, Adapter } from '@/store/lib/features/adaptersStore'
import { AddIcon } from '@chakra-ui/icons'
import { useSorobanReact } from '@soroban-react/core'
import { set } from 'react-hook-form'

interface DefaultAdapter {
  name: string;
  address: string;
  value: number;
}


function AddNewAdapterButton() {
  const adapters = useAppSelector(state => state.adapters.adapters)
  const dispatch = useAppDispatch();
  const { activeChain } = useSorobanReact()
  const [defaultAdapters, setDefaultAdapters] = useState<DefaultAdapter[]>([])
  const [newAdapter, setNewAdapter] = useState<Adapter>()
  const [newAddress, setNewAddress] = useState<string>()
  const [newName, setNewName] = useState<string>()
  const [isInputVisible, setIsInputVisible] = useState<boolean>(false)
  const [selectValue, setSelectValue] = useState<string>('')


  useEffect(() => {
    const tempAdapters = getDefaultAdapters(activeChain?.networkPassphrase || 'Test SDF Network ; September 2015')
    console.log('tempAdapters', tempAdapters)
    setDefaultAdapters(tempAdapters[0].adapters as DefaultAdapter[])
  }, [activeChain])

  const { isOpen, onOpen, onClose } = useDisclosure()
  const handleOpenModal = () => {
    isOpen ? onClose() : onOpen()
  }

  const resetForm = () => {
    setNewAdapter({ address: '', name: '', value: 0 })
    setNewAddress('')
    setNewName('')
    setSelectValue('')
    setIsInputVisible(false)
  }

  const handleInputSelect = async (e: any) => {
    const value = e.target.value
    console.log('value', value)
    setSelectValue(value)
    const isDefaultAdapter = await defaultAdapters.find(adapter => adapter.address === value)
    if (!!isDefaultAdapter) {
      setIsInputVisible(false)
      setNewAdapter(isDefaultAdapter)
    } else if (value === 'custom') {
      setIsInputVisible(true)
      resetForm()
    }
  }

  const handleInput = (e: any) => {
    const id = e.target.id
    const value = e.target.value
    if (id === 'address') {
      setNewAddress(value)
      setNewAdapter({ address: value, name: newName!, value: 0 })
    } else if (id === 'name') {
      setNewName(value)
      setNewAdapter({ address: newAddress!, name: value, value: 0 })
    }
  }

  const addAdapter = async () => {
    const isDefaultAdapter = await defaultAdapters.find(adapter => adapter.address === newAdapter?.address)
    const hasEmptyFields = newAdapter?.address === '' || newAdapter?.name === '' || newName === '' || newAddress === ''
    const adapterExists = adapters.find(adapter => adapter.address === newAdapter?.address)
    if (adapterExists) {
      console.error('Adapter already exists')
      return false
    }
    if (hasEmptyFields && !isDefaultAdapter) {
      console.error('Please fill all fields')
      return false
    }
    await dispatch(pushAdapter(newAdapter!))
    resetForm()
    isOpen ? onClose() : onOpen()
  }
  return (
    <>
      <Button colorScheme="green" size="lg" mt={4} onClick={handleOpenModal}>
        Add adapter +
      </Button>
      <Modal isOpen={isOpen} onClose={onClose} isCentered>
        <ModalOverlay backdropFilter='blur(10px)' />
        <ModalContent >
          <ModalHeader>Add new adapter</ModalHeader>
          <ModalCloseButton />
          <ModalBody>
            {isInputVisible &&
              <FormControl>
                <Input id='address' type='text' placeholder='Address' onChange={handleInput} value={newAddress} />
                <Input id='name' type='text' placeholder='Name' onChange={handleInput} value={newName} />
              </FormControl>
            }
            <Select placeholder='Select option' onChange={handleInputSelect} value={selectValue}>
              {defaultAdapters.map((adapter, index) => (
                <option key={adapter.name} value={adapter.address}>{(adapter.name != '') ? adapter.name : adapter.address}</option>
              ))}
              <option value={'custom'}>Custom</option>
            </Select>
          </ModalBody>

          <ModalFooter>
            <Button variant='ghost' mr={3} onClick={onClose}>
              Close
            </Button>
            <IconButton
              aria-label='add_adapter'
              colorScheme='green'
              icon={<AddIcon />}
              onClick={addAdapter}
            />
          </ModalFooter>
        </ModalContent>
      </Modal>
    </>
  )
}

export default AddNewAdapterButton