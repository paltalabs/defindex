import { Button, For, HStack, NumberInput, NumberInputRoot, Stack, Text } from "@chakra-ui/react"
import { DialogBody, DialogContent, DialogHeader } from "../ui/dialog"
import { useAppDispatch, useAppSelector } from "@/store/lib/storeHooks"
import { useVault, useVaultCallback, VaultMethod } from "@/hooks/useVault"
import { setStrategyTempAmount } from "@/store/lib/features/walletStore"
import { useContext, useEffect, useState } from "react"
import { InputGroup } from "../ui/input-group"
import { NumberInputField } from "../ui/number-input"
import { AssetInvestmentAllocation } from "@/hooks/types"
import { Address, Asset, nativeToScVal, xdr } from "@stellar/stellar-sdk"
import { ModalContext } from "@/contexts"
import { useSorobanReact } from "@soroban-react/core"
import { Field } from "../ui/field"


export const InvestStrategies = () => {
  const { selectedVault } = useAppSelector(state => state.wallet.vaults)
  const { getUserBalance } = useVault()
  const vaultCB = useVaultCallback()
  const dispatch = useAppDispatch()
  const {
    transactionStatusModal: txModal,
    investStrategiesModal: investModal
  } = useContext(ModalContext)
  const { address } = useSorobanReact()
  const [investment, setInvestment] = useState<AssetInvestmentAllocation[]>([])

  const handleInvestInput = (assetIndex: number, strategyIndex: number, amount: number) => {
    if (isNaN(amount)) { amount = 0 }
    if (investment[assetIndex] == undefined) return
    if (investment[assetIndex].strategy_investments[strategyIndex] == undefined) return
    const newInvestment = [...investment]
    newInvestment[assetIndex]!.strategy_investments[strategyIndex]!.amount = amount
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
                    val: nativeToScVal(BigInt((parseInt(strategy_investment.amount.toString()) ?? 0) * 10 ** 7), { type: "i128" }), // Ensure i128 conversion
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
      console.log(response)
      await txModal.handleSuccess(response.txHash)
      await investModal.setIsOpen(false)
    } catch (error: any) {
      await txModal.handleError(error.toString())
      console.error('Could not invest: ', error)
    }
  }

  useEffect(() => {
    if (!selectedVault) return
    if (selectedVault.assets && investModal.isOpen) {
      selectedVault.assets.forEach((asset) => {
        const investmentAllocation: AssetInvestmentAllocation = {
          asset: asset.address,
          strategy_investments: []
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
          <HStack justifyContent={'space-around'} my={6}>
            <For each={selectedVault?.assets}>
              {(asset, j) => (
                <For each={asset.strategies} key={j}>
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
                      <Field invalid={investment[j]?.strategy_investments[k]?.amount == 0}>
                        <InputGroup endElement={
                          <Text fontSize={'2xs'}>{asset.symbol}</Text>
                        }>
                          <NumberInputRoot
                            value={investment[j]?.strategy_investments[k]?.amount.toString()}
                            onValueChange={(e) => handleInvestInput(j, k, e.valueAsNumber)}
                          >
                            <NumberInputField />
                          </NumberInputRoot>
                        </InputGroup>
                      </Field>
                    </Stack>
                  )}
                </For>
              )}
            </For>
          </HStack>
          <Button onClick={() => handleInvest()}>Invest</Button>
        </Stack>
      </DialogBody>
    </DialogContent>
  )
}