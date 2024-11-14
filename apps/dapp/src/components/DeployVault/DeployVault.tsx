import { useState } from 'react'
import {
  Card,
  Grid,
  GridItem,
  Input,
  Button,
  For,
  Box,
} from '@chakra-ui/react'
import AddNewStrategyButton from './AddNewStrategyButton'
import { useAppDispatch, useAppSelector } from '@/store/lib/storeHooks'
import { ConfirmDelpoyModal } from './ConfirmDelpoyModal'
import { removeAmountByIndex, removeAsset, setName, setSymbol } from '@/store/lib/features/vaultStore'
import { DialogBody, DialogCloseTrigger, DialogContent, DialogFooter, DialogHeader, DialogRoot } from '../ui/dialog'
import { Asset } from '@/store/lib/types'
import { shortenAddress } from '@/helpers/address'
import { Tooltip } from '../ui/tooltip'
import { FaRegTrashCan } from "react-icons/fa6";

export const DeployVault = () => {
  const dispatch = useAppDispatch()
  //const strategies: Strategy[] = []//useAppSelector(state => state.newVault.strategies)
  const assets: Asset[] = useAppSelector(state => state.newVault.assets)
  const amounts: number[] = useAppSelector(state => state.newVault.amounts)
  const totalValues = useAppSelector(state => state.newVault.totalValues)
  const vaultName = useAppSelector(state => state.newVault.name)
  const vaultSymbol = useAppSelector(state => state.newVault.symbol)
  const [openConfirm, setOpenConfirm] = useState<boolean>(false)

  const handleClose = () => {
    setOpenConfirm(false)
  }

  const setVaultName = async (e: any) => {
    await dispatch(setName(e.target.value))
  }

  const setVaultSymbol = async (e: any) => {
    await dispatch(setSymbol(e.target.value))
  }

  const handleRemoveAsset = (asset: Asset, index: number) => {
    dispatch(removeAmountByIndex(index))
    dispatch(removeAsset(asset.address))
  }

  return (
    <DialogContent>
      <DialogBody>
        <Grid
          templateColumns={['1fr', null, 'repeat(12, 2fr)']}
          alignSelf={'end'}
          alignContent={'center'}
          mb={4}
          gap={6}
        >
          <GridItem colStart={1} colSpan={[12, null, 5]} mb={{ base: 4, md: 0 }}>
            <Input onChange={setVaultName} value={vaultName} w={'full'} placeholder='Defindex name...'></Input>
          </GridItem>
          <GridItem colStart={1} colSpan={[12, null, 4]} mb={{ base: 4, md: 0 }}>
            <Input onChange={setVaultSymbol} value={vaultSymbol} w={'full'} placeholder='Defindex symbol...' maxLength={6} minLength={1}></Input>
          </GridItem>
          <GridItem colStart={[1, null, 12]} colSpan={[12, null, 1]} textAlign={['center', null, 'end']}>
            <AddNewStrategyButton />
          </GridItem>
        </Grid>
        <Grid
          templateColumns={['1fr', null, 'repeat(12, 1fr)']}
          alignSelf={'end'}
          alignContent={'center'}
          mb={4}
          gap={6}>
          <For each={assets}>
            {(asset, j) => (
              <GridItem colSpan={6} key={j}>
                <Card.Root>
                  <Card.Header>
                    <Grid
                      templateColumns={['1fr', null, 'repeat(12, 1fr)']}
                    >
                      <GridItem colSpan={11}>
                        <Card.Title>{asset.strategies[0] ? asset.strategies[0].name : shortenAddress(asset.strategies[0]!.address)}</Card.Title>
                      </GridItem>
                      <GridItem css={{ cursor: 'pointer' }} onClick={() => handleRemoveAsset(asset, j)}>
                        <Box>
                          <FaRegTrashCan />
                        </Box>
                      </GridItem>
                    </Grid>
                  </Card.Header>
                  <Card.Body>
                    <ul>
                      <For each={asset.strategies}>
                        {(strategy, index) => (
                          <Box key={index}>
                            <li>
                              Strategy asset: {asset.symbol}
                            </li>
                            <Tooltip
                              content={strategy.address}
                            >
                              <li>
                                Strategy Address: {shortenAddress(strategy.address)}
                              </li>
                            </Tooltip>
                            {amounts[j] && <li>Initial deposit: ${amounts[j]} {asset.symbol}</li>}
                          </Box>
                        )}
                      </For>
                    </ul>
                  </Card.Body>
                </Card.Root>
              </GridItem>
            )}
          </For>
        </Grid>
        {/*         {assets.length > 0 &&
        <Grid templateColumns={['1fr', null, 'repeat(8, 2fr)']} dir='reverse'>
          <GridItem colStart={[1, null, 8]} textAlign={['center', null, 'end']}>
              <h2>Total: {totalValues}</h2>
          </GridItem>
          </Grid>
        } */}
      </DialogBody>
      <DialogFooter>
        <DialogRoot open={openConfirm} onOpenChange={(e) => setOpenConfirm(e.open)}>
          <Button
            onClick={() => setOpenConfirm(true)}
            disabled={vaultName == '' || vaultSymbol == '' || assets.length == 0}
            colorScheme="green"
            size="lg"
            w={['100%', null, 'auto']}
            >
            Create Vault
          </Button>
          <DialogContent>
            <DialogHeader>
              <DialogCloseTrigger />
            </DialogHeader>
            <ConfirmDelpoyModal isOpen={openConfirm} onClose={handleClose} />
          </DialogContent>
        </DialogRoot>
      </DialogFooter>
    </DialogContent>
  )
}
