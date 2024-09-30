import { useState } from 'react'
import {
  Card,
  Button,
  Grid,
  GridItem,
  Container,
  Input,
  Text,
} from '@chakra-ui/react'
import ItemSlider from './Slider'
import AddNewStrategyButton from './AddNewStrategyButton'
import { useAppDispatch, useAppSelector } from '@/store/lib/storeHooks'

import { useSorobanReact } from '@soroban-react/core'
import { ConfirmDelpoyModal } from './ConfirmDelpoyModal'
import { setName, Strategy } from '@/store/lib/features/vaultStore'

export const DeployIndex = () => {
  const dispatch = useAppDispatch()
  const strategies: Strategy[] = useAppSelector(state => state.strategies.strategies)
  const totalValues = useAppSelector(state => state.strategies.totalValues)
  const [openConfirm, setOpenConfirm] = useState<boolean>(false)

  const handleClose = () => {
    setOpenConfirm(false)
  }

  const setVaultName = async (e: any) => {
    await dispatch(setName(e.target.value))
  }

  return (
    <Container centerContent minW={'100%'} px={0}>
      <ConfirmDelpoyModal isOpen={openConfirm} onClose={handleClose} />
      <Card variant="outline" p={16} bgColor="whiteAlpha.50">
        <Grid
          templateColumns={'repeat(12, 2fr)'}
          templateRows={'repeat(5, 1fr)'}
          alignSelf={'end'}
          alignContent={'center'}
          mb={4}
        >
          <GridItem colStart={1} colSpan={3}>
            <Input onChange={setVaultName} placeholder='Defindex name...'></Input>
          </GridItem><GridItem colStart={1} colSpan={3} rowStart={3}>
            <Text mt={4}>Manager</Text>
          </GridItem>
          <GridItem colStart={4} colSpan={3} rowStart={3}>
            <Input onChange={setVaultName} placeholder='GAFS3TLVM...'></Input>
          </GridItem>
          <GridItem colStart={8} colSpan={3} rowStart={3}>
            <Text mt={4}>Emergency Manager</Text>
          </GridItem>
          <GridItem colStart={11} colSpan={3} rowStart={3}>
            <Input onChange={setVaultName} placeholder='GAFS3TLVM...'></Input>
          </GridItem>
          <GridItem colStart={1} colSpan={3} rowStart={5}>
            <Text mt={4}>Fee Receiver</Text>
          </GridItem>
          <GridItem colStart={4} colSpan={3} rowStart={5}>
            <Input onChange={setVaultName} placeholder='GAFS3TLVM...'></Input>
          </GridItem>
          <GridItem colStart={12}>
            <AddNewStrategyButton />
          </GridItem>
        </Grid>
        {strategies.map((strategy, index) => (
          <ItemSlider key={index} name={strategy.name} address={strategy.address} value={strategy.value} />
        ))}
        <Grid templateColumns={'repeat(8, 2fr)'} dir='reverse'>
          <GridItem colStart={8} textAlign={'end'}>
            <h2>Total: {totalValues}%</h2>
          </GridItem>
        </Grid>
        <Button
          isDisabled={totalValues! > 100 || strategies.length == 0 || totalValues == 0}
          isLoading={openConfirm}
          colorScheme="green"
          size="lg"
          mt={4}
          onClick={() => setOpenConfirm(true)}>
          Deploy DeFindex
        </Button>
      </Card>
    </Container>
  )
}