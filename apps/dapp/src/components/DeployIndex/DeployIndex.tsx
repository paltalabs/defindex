import { useState } from 'react'
import {
  Card,
  Button,
  Grid,
  GridItem,
} from '@chakra-ui/react'
import ItemSlider from './Slider'
import AddNewAdapterButton from './AddNewAdapterButton'
import { useAppSelector } from '@/store/lib/storeHooks'

import { useSorobanReact } from '@soroban-react/core'
import { ConfirmDelpoyModal } from './ConfirmDelpoyModal'

export const DeployIndex = () => {

  const { activeChain } = useSorobanReact()
  const [openConfirm, setOpenConfirm] = useState<boolean>(false)
  const adapters = useAppSelector(state => state.adapters.adapters)

  const totalValues = useAppSelector(state => state.adapters.totalValues)

  const handleClose = () => {
    setOpenConfirm(false)
  }
  return (
    <>
      <h2>
        Deploy Index on {activeChain?.name} Chain:
      </h2>
      <ConfirmDelpoyModal isOpen={openConfirm} onClose={handleClose} />
      <Card variant="outline" px={16} py={16} width={'75vw'} bgColor="whiteAlpha.100">
        <Grid templateColumns={'repeat(12, 2fr)'} alignSelf={'end'}>
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
    </>
  )
}