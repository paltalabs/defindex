import { Stack, Step, StepIcon, StepIndicator, Stepper, StepSeparator, StepStatus, Text, useSteps } from "@chakra-ui/react"
import { WarningIcon } from '@chakra-ui/icons'

const steps = [
  { title: 'Review Index', description: 'Review index' },
  { title: 'Sign transaction', description: 'Sign transaction' },
  { title: 'Wait for confirmation', description: 'Result' },
]

export function DeploySteps({ activeStep, hasError }: { activeStep: number, hasError: boolean }) {
  const { setActiveStep } = useSteps({
    index: 0,
    count: steps.length,
  })

  const activeStepText = steps[activeStep]?.description

  return (
    <Stack>
      <Text>
        <b>{activeStepText}</b>
      </Text>
      <Stepper colorScheme={!hasError ? 'green' : 'red'} size='sm' index={activeStep} gap='4'>
        {steps.map((step, index) => (
          <Step key={index}>
            <StepIndicator>
              <StepStatus complete={!hasError ? <StepIcon /> : <WarningIcon />} />
            </StepIndicator>
            <StepSeparator />
          </Step>
        ))}
      </Stepper>
    </Stack>
  )
}
