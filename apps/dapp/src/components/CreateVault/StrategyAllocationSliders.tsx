import { VaultContext } from '@/contexts'
import { parsePascalCase } from '@/helpers/utils'
import { Box, Flex, Slider, Stack, Text } from '@chakra-ui/react'
import { useContext } from 'react'

interface StrategyAllocationSlidersProps {
  assetIndex: number;
  assetAmount: number;
  assetSymbol: string;
}

export function StrategyAllocationSliders({ assetIndex, assetAmount, assetSymbol }: StrategyAllocationSlidersProps) {
  const vaultContext = useContext(VaultContext);
  const strategies = vaultContext?.newVault.assetAllocation[assetIndex]?.strategies || [];

  if (strategies.length === 0 || assetAmount <= 0) {
    return null;
  }

  const handlePercentageChange = (strategyIndex: number, newPercentage: number) => {
    if (!vaultContext) return;

    const currentStrategies = [...strategies];
    const oldPercentage = currentStrategies[strategyIndex].amount || 0;
    const diff = newPercentage - oldPercentage;

    // Update the changed strategy
    currentStrategies[strategyIndex] = {
      ...currentStrategies[strategyIndex],
      amount: newPercentage,
    };

    // Distribute the difference among other strategies proportionally
    const otherStrategies = currentStrategies.filter((_, i) => i !== strategyIndex);
    const otherTotal = otherStrategies.reduce((sum, s) => sum + (s.amount || 0), 0);

    if (otherTotal > 0 && diff !== 0) {
      const remaining = -diff;
      otherStrategies.forEach((_, i) => {
        const actualIndex = i >= strategyIndex ? i + 1 : i;
        const currentAmount = currentStrategies[actualIndex].amount || 0;
        const proportion = currentAmount / otherTotal;
        const adjustment = Math.round(remaining * proportion);
        const newAmount = Math.max(0, Math.min(100, currentAmount + adjustment));
        currentStrategies[actualIndex] = {
          ...currentStrategies[actualIndex],
          amount: newAmount,
        };
      });

      // Ensure total is exactly 100
      const total = currentStrategies.reduce((sum, s) => sum + (s.amount || 0), 0);
      if (total !== 100 && currentStrategies.length > 1) {
        const lastOtherIndex = strategyIndex === currentStrategies.length - 1 ? 0 : currentStrategies.length - 1;
        currentStrategies[lastOtherIndex] = {
          ...currentStrategies[lastOtherIndex],
          amount: (currentStrategies[lastOtherIndex].amount || 0) + (100 - total),
        };
      }
    }

    const newAssetAllocation = vaultContext.newVault.assetAllocation.map((item, i) => {
      if (i === assetIndex) {
        return { ...item, strategies: currentStrategies };
      }
      return item;
    });

    vaultContext.setNewVault({
      ...vaultContext.newVault,
      assetAllocation: newAssetAllocation,
    });
  };

  const formatAmount = (percentage: number) => {
    const amount = (assetAmount * percentage) / 100;
    return amount.toFixed(2);
  };

  return (
    <Box w="full" mt={4}>
      <Text fontSize="sm" fontWeight="medium" mb={2} color="gray.400">
        Strategy Allocation
      </Text>
      <Stack gap={3}>
        {strategies.map((strategy, idx) => (
          <Box key={strategy.address} p={3} borderRadius="md" bg="whiteAlpha.50">
            <Flex justify="space-between" align="center" mb={2}>
              <Text fontSize="sm" color="gray.300">
                {parsePascalCase(strategy.name)}
              </Text>
              <Text fontSize="sm" fontWeight="bold" color="green.400">
                {strategy.amount || 0}% = {formatAmount(strategy.amount || 0)} {assetSymbol}
              </Text>
            </Flex>
            <Slider.Root
              value={[strategy.amount || 0]}
              min={0}
              max={100}
              step={1}
              onValueChange={(details) => handlePercentageChange(idx, details.value[0])}
            >
              <Slider.Control>
                <Slider.Track>
                  <Slider.Range />
                </Slider.Track>
                <Slider.Thumb index={0} />
              </Slider.Control>
            </Slider.Root>
          </Box>
        ))}
        <Flex justify="flex-end">
          <Text fontSize="xs" color={strategies.reduce((sum, s) => sum + (s.amount || 0), 0) === 100 ? 'green.400' : 'red.400'}>
            Total: {strategies.reduce((sum, s) => sum + (s.amount || 0), 0)}%
          </Text>
        </Flex>
      </Stack>
    </Box>
  );
}
