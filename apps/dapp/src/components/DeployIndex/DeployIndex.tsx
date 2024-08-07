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
import AddNewAdapterButton from './AddNewAdapterButton'
import { useAppDispatch, useAppSelector } from '@/store/lib/storeHooks'

import { useSorobanReact } from '@soroban-react/core'
import { ConfirmDelpoyModal } from './ConfirmDelpoyModal'
import { setAdapterName } from '@/store/lib/features/adaptersStore'

export const DeployIndex = () => {
  const dispatch = useAppDispatch()  
  const adapters = useAppSelector(state => state.adapters.adapters)
  const totalValues = useAppSelector(state => state.adapters.totalValues)
  const [openConfirm, setOpenConfirm] = useState<boolean>(false)

  const handleClose = () => {
    setOpenConfirm(false)
  }

  const setName = async (e: any) => {
    await dispatch(setAdapterName(e.target.value))
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
            <AddNewAdapterButton />
          </GridItem>
        </Grid>
        {adapters.map((adapter, index) => (
          <ItemSlider key={index} name={adapter.name} address={adapter.address} value={adapter.value} />
        ))}
        <Grid templateColumns={'repeat(8, 2fr)'} dir='reverse'>
          <GridItem colStart={8} textAlign={'end'}>
            <h2>Total: {totalValues}%</h2>
          </GridItem>
        </Grid>
        <Button
          isDisabled={totalValues! > 100 || adapters.length == 0 || totalValues == 0}
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