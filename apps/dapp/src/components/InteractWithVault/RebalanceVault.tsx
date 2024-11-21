import React, { useEffect, useState } from 'react';
import { useAppSelector } from '@/store/lib/storeHooks';
import { useSorobanReact } from '@soroban-react/core';
import { DialogBody, DialogContent, DialogHeader } from '../ui/dialog';
import { For, Grid, GridItem, HStack, Input, NativeSelectField, Stack, Text } from '@chakra-ui/react';
import { Strategy } from '@/store/lib/types';
import { NativeSelectRoot } from '../ui/native-select';
import { InputGroup } from '../ui/input-group';
import { NumberInputField, NumberInputRoot } from '../ui/number-input';
import { useVault } from '@/hooks/useVault';

interface strategiesWithBalances extends Strategy {
  balance: number;
}

const RebalanceVault: React.FC = (() => {
  const { address } = useSorobanReact()
  const { selectedVault } = useAppSelector(state => state.wallet.vaults);
  const [strategiesWithBalances, setStrategiesWithBalances] = useState<strategiesWithBalances[]>([]);
  const { getUserBalance } = useVault();

  useEffect(() => {
    const tempStrategies: strategiesWithBalances[] = [];
    selectedVault?.assets[0]?.strategies.forEach(async (strategy) => {
      const balance = await getUserBalance(selectedVault.address, strategy.address);
      console.log(balance)
      tempStrategies.push({ ...strategy, balance: balance ?? 0 });
    });
    setStrategiesWithBalances(tempStrategies);
  }, [selectedVault]);

  enum Action {
    INVEST = 'invest',
    WITHDRAW = 'withdraw',
  }
  return (
    <DialogContent>
      <DialogHeader>
        Rebalance
      </DialogHeader>
      <DialogBody>
        <Text>
          Strategies:
        </Text>
        <For each={strategiesWithBalances}>
          {(strategy, index) => (
            <Stack key={index}>
              <HStack>
                <Text>{strategy.name}</Text>
                <Text>Balance: ${strategy.balance}</Text>
              </HStack>
              <Grid templateColumns="repeat(12, 1fr)" gap={4}>
                <GridItem colSpan={4}>
                  <NativeSelectRoot>
                    <NativeSelectField>
                      {Object.values(Action).map((action) => (
                        <option key={action} value={action}>
                          {action}
                        </option>
                      ))}
                    </NativeSelectField>
                  </NativeSelectRoot>
                </GridItem>
                <GridItem colSpan={6}>
                  <InputGroup
                    endElement={
                      <Text>
                        {selectedVault?.assets.find(asset => asset.strategies.includes(strategy))?.symbol}
                      </Text>
                    }>
                    <NumberInputRoot>
                      <NumberInputField></NumberInputField>
                    </NumberInputRoot>
                  </InputGroup>
                </GridItem>
              </Grid>
            </Stack>
          )}
        </For>
        <HStack justifyContent={'space-between'} alignContent={'center'}>
        </HStack>
      </DialogBody>


    </DialogContent>
  );
});

export default RebalanceVault;