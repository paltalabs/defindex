import React from 'react'
import { useEffect, useState } from 'react'
import {
  Button,
  createListCollection,
  DialogBackdrop,
  DialogBody,
  DialogCloseTrigger,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogRoot,
  IconButton,
  SelectContent,
  SelectItem,
  SelectRoot,
  SelectValueText,
} from '@chakra-ui/react'
import { AddIcon } from '@chakra-ui/icons'
import { useAppDispatch, useAppSelector } from '@/store/lib/storeHooks'
import { pushStrategy, getDefaultStrategies } from '@/store/lib/features/vaultStore'
import { useSorobanReact } from '@soroban-react/core'
import { Strategy } from '@/store/lib/features/walletStore'

function AddNewStrategyButton() {
  const strategies = useAppSelector(state => state.newVault.strategies)
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


  const resetForm = () => {
    setNewStrategy({ address: '', name: '', share: 0, index: '0' })
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
  }

  const tempCollection = createListCollection({
    items: defaultStrategies.map((strategy) => ({
      key: strategy.address,
      value: strategy.name,
    })),
  })
  return (
    <>
      <Button colorScheme="green" size="md" onClick={() => { console.log('open add strategy modal...') }} textAlign={'end'} disabled={defaultStrategies.length === 0}>
        Add new strategy
      </Button>
      <DialogRoot open={true}>
        <DialogBackdrop backdropFilter='blur(5px)' />
        <DialogContent >
          <DialogHeader>Add new strategy</DialogHeader>
          <DialogCloseTrigger />
          <DialogBody>
            <SelectRoot key={''} collection={tempCollection}>

              <SelectContent onChange={handleInputSelect}>
                <SelectValueText>{'Select strategy'}</SelectValueText>
              {defaultStrategies.map((strategy, index) => (
                <SelectItem key={strategy.name} item={strategy.address}>{(strategy.name != '') ? strategy.name : strategy.address}</SelectItem>
              ))}
              </SelectContent>
            </SelectRoot>
          </DialogBody>

          <DialogFooter>
            <Button variant='ghost' mr={3}>
              Close
            </Button>
            <IconButton
              aria-label='add_strategy'
              colorScheme='green'
              onClick={addStrategy}
            >
              <AddIcon />
            </IconButton>
          </DialogFooter>
        </DialogContent>
      </DialogRoot>
    </>
  )
}

export default AddNewStrategyButton