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
import { ConfirmDelpoyModal } from './ConfirmDelpoyModal'
import { setName, Strategy } from '@/store/lib/features/vaultStore'

export const DeployVault = () => {
  const dispatch = useAppDispatch()
  const strategies: Strategy[] = useAppSelector(state => state.newVault.strategies)
  const totalValues = useAppSelector(state => state.newVault.totalValues)
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
          templateColumns={['1fr', null, 'repeat(12, 2fr)']}
          alignSelf={'end'}
          alignContent={'center'}
          mb={4}
        >
          <GridItem colStart={1} colSpan={[12, null, 3]} mb={{ base: 4, md: 0 }}>
            <Input onChange={setVaultName} placeholder='Defindex name...'></Input>
          </GridItem>
          <GridItem colStart={[1, null, 12]} colSpan={[12, null, 1]} textAlign={['center', null, 'end']}>
            <AddNewStrategyButton />
          </GridItem>
        </Grid>
        {strategies.map((strategy, index) => (
          <ItemSlider key={index} name={strategy.name} address={strategy.address} value={strategy.value} />
        ))}
        <Grid templateColumns={['1fr', null, 'repeat(8, 2fr)']} dir='reverse'>
          <GridItem colStart={[1, null, 8]} textAlign={['center', null, 'end']}>
            <h2>Total: {totalValues}%</h2>
          </GridItem>
        </Grid>
        <Button
          isDisabled={totalValues! > 100 || strategies.length == 0 || totalValues == 0}
          isLoading={openConfirm}
          colorScheme="green"
          size="lg"
          mt={4}
          onClick={() => setOpenConfirm(true)}
          w={['100%', null, 'auto']}
        >
          Deploy Defindex
        </Button>
      </Card>
    </Container>
  )
}
