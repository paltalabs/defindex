import { useState } from 'react'
import {
  Card,
  Button,
  Grid,
  GridItem,
  Container,
  Input,
} from '@chakra-ui/react'
import ItemSlider from './Slider'
import AddNewStrategyButton from './AddNewStrategyButton'
import { useAppDispatch, useAppSelector } from '@/store/lib/storeHooks'

import { useSorobanReact } from '@soroban-react/core'
import { ConfirmDelpoyModal } from './ConfirmDelpoyModal'
import { setStrategyName, Strategy } from '@/store/lib/features/strategiesStore'

export const DeployIndex = () => {
  const dispatch = useAppDispatch()  
  const strategies: Strategy[] = useAppSelector(state => state.strategies.strategies)
  const totalValues = useAppSelector(state => state.strategies.totalValues)
  const [openConfirm, setOpenConfirm] = useState<boolean>(false)

  const handleClose = () => {
    setOpenConfirm(false)
  }

  const setName = async (e: any) => {
    await dispatch(setStrategyName(e.target.value))
  }

  return (
    <Container centerContent minW={'100%'} px={0}>
      <ConfirmDelpoyModal isOpen={openConfirm} onClose={handleClose} />
      <Card variant="outline" p={16} bgColor="whiteAlpha.50">
        <Grid
          templateColumns={'repeat(12, 2fr)'}
          alignSelf={'end'}
          alignContent={'center'}
          mb={4}
        >
          <GridItem colStart={1} colSpan={3}>
            <Input onChange={setName} placeholder='Defindex name...'></Input>
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