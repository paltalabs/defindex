import { useState } from 'react'
import {
  Card,
  Grid,
  GridItem,
  Container,
  Input,
  Button,
  Box,
  DialogTrigger,
} from '@chakra-ui/react'
import ItemSlider from './Slider'
import AddNewStrategyButton from './AddNewStrategyButton'
import { useAppDispatch, useAppSelector } from '@/store/lib/storeHooks'
import { ConfirmDelpoyModal } from './ConfirmDelpoyModal'
import { setName } from '@/store/lib/features/vaultStore'
import { Strategy } from '@/store/lib/features/walletStore'
import { DialogBody, DialogCloseTrigger, DialogContent, DialogFooter, DialogHeader, DialogRoot, DialogTitle } from '../ui/dialog'

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
    <>
      <DialogContent>
        <DialogBody>
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
          <ItemSlider key={index} name={strategy.name} address={strategy.address} share={strategy.share} />
        ))}
        <Grid templateColumns={['1fr', null, 'repeat(8, 2fr)']} dir='reverse'>
          <GridItem colStart={[1, null, 8]} textAlign={['center', null, 'end']}>
            <h2>Total: {totalValues}%</h2>
          </GridItem>
        </Grid>
        </DialogBody>
        <DialogFooter>
          <DialogRoot open={openConfirm} onOpenChange={(e) => setOpenConfirm(e.open)}>
            <DialogTrigger asChild>
              <Button
                disabled={totalValues! > 100 || strategies.length == 0 || totalValues == 0}
                colorScheme="green"
                size="lg"
                mt={4}
                onClick={() => setOpenConfirm(true)}
                w={['100%', null, 'auto']}
              >
                Deploy Defindex
              </Button>
            </DialogTrigger>
            <DialogContent>
              <ConfirmDelpoyModal isOpen={openConfirm} onClose={handleClose} />
            </DialogContent>
          </DialogRoot>
        </DialogFooter>
      </DialogContent>
    </>
  )
}
