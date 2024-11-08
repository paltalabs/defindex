import { useAppSelector } from "@/store/lib/storeHooks"
import { DialogBody, DialogContent, DialogFooter, DialogHeader } from "../ui/dialog"
import { shortenAddress } from "@/helpers/shortenAddress"
import { Button, Grid, GridItem, Icon } from "@chakra-ui/react"
import { FaRegEdit, FaWindowClose } from "react-icons/fa"
import { IoClose } from "react-icons/io5"
import { VaultMethod } from "@/hooks/useVault"
import { Doughnut } from 'react-chartjs-2';

export const InspectVault = ({
  handleOpenDeployVault,
  handleOpenDeposit,
  onClose
}: {
  handleOpenDeployVault: (method: string, value: boolean, args?: any) => any,
  handleOpenDeposit: (method: string, args?: any) => any,
  onClose: () => void,
}) => {
  const selectedVault = useAppSelector(state => state.wallet.vaults.selectedVault)
  if (!selectedVault?.address) return null
  const data = {
    labels: selectedVault.strategies.map((strategy) => strategy.name),
    datasets: [
      {
        label: 'Distribution',
        data: selectedVault.strategies.map((strategy) => {
          return strategy.share
        }),
        borderColor: 'rgba(25, 192, 62, 1)',
        backgroundColor: '[rgba(25, 192, 62, 0.5)]',
        borderWidth: 1,
        hoverOffset: 11
      },
    ],
  }
  const options = {
    responsive: true,
    maintainAspectRatio: true,
    layout: {
      padding: 50,
      height: 50
    },
  }

  return (
    <DialogContent>
      <DialogHeader>
        <Grid templateColumns="repeat(24, 1fr)" justifyItems={'space-between'}>
          <GridItem colSpan={22} >
            <h2>Inspect {selectedVault?.name ? selectedVault.name : shortenAddress(selectedVault?.address!)}</h2>
          </GridItem>
          <GridItem colSpan={1}>
            <Icon onClick={() => { handleOpenDeployVault('edit_vault', true, selectedVault) }} css={{ cursor: "pointer" }}>
              <FaRegEdit />
            </Icon>
          </GridItem>
          <GridItem colSpan={1}>
            <Icon onClick={onClose} css={{ cursor: "pointer" }}>
              <IoClose />
            </Icon>
          </GridItem>
        </Grid>
      </DialogHeader>
      <DialogBody>
        <Grid templateColumns="repeat(12, 1fr)" gap={4} justifyItems={'center'}>
          <GridItem colSpan={6} colStart={4} justifyItems={'center'} alignSelf={'center'}>
            <Doughnut data={data} options={options} />
          </GridItem>
          <GridItem colSpan={12} justifyItems={'center'}>
            <h3>Vault address</h3>
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
            <Button onClick={() => { handleOpenDeposit(VaultMethod.DEPOSIT, selectedVault) }}>Deposit</Button>
          </GridItem>
          <GridItem colSpan={3}>
            <Button onClick={() => { handleOpenDeposit(VaultMethod.EMERGENCY_WITHDRAW, selectedVault) }}>Emergency Withdraw</Button>
          </GridItem>
          <GridItem colSpan={3} >
            <Button onClick={() => { handleOpenDeposit(VaultMethod.WITHDRAW, selectedVault) }}>Withdraw</Button>
          </GridItem>
        </Grid>
      </DialogFooter>
    </DialogContent>
  )
}