'use client'
import React, { useContext, useEffect, useState } from 'react'
import BackgroundCard from '../ui/BackgroundCard'
import { Button, createListCollection, Flex, Separator, Tabs, Text } from '@chakra-ui/react'
import { FormField } from '../ui/CustomInputFields'
import './VaultInteraction.css'
import { Asset, Vault, VaultContext } from '@/contexts'
import { useVault, useVaultCallback, VaultMethod } from '@/hooks/useVault'
import { Address, nativeToScVal, xdr } from '@stellar/stellar-sdk'
import { useSorobanReact, WalletNetwork } from 'stellar-react'
import { toaster } from '../ui/toaster'
import { parseNumericInput } from '@/helpers/input'

function VaultInteractionButton({ buttonTitle, onClick }: { buttonTitle: string, onClick: () => void }) {
  return (
    <Button
      borderRadius={20}
      px={16}
      className='custom-button'
      variant="outline"
      onClick={onClick}>
      {buttonTitle}
    </Button>
  )
}
function VaultInteraction({ vault }: { vault: Vault }) {
  const [amount, setAmount] = React.useState(0);
  const [tolerance, setTolerance] = React.useState(0);
  const sorobanContext = useSorobanReact();
  const { selectedVault, refreshSelectedVault } = useContext(VaultContext)!;
  const { address } = sorobanContext;
  const [selectedAsset, setSelectedAsset] = useState<string | null>(null);
  const [assetCollection, setAssetCollection] = useState<ReturnType<typeof createListCollection<{ label: string; value: string }>>>()
  const useVaultCB = useVaultCallback();
  const vaultHook = useVault();


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
      ).finally(async () => {
        const newBalance = await vaultHook.getUserBalance(selectedVault.address, address)
        await refreshSelectedVault();
      });
    }
    catch (error: any) {
      console.error('Error:', error)
    } finally {
      setAmount(0)
    }
  }

  const handleDeposit = async () => {
    if (!address) {
      alert('Please connect your wallet');
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

  const handleTolerance = async (tolerance: any) => {
    const input: number = parseNumericInput(tolerance.toString(), 2);
    console.log(input)
    if (input >= 0 && input <= 100) {
      setTolerance(input);
    } else return;
  }

  useEffect(() => {
    if (selectedVault && selectedVault.assetAllocation) {
      console.log('selectedVault', selectedVault)
      const assets = selectedVault.assetAllocation.map((asset) => ({
        label: asset.assetSymbol,
        value: asset.assetSymbol,
      }));
      setAssetCollection(createListCollection({
        items: assets
      }));
    }
  }, [selectedVault])
  if (!assetCollection) return null;
  return (
    <BackgroundCard>
      <Tabs.Root value={'deposit'} w={"100%"} variant="plain">
        <Tabs.List gap={4} className="tabs-list"  >
          <Tabs.Trigger className="button" px={4} justifyItems={'center'} value="deposit">Deposit</Tabs.Trigger>
          <Tabs.Trigger className="button" px={4} justifyItems={'center'} value="withdraw">Withdraw</Tabs.Trigger>
          <Tabs.Indicator className='indicator' />
        </Tabs.List>
        <Separator orientation="vertical" w='full' className='separator' />

        <Tabs.Content value="deposit">
          {selectedVault?.assetAllocation.map((asset) => {
            return (
              <Flex
                direction={{ sm: 'column', md: 'row' }}
                justifyContent={'space-between'}
                alignItems={'end'}
                key={asset.assetSymbol}
                gap={4}
                my={8}
              >
                <FormField
                  label={'Asset'}
                  placeholder='XLM'
                  value={asset.assetSymbol}
                />
                <FormField
                  label="Amount"
                  placeholder='0.00'
                  type="number"
                  onChange={(e) => setAmount(Number(e.target.value))}
                  endElement={
                    <Text pr={2}>{asset.assetSymbol}</Text>
                  } />
              </Flex>
            )
          }
          )}

          <Button
            borderRadius={20}
            px={16}
            className='custom-button'
            variant="outline"
            onClick={handleDeposit}>
            Deposit
          </Button>
        </Tabs.Content>

        <Tabs.Content value="withdraw">

          {selectedVault?.assetAllocation.map((asset) => {
            return (
              <Flex
                direction={{ sm: 'column', md: 'row' }}
                justifyContent={'space-between'}
                alignItems={'end'}
                key={asset.assetSymbol}
                gap={4}
                my={8}
              >
                <FormField
                  label={'Asset'}
                  placeholder='XLM'
                  value={asset.assetSymbol}
                />
                <FormField
                  label="Amount"
                  placeholder='0.00'
                  type="number"
                  onChange={(e) => setAmount(Number(e.target.value))}
                  endElement={
                    <Text pr={2}>USDC</Text>
                  } />
                <FormField
                  label="Tolerance"
                  placeholder='0.00'
                  min={0}
                  value={tolerance}
                  onChange={(e) => handleTolerance(e.target.value)}
                  endElement={
                    <Text pr={2}>%</Text>
                  }
                />
              </Flex>
            )
          })}
          <Button
            className='custom-button'
            borderRadius={20}
            px={16}
            onClick={handleWithdraw}>
            Withdraw
          </Button>
        </Tabs.Content>
      </Tabs.Root>
      <VaultInteractionButton onClick={handleDeposit} buttonTitle='Deposit' />

    </BackgroundCard>
  )
}

export default VaultInteraction
