import { useAppSelector } from "@/store/lib/storeHooks"
import { DialogBody, DialogContent, DialogFooter, DialogHeader } from "../ui/dialog"
import { shortenAddress } from "@/helpers/shortenAddress"
import { Button, Grid, GridItem, Icon } from "@chakra-ui/react"
import { FaRegEdit, FaWindowClose } from "react-icons/fa"
import { IoClose } from "react-icons/io5"

export const InspectVault = () => {
  const selectedVault = useAppSelector(state => state.wallet.vaults.selectedVault)
  if (!selectedVault?.address) return null
  return (
    <DialogContent>
      <DialogHeader>
        <Grid templateColumns="repeat(24, 1fr)" justifyItems={'space-between'}>
          <GridItem colSpan={22} >
            <h2>Inspect {selectedVault?.name ? selectedVault.name : shortenAddress(selectedVault?.address!)}</h2>
          </GridItem>
          <GridItem colSpan={1}>
            <Icon onClick={() => { console.log('edit') }} css={{ cursor: "pointer" }}>
              <FaRegEdit />
            </Icon>
          </GridItem>
          <GridItem colSpan={1}>
            <Icon onClick={() => { console.log('close') }} css={{ cursor: "pointer" }}>
              <IoClose />
            </Icon>
          </GridItem>
        </Grid>
      </DialogHeader>
      <DialogBody>
        <Grid templateColumns="repeat(12, 1fr)" gap={4} justifyItems={'center'}>
          <GridItem colSpan={12} justifyItems={'center'}>
            <h3>Address</h3>
            <p>{selectedVault.address}</p>
          </GridItem>
          <GridItem colSpan={3}>
            <ul>Strategies:</ul>
            {selectedVault.strategies.map((strategy, index) => (
              <li key={index}>{strategy.name}</li>
            ))}
          </GridItem>
          <GridItem colSpan={3}>
            <ul>TVL:</ul>
            <li>
              {selectedVault.totalValues.toLocaleString('en-US', { style: 'currency', currency: 'USD' })}
            </li>
          </GridItem>
          <GridItem colSpan={3}>
            <ul>APY:</ul>
            <li>
              24.00%
            </li>
          </GridItem>
          <GridItem colSpan={3}>
            <ul>Investors:</ul>
            <li>
              42
            </li>
          </GridItem>
        </Grid>
      </DialogBody>
      <DialogFooter>
        <Grid templateColumns={'repeat(9, 1fr)'} gap={4} justifyItems={'center'} w={'full'}>
          <GridItem colSpan={3}>
            <Button onClick={() => { console.log('deposit') }}>Deposit</Button>
          </GridItem>
          <GridItem colSpan={3}>
            <Button onClick={() => { console.log('emergency withdraw') }}>Emergency Withdraw</Button>
          </GridItem>
          <GridItem colSpan={3} >
            <Button onClick={() => { console.log('withdraw') }}>Withdraw</Button>
          </GridItem>
        </Grid>
      </DialogFooter>
    </DialogContent>
  )
}