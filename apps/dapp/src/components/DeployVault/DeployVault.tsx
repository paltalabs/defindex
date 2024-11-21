import { useState } from 'react'
import {
  Card,
  Grid,
  GridItem,
  Input,
  Button,
  For,
  Box,
  HStack,
  Text,
  Stack,
} from '@chakra-ui/react'
import AddNewStrategyButton from './AddNewStrategyButton'
import { useAppDispatch, useAppSelector } from '@/store/lib/storeHooks'
import { ConfirmDelpoyModal } from './ConfirmDelpoyModal'
import { removeAsset, removeStrategy, setAssetAmount, setName, setSymbol } from '@/store/lib/features/vaultStore'
import { DialogBody, DialogCloseTrigger, DialogContent, DialogFooter, DialogHeader, DialogRoot } from '../ui/dialog'
import { Asset, Strategy } from '@/store/lib/types'
import { shortenAddress } from '@/helpers/address'
import { Tooltip } from '../ui/tooltip'
import { FaRegTrashCan } from "react-icons/fa6";

export const DeployVault = () => {
  const dispatch = useAppDispatch()
  //const strategies: Strategy[] = []//useAppSelector(state => state.newVault.strategies)
  const assets: Asset[] = useAppSelector(state => state.newVault.assets)
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

  const handleRemoveStrategy = (strategy: Strategy) => {
    const asset = assets.find((a) => a.strategies.includes(strategy))
    dispatch(removeStrategy(strategy))
    console.log(asset)
    if (asset?.strategies.length === 1) {
      dispatch(removeAsset(asset.address))
    }
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
        <For each={assets}>
          {(asset, j) => (
            <Stack key={j}>
              <Text>{asset.symbol}</Text>
              {asset.amount && <Text>Initial deposit: {asset.amount}</Text>}
              <HStack w='full' justifyContent={'space-evenly'}>
                <For each={asset.strategies}>
                  {(strategy, index) => (
                    <Card.Root key={index}>
                      <Card.Header>
                        <Grid
                          templateColumns={['1fr', null, 'repeat(12, 1fr)']}
                        >
                          <GridItem colSpan={11}>
                            <Card.Title>{strategy.name ?? shortenAddress(strategy.address)}</Card.Title>
                          </GridItem>
                          <GridItem css={{ cursor: 'pointer' }} onClick={() => handleRemoveStrategy(strategy)}>
                            <Box>
                              <FaRegTrashCan />
                            </Box>
                          </GridItem>
                        </Grid>
                      </Card.Header>
                      <Card.Body>
                        <ul>
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
                          </Box>

                        </ul>
                      </Card.Body>
                    </Card.Root>
                  )}
                </For>
              </HStack>
            </Stack>
          )}
        </For>
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
