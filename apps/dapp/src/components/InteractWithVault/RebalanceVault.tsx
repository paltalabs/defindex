import React, { useContext, useEffect, useState } from 'react';
import { Address, xdr } from '@stellar/stellar-sdk';
import { Box, Button, For, Grid, GridItem, HStack, IconButton, NativeSelectField, Separator, Stack, Text } from '@chakra-ui/react';
import { DialogBody, DialogContent, DialogHeader } from '../ui/dialog';
import { FaRegTrashCan } from "react-icons/fa6";
import { IoMdAdd } from "react-icons/io";
import { InputGroup } from '../ui/input-group';
import { ModalContext } from '@/contexts';
import { NativeSelectRoot } from '../ui/native-select';
import { NumberInputField, NumberInputRoot } from '../ui/number-input';
import { useAppDispatch, useAppSelector } from '@/store/lib/storeHooks';
import { useSorobanReact } from 'stellar-react';
import { useVault, useVaultCallback, VaultMethod } from '@/hooks/useVault';
import { ActionType, RebalanceInstruction } from '@/hooks/types';
import { mapInstructionsToParams } from '@/helpers/vault';
import { setStrategyTempAmount, updateVaultData } from '@/store/lib/features/walletStore';

interface RebalanceInstructionState {
  action: ActionType | undefined;
  amount: number;
  strategy: string;
  description: string;
}

const RebalanceVault: React.FC = (() => {
  const { address } = useSorobanReact()
  const { selectedVault } = useAppSelector(state => state.wallet.vaults);
  const { getUserBalance, getIdleFunds, getInvestedFunds } = useVault();
  const vaultCB = useVaultCallback();
  const dispatch = useAppDispatch();
  const {
    transactionStatusModal: txModal,
    inspectVaultModal: inspectModal,
    rebalanceVaultModal: rebalanceModal
  } = useContext(ModalContext)
  const [instructions, setInstructions] = useState<RebalanceInstructionState[]>([])
  const [tempInstruction, setTempInstruction] = useState<RebalanceInstructionState>({
    action: undefined,
    amount: 0,
    strategy: '',
    description: ''
  })
  const validActions = [
    "Invest",
    "Unwind"
  ]

  const generateDescription = (action: ActionType, amount: number, strategy: string) => {
    const strategyName = selectedVault?.assets[0]?.strategies.find((s) => s.address === strategy)?.name
    return `${ActionType[action]} ${amount} ${selectedVault?.assets[0]?.symbol} ${action == 1 ? 'to' : 'from'} ${strategyName}`
  }

  const handleRemoveInstruction = (index: number) => {
    const newInstructions = instructions.filter((_, i) => i !== index)
    setInstructions(newInstructions)
  }

  const handleRebalanceVault = async (instructions: RebalanceInstructionState[]) => {
    txModal.initModal()
    if (!selectedVault || !address) return
    const params = mapInstructionsToParams(instructions as RebalanceInstruction[])
    const rebalanceParams: xdr.ScVal[] = [
      new Address(address).toScVal(),
      params
    ]
    try {
      const result = await vaultCB(VaultMethod.REBALANCE, selectedVault?.address!, rebalanceParams, true)
      await txModal.handleSuccess(result.txHash)
      const newInvestedFunds = await getInvestedFunds(selectedVault.address)
      const newIdleFunds = await getIdleFunds(selectedVault.address)
      await dispatch(updateVaultData({
        address: selectedVault.address,
        idleFunds: newIdleFunds,
        investedFunds: newInvestedFunds
      }))
      await setTimeout(() => {
        rebalanceModal.setIsOpen(false)
        inspectModal.setIsOpen(false)
      }, 4500)
    } catch (e: any) {
      const error = e.toString()
      await txModal.handleError(error)
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
  }, [selectedVault]);

  useEffect(() => {
    setTempInstruction({
      ...tempInstruction,
      description: generateDescription(tempInstruction.action!, tempInstruction.amount, tempInstruction.strategy)
    })
  }, [tempInstruction.action, tempInstruction.amount, tempInstruction.strategy])

  useEffect(() => {
    if (!rebalanceModal.isOpen) {
      setInstructions([])
    }
  }, [rebalanceModal.isOpen])

  return (
    <DialogContent>
      <DialogHeader>
        Rebalance
      </DialogHeader>
      <DialogBody>
        <HStack align={'start'}>
          <Stack ml={4}>
            <For each={selectedVault?.assets}>
              {(asset, i) => (
                <Stack key={i}>
                  {asset.symbol} strategies:
                  <For each={asset.strategies}>
                    {(strategy, j) => (
                      <HStack key={j} ml={2}>
                        {strategy.name} ${strategy.tempAmount}
                        <Text fontSize={'2xs'}>{selectedVault?.assets[0]?.symbol}</Text>
                      </HStack>
                    )}
                  </For>
                </Stack>
              )}
            </For>
          </Stack>

          <Stack ml={4}>
            <Text>
              Idle funds:
            </Text>
            {selectedVault?.idleFunds.map((idleFund, i) => (
              <HStack key={i} ml={2}>
                ${idleFund.amount}<Text fontSize={'2xs'}> {selectedVault.assets[i]?.symbol}</Text>
              </HStack>
            ))}
          </Stack>

          <Stack ml={4}>
            <Text>
              Invested funds:
            </Text>
            {selectedVault?.investedFunds.map((investedFund, i) => (
              <HStack key={i} ml={2}>
                ${investedFund.amount}<Text fontSize={'2xs'}> {selectedVault.assets[i]?.symbol}</Text>
              </HStack>
            ))}
          </Stack>
        </HStack>

        <Separator my={6} />

        <Grid templateColumns="repeat(7, 1fr)" gap={4}>
          <GridItem colSpan={2}>
            <NativeSelectRoot>
              <NativeSelectField onChange={(e) => setTempInstruction({ ...tempInstruction, action: ActionType[e.currentTarget.value as keyof typeof ActionType] })}>
                <option value={''}>Select action</option>
                <For each={validActions}>
                  {(action, index) => (
                    <option key={index} value={action}>{action}</option>
                  )}
                </For>
              </NativeSelectField>
            </NativeSelectRoot>
          </GridItem>
          <GridItem colSpan={2}>
            <NativeSelectRoot>
              <NativeSelectField onChange={(e) => setTempInstruction({ ...tempInstruction, strategy: e.currentTarget.value })}>
                <option value={''}>Select Strategy</option>
                <For each={selectedVault?.assets[0]?.strategies}>
                  {(strategy, index) => (
                    <option key={index} value={strategy.address}>{strategy.name}</option>
                  )}
                </For>
              </NativeSelectField>
            </NativeSelectRoot>
          </GridItem>
          <GridItem colSpan={2}>
            <InputGroup
              endElement={
                <Text>
                  {selectedVault?.assets[0]?.symbol}
                </Text>
              }
            >
              <NumberInputRoot
                onValueChange={(e) => setTempInstruction({ ...tempInstruction, amount: Number(e.value) })}
              >
                <NumberInputField />
              </NumberInputRoot>
            </InputGroup>
          </GridItem>
          <GridItem colSpan={1} justifyContent={'end'}>
            <IconButton
              disabled={tempInstruction.action === undefined || tempInstruction.amount === 0 || tempInstruction.strategy === ''}
              onClick={() => {
                setInstructions([...instructions, tempInstruction as RebalanceInstructionState])
              }
              }
            >
              <IoMdAdd />
            </IconButton>
          </GridItem>
        </Grid>
        <Separator my={6} />
        {instructions.length > 0 && <Box style={{ backgroundColor: 'rgba(250,250,255,0.05)', borderRadius: '8px', padding: '16px 16px' }}>
          <For each={instructions}>
            {(instruction, index) => (
              <Stack key={index} my={2}>
                <HStack justifyContent={'space-between'}
                  style={{ backgroundColor: 'rgba(250,250,255,0.05)', borderRadius: '8px', padding: '6px 12px' }}
                >
                  <Text>
                    {instruction.description}
                  </Text>
                  <IconButton
                    size={'xs'}
                    variant={'ghost'}
                    onClick={() => handleRemoveInstruction(index)}
                    colorPalette={'red'}
                  >
                    <FaRegTrashCan />
                  </IconButton>
                </HStack>
              </Stack>
            )}
          </For>
        </Box>}
        <Button
          my={4}
          w={'full'}
          disabled={instructions.length === 0}
          onClick={() => {
            handleRebalanceVault(instructions)
          }}
        >
          Rebalance
        </Button>
      </DialogBody>
    </DialogContent>
  );
});

export default RebalanceVault;