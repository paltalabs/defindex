'use client'
import React, { useContext } from 'react'
import BackgroundCard from '../ui/BackgroundCard'
import { Button, createListCollection, Separator, Tabs } from '@chakra-ui/react'
import { CustomSelect, FormField } from '../ui/CustomInputFields'
import './VaultInteraction.css'
import { Asset, Vault, VaultContext } from '@/contexts'
import { useVault, useVaultCallback, VaultMethod } from '@/hooks/useVault'
import { Address, nativeToScVal, xdr } from '@stellar/stellar-sdk'
import { useSorobanReact } from 'stellar-react'
import { toaster } from '../ui/toaster'



function VaultInteraction({ vault }: { vault: Vault }) {
  const [amount, setAmount] = React.useState(0);
  const [tolerance, setTolerance] = React.useState(0);
  const sorobanContext = useSorobanReact();
  const { selectedVault } = useContext(VaultContext)!;
  const { address } = sorobanContext;
  const [selectedAsset, setSelectedAsset] = React.useState<string | null>(null);
  const useVaultCB = useVaultCallback();
  const vaultHook = useVault();
  const assetCollection = createListCollection({
    items: [
      { label: 'USDC', value: 'usdc' },
      { label: 'XLM', value: 'xlm' },
    ]
  });


  const vaultOperation = async (vaultMethod: VaultMethod, selectedVault: Vault) => {
    if (!address || !vaultMethod || !selectedVault.address) return;
    if (!amount && vaultMethod != VaultMethod.RESCUE) throw new Error('Amount is required');
    const parsedAmount = parseFloat(amount.toString())
    const convertedAmount = parsedAmount * Math.pow(10, 7)
    let params: xdr.ScVal[] = []

    switch (vaultMethod) {
      case VaultMethod.DEPOSIT:
        const depositParams: xdr.ScVal[] = [
          xdr.ScVal.scvVec([nativeToScVal(parseFloat(convertedAmount.toString()), { type: "i128" })]),
          xdr.ScVal.scvVec([nativeToScVal((convertedAmount * 0.9), { type: "i128" })]),
          new Address(address).toScVal(),
          xdr.ScVal.scvBool(false)
        ]
        params = depositParams
        break;
      case VaultMethod.WITHDRAW:
        const currentAsset: Asset = selectedVault.assetAllocation[0]
        console.log(currentAsset)
        const withdrawAmount = ((amount * selectedVault.totalSupply) / Number(currentAsset.total_amount));
        const truncatedWithdrawAmount = Math.floor(withdrawAmount * 1e7) / 1e7;
        console.log(withdrawAmount, truncatedWithdrawAmount)
        const convertedWithdrawAmount = Number(truncatedWithdrawAmount) * Math.pow(10, 7);
        console.log('convertedWithdrawAmount', convertedWithdrawAmount)
        /*   if (false) {
            const tolerance = (0.1 / 100) * convertedWithdrawAmount;
            const minWithdrawAmount = convertedWithdrawAmount - tolerance;
            const minAmountsOut = xdr.ScVal.scvVec([nativeToScVal(Math.ceil(minWithdrawAmount), { type: "i128" })]);
            const withdrawParams: xdr.ScVal[] = [
              nativeToScVal(Math.ceil(convertedWithdrawAmount), { type: "i128" }),
              minAmountsOut,
              new Address(address).toScVal(),
            ];
            params = withdrawParams;
            break;
          } */
        const withdrawParams: xdr.ScVal[] = [
          nativeToScVal(Math.ceil(convertedWithdrawAmount), { type: "i128" }),
          xdr.ScVal.scvVec([nativeToScVal((convertedWithdrawAmount), { type: "i128" })]),
          new Address(address!).toScVal(),
        ];
        params = withdrawParams;
        break;
      default:
        throw new Error('Invalid vault method')
    }
    try {
      const result = await useVaultCB(
        vaultMethod!,
        selectedVault?.address!,
        params,
        true,
      ).then(async (res) => {
        toaster.create({
          type: 'success',
          title: 'Success',
          description: `Transaction successful! ${res.txHash}`,
          duration: 5000,
        })
      }
      ).finally(async () => {
        const newBalance = await vaultHook.getUserBalance(selectedVault.address, address)
        const newIdleFunds = await vaultHook.getIdleFunds(selectedVault.address!)
        const newInvestedFunds = await vaultHook.getInvestedFunds(selectedVault.address)
        const newTVL = await vaultHook.getTotalManagedFunds(selectedVault?.address!)
        const newVaultData: Partial<Vault> = {
          address: selectedVault.address,
        }
      });
    }
    catch (error: any) {
      console.error('Error:', error)
      toaster.create({
        type: 'error',
        title: 'Error',
        description: error.message,
        duration: 5000,
      })
    } finally {
      setAmount(0)
    }
  }


  const handleDeposit = async () => {
    if (!selectedAsset) {
      alert('Please select an asset');
      return;
    }
    if (amount <= 0) {
      alert('Please enter a valid amount');
      return;
    }
    try {
      await vaultOperation(VaultMethod.DEPOSIT, selectedVault!)
    } catch (error) {
      console.error('Error depositing:', error);
      alert('Error depositing');
    }
  }

  const handleWithdraw = async () => {
    if (!selectedAsset) {
      alert('Please select an asset');
      return;
    }
    if (amount <= 0) {
      alert('Please enter a valid amount');
      return;
    }
    try {
      await vaultOperation(VaultMethod.WITHDRAW, selectedVault!)
    } catch (error) {
      console.error(error);
      alert('Error withdrawing');
    }
  }

  return (
    <BackgroundCard>
      <Tabs.Root defaultValue={"deposit"} w={"100%"} variant="plain">
        <Tabs.List gap={4} className="tabs-list"  >
          <Tabs.Trigger className="button" px={4} justifyItems={'center'} value="deposit">Deposit</Tabs.Trigger>
          <Tabs.Trigger className="button" px={4} justifyItems={'center'} value="withdraw">Withdraw</Tabs.Trigger>
          <Tabs.Indicator className='indicator' />
        </Tabs.List>
        <Separator orientation="vertical" w='full' className='separator' />

        <Tabs.Content value="deposit">
          <CustomSelect label="From wallet" placeholder='USDc' collection={assetCollection} multiple={false} onSelect={(value) => setSelectedAsset(value.toString())} />
          <FormField label="Amount" placeholder='0.00' type="number" onChange={(e) => setAmount(Number(e.target.value))} />
          <Button
            variant="outline"
            onClick={handleDeposit}>
            Deposit
          </Button>
        </Tabs.Content>

        <Tabs.Content value="withdraw">
          <CustomSelect label="From wallet" placeholder='USDc' collection={assetCollection} multiple={false} onSelect={(value) => setSelectedAsset(value.toString())} />
          <FormField label="Amount" placeholder='0.00' type="number" onChange={(e) => setAmount(Number(e.target.value))} />
          <FormField label="Tolerance" placeholder='0.00' type="number" onChange={(e) => setTolerance(Number(e.target.value))} />
          <Button
            variant="outline"
            onClick={handleWithdraw}>
            Withdraw
          </Button>
        </Tabs.Content>
      </Tabs.Root>

    </BackgroundCard>
  )
}

export default VaultInteraction
