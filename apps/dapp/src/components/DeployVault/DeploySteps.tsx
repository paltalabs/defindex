import { Stack, StepsItem, StepsList, StepsRoot, Text } from "@chakra-ui/react"

const steps = [
  { title: 'Review Vault', description: 'Review vault' },
  { title: 'Sign transaction', description: 'Sign transaction' },
  { title: 'Wait for confirmation', description: 'Result' },
]

export function DeploySteps({ activeStep, hasError }: { activeStep: number, hasError: boolean }) {

  const activeStepText = steps[activeStep]?.description

  return (
    <Stack>
      <Text>
        <b>{activeStepText}</b>
      </Text>
      <StepsRoot colorScheme={!hasError ? 'green' : 'red'} size='sm' gap='4'>
        <StepsList>
          <StepsItem index={0}>

          </StepsItem>
        </StepsList>
        {/*         {steps.map((step, index) => (
            <StepIndicator>
              <StepStatus complete={!hasError ? <StepIcon /> : <WarningIcon />} />
            </StepIndicator>
            <StepSeparator />
        ))} */}
      </StepsRoot>
    </Stack>
  )
}
