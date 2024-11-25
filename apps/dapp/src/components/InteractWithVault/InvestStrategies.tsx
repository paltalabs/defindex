import { Box, Button, For, HStack, NumberInput, NumberInputRoot, Stack, Text } from "@chakra-ui/react"
import { DialogBody, DialogContent, DialogHeader } from "../ui/dialog"
import { useAppDispatch, useAppSelector } from "@/store/lib/storeHooks"
import { useVault, useVaultCallback, VaultMethod } from "@/hooks/useVault"
import { setStrategyTempAmount, updateVaultData } from "@/store/lib/features/walletStore"
import { useContext, useEffect, useState } from "react"
import { InputGroup } from "../ui/input-group"
import { NumberInputField } from "../ui/number-input"
import { AssetInvestmentAllocation } from "@/hooks/types"
import { Address, Asset, nativeToScVal, xdr } from "@stellar/stellar-sdk"
import { ModalContext } from "@/contexts"
import { useSorobanReact } from "@soroban-react/core"
import { Field } from "../ui/field"

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
    console.log(amount)
    if (investment[assetIndex] == undefined) {
      console.warn('Asset investment not found')
      return
    }
    if (investment[assetIndex]!.strategy_investments[strategyIndex] !== undefined) {
      console.warn('Strategy investment not found')
      return
    }
    if (!selectedVault?.userBalance) {
      console.warn('User balance not found')
      return
    }
    const newInvestment = [...investment]
    newInvestment[assetIndex]!.strategy_investments[strategyIndex]!.amount = amount
    newInvestment[assetIndex]!.total = newInvestment[assetIndex]!.strategy_investments.reduce((acc, curr) => acc + curr.amount, 0)
    setInvestment(newInvestment)
  }

  const handleInvest = async () => {
    if (!selectedVault || !address) return
    txModal.initModal()
    investModal.setIsOpen(false)
    const mappedParam = xdr.ScVal.scvVec(
      investment.map((entry) =>
        xdr.ScVal.scvMap([
          new xdr.ScMapEntry({
            key: xdr.ScVal.scvSymbol("asset"),
            val: new Address(entry.asset).toScVal()// Convert asset address to ScVal
          }),
          new xdr.ScMapEntry({
            key: xdr.ScVal.scvSymbol("strategy_investments"),
            val: xdr.ScVal.scvVec(
              entry.strategy_investments.map((strategy_investment) => {
                return xdr.ScVal.scvMap([
                  new xdr.ScMapEntry({
                    key: xdr.ScVal.scvSymbol("amount"),
                    val: nativeToScVal(BigInt((strategy_investment.amount ?? 0) * 10 ** 7), { type: "i128" }), // Ensure i128 conversion
                  }),
                  new xdr.ScMapEntry({
                    key: xdr.ScVal.scvSymbol("strategy"),
                    val: new Address(strategy_investment.strategy).toScVal() // Convert strategy address
                  }),
                ])
              })
            ),
          }),
        ])
      )
    );
    try {
      const response = await vaultCB(VaultMethod.INVEST, selectedVault.address, [mappedParam], true)
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
                              defaultValue="0"
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