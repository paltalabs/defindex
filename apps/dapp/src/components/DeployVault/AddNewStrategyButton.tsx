'use client'
import React from 'react'
import { useEffect, useState } from 'react'
import {
  Button,
  IconButton,
  NativeSelectField,
} from '@chakra-ui/react'
import { AddIcon } from '@chakra-ui/icons'
import {  
  DialogBackdrop,
  DialogBody,
  DialogContent,
  DialogFooter,
  DialogRoot,
  DialogTrigger,
} from '@/components/ui/dialog'
import { useAppDispatch, useAppSelector } from '@/store/lib/storeHooks'
import { pushStrategy, getDefaultStrategies } from '@/store/lib/features/vaultStore'
import { useSorobanReact } from '@soroban-react/core'
import { Strategy } from '@/store/lib/features/walletStore'
import { NativeSelectRoot } from '../ui/native-select'

function AddNewStrategyButton() {
  const [open, setOpen] = useState<boolean>(false)
  const strategies = useAppSelector(state => state.newVault.strategies)
  const dispatch = useAppDispatch();
  const { activeChain } = useSorobanReact()
  const [defaultStrategies, setDefaultStrategies] = useState<Strategy[]>([])
  const [newStrategy, setNewStrategy] = useState<Strategy>({ address: '', name: '', share: 0, index: '0' })
  const [newAddress, setNewAddress] = useState<string>()
  const [newName, setNewName] = useState<string>()
  const [selectValue, setSelectValue] = useState<string[]>([''])


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
    setSelectValue([''])
    setOpen(false)
  }

  const handleInputSelect = async (e: any) => {
    const value = e.target.value
    await setSelectValue(value)
    const isDefaultStrategy = await defaultStrategies.find(strategy => strategy.address === value)
    if (!!isDefaultStrategy) {
      setNewStrategy(isDefaultStrategy)
    }
  }

  const addStrategy = async () => {
    const isDefaultStrategy = await defaultStrategies.find(strategy => strategy.address === newStrategy?.address)
    const hasEmptyFields = newStrategy?.address === '' || newStrategy?.name === ''
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
  return (
    <DialogRoot open={open} onOpenChange={(e) => { setOpen(e.open) }} placement={'center'}>
      <DialogBackdrop backdropFilter='blur(1px)' />
      <DialogTrigger asChild>
        <Button
          size="md"
          textAlign={'end'}
          disabled={defaultStrategies.length === 0}>
          Add new strategy
        </Button>
      </DialogTrigger>
      <DialogContent>
        <DialogBody>
          <NativeSelectRoot>
            <NativeSelectField
              placeholder='Select a strategy'
              onChange={(e) => { handleInputSelect(e) }}>
              {defaultStrategies.map((strategy) => {
                return (
                  <option key={strategy.address} value={strategy.address}>{strategy.name}</option>
                )
              })}
            </NativeSelectField>
          </NativeSelectRoot>
        </DialogBody>
        <DialogFooter>
          <Button variant='ghost' mr={3} onClick={() => setOpen(false)}>
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
  )
}

export default AddNewStrategyButton