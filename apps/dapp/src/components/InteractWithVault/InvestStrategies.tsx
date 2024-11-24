import { Button, For, HStack, NumberInput, NumberInputRoot, Stack, Text } from "@chakra-ui/react"
import { DialogBody, DialogContent, DialogHeader } from "../ui/dialog"
import { useAppDispatch, useAppSelector } from "@/store/lib/storeHooks"
import { useVault, useVaultCallback, VaultMethod } from "@/hooks/useVault"
import { setStrategyTempAmount } from "@/store/lib/features/walletStore"
import { useContext, useEffect } from "react"
import { InputGroup } from "../ui/input-group"
import { NumberInputField } from "../ui/number-input"
import { AssetInvestmentAllocation } from "@/hooks/types"
import { Address, nativeToScVal, xdr } from "@stellar/stellar-sdk"
import { ModalContext } from "@/contexts"
import { useSorobanReact } from "@soroban-react/core"


export const InvestStrategies = () => {
  const { selectedVault } = useAppSelector(state => state.wallet.vaults)
  const { getUserBalance } = useVault()
  const vaultCB = useVaultCallback()
  const dispatch = useAppDispatch()
  const { transactionStatusModal: txModal } = useContext(ModalContext)
  const { address } = useSorobanReact()
  const investment: AssetInvestmentAllocation[] = []

  const handleInvestInput = (asset: string, strategy: string, amount: number) => {
    const assetIndex = investment.findIndex((a) => a.asset === asset)
    if (assetIndex === -1) {
      investment.push({
        asset,
        strategy_investments: [{
          amount,
          strategy
        }]
      })
    } else {
      const strategyIndex = investment[assetIndex]!.strategy_investments.findIndex((s) => s.strategy === strategy)
      if (strategyIndex === -1) {
        investment[assetIndex]!.strategy_investments.push({
          amount,
          strategy
        })
      } else {
        investment[assetIndex]!.strategy_investments[strategyIndex]!.amount = amount
      }
    }
  }

  const handleInvest = async () => {
    if (!selectedVault || !address) return
    txModal.initModal()
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
              entry.strategy_investments.map((strategy_investment) =>
                xdr.ScVal.scvMap([
                  new xdr.ScMapEntry({
                    key: xdr.ScVal.scvSymbol("amount"),
                    val: nativeToScVal(BigInt(strategy_investment.amount * 10 ** 7), { type: "i128" }), // Ensure i128 conversion
                  }),
                  new xdr.ScMapEntry({
                    key: xdr.ScVal.scvSymbol("strategy"),
                    val: new Address(strategy_investment.strategy).toScVal() // Convert strategy address
                  }),
                ])
              )
            ),
          }),
        ])
      )
    );
    try {
      const response = await vaultCB(VaultMethod.INVEST, selectedVault.address, [mappedParam], true)
      console.log(response)
      txModal.handleSuccess(response.txHash)
    } catch (error: any) {
      txModal.handleError(error.toString())
      console.error('Could not invest: ', error)
    }
  }

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
                      <InputGroup endElement={
                        <Text fontSize={'2xs'}>{asset.symbol}</Text>
                      }>
                        <NumberInputRoot onValueChange={(e) => handleInvestInput(asset.address, strategy.address, e.valueAsNumber)}>
                          <NumberInputField />
                        </NumberInputRoot>
                      </InputGroup>
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