import { Address, xdr } from "@stellar/stellar-sdk"
import { Button, For, HStack, NumberInputRoot, Stack, Text } from "@chakra-ui/react"
import { useSorobanReact } from "@soroban-react/core"
import { useContext, useEffect, useState } from "react"
import { ActionType, AssetInvestmentAllocation, RebalanceInstruction } from "@/hooks/types"
import { mapInstructionsToParams } from "@/helpers/vault"
import { ModalContext } from "@/contexts"
import { setStrategyTempAmount, updateVaultData } from "@/store/lib/features/walletStore"
import { useAppDispatch, useAppSelector } from "@/store/lib/storeHooks"
import { useVault, useVaultCallback, VaultMethod } from "@/hooks/useVault"
import { DialogBody, DialogContent, DialogHeader } from "../ui/dialog"
import { Field } from "../ui/field"
import { InputGroup } from "../ui/input-group"
import { NumberInputField } from "../ui/number-input"

interface InvestState extends AssetInvestmentAllocation {
  total: number
}

export const InvestStrategies = () => {
  const { selectedVault } = useAppSelector(state => state.wallet.vaults)
  const { getUserBalance, getIdleFunds, getInvestedFunds } = useVault()
  const vaultCB = useVaultCallback()
  const dispatch = useAppDispatch()
  const {
    transactionStatusModal: txModal,
    investStrategiesModal: investModal,
    inspectVaultModal: inspectModal
  } = useContext(ModalContext)
  const { address } = useSorobanReact()
  const [investment, setInvestment] = useState<InvestState[]>([])
  const [invalidAmount, setInvalidAmount] = useState<boolean>(false)

  const handleInvestInput = (assetIndex: number, strategyIndex: number, amount: number) => {
    const newInvestment = [...investment]
    newInvestment[assetIndex]!.strategy_investments[strategyIndex]!.amount = amount
    newInvestment[assetIndex]!.total = newInvestment[assetIndex]!.strategy_investments.reduce((acc, curr) => acc + curr.amount, 0)
    setInvestment(newInvestment)
  }

  const handleInvest = async () => {
    if (!selectedVault || !address) return
    txModal.initModal()
    investModal.setIsOpen(false)
    const instructions = investment.map((entry) => {
      return entry.strategy_investments.map((strategy_investment) => {
        return {
          action: ActionType.Invest,
          amount: strategy_investment.amount,
          strategy: strategy_investment.strategy
        } as RebalanceInstruction
      })
    })

    const params = instructions.map((instruction) => {
      return mapInstructionsToParams(instruction)
    });

    const rebalanceParams: xdr.ScVal[] = [
      new Address(address).toScVal(),
      ...params
    ]

    try {
      const response = await vaultCB(VaultMethod.REBALANCE, selectedVault.address, rebalanceParams, true)
      await txModal.handleSuccess(response.txHash)
      const newInvestedFunds = await getInvestedFunds(selectedVault.address)
      const newIdleFunds = await getIdleFunds(selectedVault.address)
      await dispatch(updateVaultData({
        address: selectedVault.address,
        idleFunds: newIdleFunds,
        investedFunds: newInvestedFunds
      }))

    } catch (error: any) {
      await txModal.handleError(error.toString())
      console.error('Could not invest: ', error)
    }
  }

  useEffect(() => {
    const totals = investment.map((asset) => {
      const totalAmount = asset.strategy_investments.reduce((acc, curr) => acc + curr.amount, 0)
      const element = {
        asset: asset.asset,
        total: totalAmount
      }
      return element
    })
    totals.forEach((asset) => {
      const assetFunds = selectedVault?.idleFunds.find((fund) => {
        return fund.address === asset.asset
      })?.amount
      if (assetFunds && assetFunds < asset.total) {
        setInvalidAmount(true)
        return
      } else if (assetFunds && assetFunds >= asset.total) {
        setInvalidAmount(false)
      }
    })
  }, [investment])

  useEffect(() => {
    if (!selectedVault) return
    if (selectedVault.assets && investModal.isOpen) {
      selectedVault.assets.forEach((asset) => {
        const investmentAllocation: InvestState = {
          asset: asset.address,
          strategy_investments: [],
          total: 0
        }
        asset.strategies.forEach((strategy) => {
          investmentAllocation.strategy_investments.push({
            amount: 0,
            strategy: strategy.address
          })
        })
        if (investment.length === 0) {
          setInvestment([investmentAllocation])
        } else {
          const assetIndex = investment.findIndex((a) => a.asset === asset.address)
          if (assetIndex === -1) {
            setInvestment([...investment, investmentAllocation])
          }
        }
      })
    } else if (!investModal.isOpen) {
      setInvestment([])
    }
  }, [selectedVault, selectedVault?.assets, investModal.isOpen])

  useEffect(() => {
    if (!selectedVault) return
    if (selectedVault.assets) {
      selectedVault.assets.forEach(async (asset) => {
        asset.strategies.forEach(async (strategy) => {
          const tempAmount = await getUserBalance(selectedVault.address, strategy.address)
          dispatch(setStrategyTempAmount({
            vaultAddress: selectedVault?.address!,
            strategyAddress: strategy.address,
            amount: tempAmount ?? 0
          }))
        })
      })
    }
  }, [selectedVault, selectedVault?.assets])

  if (!selectedVault) return null
  return (
    <DialogContent>
      <DialogHeader>
        <h1>Invest in strategies</h1>
      </DialogHeader>
      <DialogBody>
        <Stack>
          <Text>Idle funds: </Text>
          <For each={selectedVault?.idleFunds}>
            {(fund, i) => (
              <HStack alignItems={'center'} key={i}>
                <Text>$ {fund.amount ?? 0}</Text>
                <Text fontSize={'2xs'}>
                  {selectedVault?.assets.find(asset => asset.address === fund.address)?.symbol}
                </Text>
              </HStack>
            )}
          </For>
          <Text>Strategies:</Text>
          <For each={selectedVault?.assets}>
            {(asset, j) => (
              <Stack mb={6} key={j}>
                <HStack justifyContent={'space-around'} my={6}>
                  <For each={asset.strategies}>
                    {(strategy, k) => (
                      <Stack key={k}>
                        <HStack>
                          <Text>{strategy.name}</Text>
                          <HStack alignItems={'center'}>
                            <Text>Balance:</Text>
                            <Text>$ {strategy.tempAmount}</Text>
                            <Text fontSize={'2xs'}>{asset.symbol}</Text>
                          </HStack>
                        </HStack>
                        <Field invalid={invalidAmount} errorText={`Total of investment exceeds idle funds.`} >
                          <InputGroup endElement={
                            <Text fontSize={'2xs'}>{asset.symbol}</Text>
                          }>
                            <NumberInputRoot
                              inputMode="decimal"
                              onValueChange={(e) => handleInvestInput(j, k, Number(e.value))}
                            >
                              <NumberInputField />
                            </NumberInputRoot>
                          </InputGroup>
                        </Field>
                      </Stack>
                    )}
                  </For>
                </HStack>
                <Text alignSelf={'end'}>Total of investment: ${investment[j]?.total.toLocaleString(undefined, { minimumFractionDigits: 0, maximumFractionDigits: 7 })}</Text>
              </Stack>
            )}
          </For>
          <Button disabled={invalidAmount} onClick={() => handleInvest()}>Invest</Button>
        </Stack>
      </DialogBody>
    </DialogContent>
  )
}