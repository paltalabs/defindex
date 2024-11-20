'use client'
import { useSorobanReact } from "@soroban-react/core"

import { shortenAddress } from "@/helpers/address"
import { useVault, VaultMethod } from "@/hooks/useVault"

import { useAppSelector } from "@/store/lib/storeHooks"
import { Asset, AssetAmmount, VaultData } from "@/store/lib/types"

import { Button, Grid, GridItem, HStack, Icon, Stack, Text } from "@chakra-ui/react"
import { DialogBody, DialogContent, DialogFooter, DialogHeader } from "../ui/dialog"
import { FaRegEdit } from "react-icons/fa"
import { IoClose } from "react-icons/io5"
import { ClipboardIconButton, ClipboardRoot } from "../ui/clipboard"
import { ModalContext } from "@/contexts"
import { useContext } from "react"


export const InspectVault = ({
  handleOpenDeployVault,
  handleOpenInteract,
  onClose
}: {
  handleOpenDeployVault: (method: string, value: boolean, args?: any) => any,
  handleOpenInteract: (method: string, args?: any) => any,
  onClose: () => void,
}) => {
  const selectedVault: VaultData | undefined = useAppSelector(state => state.wallet.vaults.selectedVault)
  const { address } = useSorobanReact()
  const { editVaultModal: editModal } = useContext(ModalContext)
  if (!selectedVault) return null
  return (
    <DialogContent>
      <DialogHeader>
        <Grid templateColumns="repeat(24, 1fr)" justifyItems={'space-between'}>
          <GridItem colSpan={22} >
            <h2>Inspect {selectedVault?.name ? selectedVault.name : shortenAddress(selectedVault.address)}</h2>
          </GridItem>
          {address === selectedVault.manager &&
            <GridItem colSpan={1}>
              <Icon onClick={() => { editModal.setIsOpen(true) }} css={{ cursor: "pointer" }}>
                <FaRegEdit />
              </Icon>
            </GridItem>
          }
          <GridItem colSpan={1}>
            <Icon onClick={onClose} css={{ cursor: "pointer" }}>
              <IoClose />
            </Icon>
          </GridItem>
        </Grid>
      </DialogHeader>
      <DialogBody>
        <Grid templateColumns="repeat(12, 1fr)" gap={4}>
          <GridItem colSpan={12} justifyItems={'center'}>
            <h3>Vault address</h3>
            <ClipboardRoot value={selectedVault.address}>
              <HStack alignItems={'center'}>
                <Text>
                  {selectedVault.address}
                </Text>
                <ClipboardIconButton />
              </HStack>
            </ClipboardRoot>
          </GridItem>
        </Grid>
        <Stack justify={'space-around'} direction={{ sm: 'column', md: 'row' }} mt={6}>
          <Stack>
            <Text>Strategies:</Text>
            {selectedVault.assets.map((asset: Asset, index: number) => (
              <HStack key={index} alignContent={'center'}>
                • {asset.strategies[0]?.name}
                <Text fontSize={'2xs'}>{`(${asset.symbol})`}</Text>
              </HStack>
            ))}
          </Stack>
          <Stack>
            <Text>Total value locked:</Text>
            <HStack alignContent={'center'}>
              ${selectedVault.TVL.toLocaleString('en-US', { style: 'decimal', maximumFractionDigits: 4 })}
              <Text fontSize={'2xs'}>{`(${selectedVault.assets[0]!.symbol})`}</Text>
            </HStack>
          </Stack>
          <Stack>
            <Text>Idle funds:</Text>
            {selectedVault.idleFunds.map((asset: AssetAmmount, index: number) => (
              <HStack key={index}>
                <Text>
                  ${asset.amount.toLocaleString('en-US', { style: 'decimal', maximumFractionDigits: 4 })}
                </Text>
                <Text fontSize={'2xs'}>{`(${selectedVault.assets.find((a) => asset.address === a.address)?.symbol})`}</Text>
              </HStack>
            ))}
          </Stack>
          <Stack>
            <Text>Invested funds:</Text>
            {selectedVault.investedFunds.map((asset: AssetAmmount, index: number) => (
              <HStack key={index}>
                <Text>
                  ${asset.amount.toLocaleString('en-US', { style: 'decimal', maximumFractionDigits: 4 })}
                </Text>
                <Text fontSize={'2xs'}>{`(${selectedVault.assets.find((a) => asset.address === a.address)?.symbol})`}</Text>
              </HStack>
            ))}
          </Stack>
          {(address && selectedVault.userBalance) &&
            <Stack>
              <Text>User balance:</Text>
              <HStack>
                ${selectedVault.userBalance.toLocaleString('en-US', { style: 'decimal', maximumFractionDigits: 4 })}
                <Text fontSize={'2xs'}>{`(${selectedVault.assets[0]!.symbol})`}</Text>
              </HStack>
            </Stack>
          }
        </Stack>
      </DialogBody>
      <DialogFooter>
        <HStack justifyContent={'space-around'} w={'full'}>
          {address && <Button onClick={() => { handleOpenInteract(VaultMethod.DEPOSIT, selectedVault) }}>Deposit</Button>}
          {(address === selectedVault.emergencyManager || address === selectedVault.manager) &&
            <Button onClick={() => { handleOpenInteract(VaultMethod.EMERGENCY_WITHDRAW, selectedVault) }}>Emergency Withdraw</Button>
          }
          {address && <Button onClick={() => { handleOpenInteract(VaultMethod.WITHDRAW, selectedVault) }}>Withdraw</Button>}
        </HStack>
      </DialogFooter>
    </DialogContent>
  )
}