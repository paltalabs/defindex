import { useContext } from "react"
import { useSorobanReact } from "@soroban-react/core"
import { Grid, GridItem, Icon, Link as ChakraLink, Text, HStack } from "@chakra-ui/react"
import { DialogBody, DialogContent } from "../ui/dialog"
import { StepsContent, StepsItem, StepsList, StepsRoot } from "../ui/steps"
import { ProgressCircleRing, ProgressCircleRoot } from "../ui/progress-circle"
import { FaRegCheckCircle } from "react-icons/fa";
import { LuExternalLink } from "react-icons/lu";
import { FaRegCircleXmark } from "react-icons/fa6";
import { ModalContext, TransactionStatusModalStatus } from "@/contexts"


export const TransactionStatusModal = () => {
  const { activeChain } = useSorobanReact()
  const { transactionStatusModal: status } = useContext(ModalContext)
  return (
    <DialogContent>
      <DialogBody>
        <StepsRoot variant={'subtle'} step={status.step} count={2}>
          <StepsList>
            <StepsItem index={0} title="Review & sign" />
            <StepsItem index={1} title="Wait for confirmation" />
            <StepsItem index={2} title="Result" />
          </StepsList>
          <StepsContent index={0}>
            <Grid gapY={8} mt={6} templateColumns={{ base: 'repeat(1, 1fr)', md: 'repeat(12, 1fr)' }} >
              <GridItem colSpan={12}>
                <Text textAlign={'center'}>Review the transaction in your wallet, and sign the transaction.</Text>
              </GridItem>
              <GridItem colSpan={12} textAlign={'center'}>
                <ProgressCircleRoot value={null} >
                  <ProgressCircleRing />
                </ProgressCircleRoot>
              </GridItem>
            </Grid>
          </StepsContent>
          <StepsContent index={1}>
            <Grid gapY={8} mt={6} templateColumns={{ base: 'repeat(1, 1fr)', md: 'repeat(12, 1fr)' }} >
              <GridItem colSpan={12}>
                <Text textAlign={'center'}>Waiting for the blockchain confirmation.</Text>
                <Text textAlign={'center'} fontSize={'xs'}>This may take a few seconds.</Text>
              </GridItem>
              <GridItem colSpan={12} textAlign={'center'}>
                <ProgressCircleRoot value={null} >
                  <ProgressCircleRing />
                </ProgressCircleRoot>
              </GridItem>
            </Grid>
          </StepsContent>
          <StepsContent index={2}>
            <Grid gapY={8} mt={6} templateColumns={{ base: 'repeat(1, 1fr)', md: 'repeat(12, 1fr)' }} >
              {status.status === TransactionStatusModalStatus.SUCCESS &&
                <>
                  <GridItem colSpan={12}>
                    <Text textAlign={'center'}>Operation success.</Text>
                  </GridItem>
                  <GridItem colSpan={12} textAlign={'center'}>
                    <Icon fontSize="40px" color="green">
                      <FaRegCheckCircle />
                    </Icon>
                  </GridItem>
                  <GridItem colSpan={12} textAlign={'center'}>
                    <ChakraLink target="_blank" href={`https://stellar.expert/explorer/${activeChain?.name?.toLowerCase()}/tx/${status.txHash}`} >
                      View on explorer <LuExternalLink />
                    </ChakraLink>
                  </GridItem>
                </>
              }
              {status.status === TransactionStatusModalStatus.ERROR &&
                <>
                  <GridItem colSpan={12}>
                    <Text textAlign={'center'}>Operation failed.</Text>
                  </GridItem>
                  <GridItem colSpan={12} textAlign={'center'}>
                    <Icon fontSize="40px" color="red">
                      <FaRegCircleXmark />
                    </Icon>
                  </GridItem>
                  <GridItem colSpan={12} textAlign={'center'}>
                    <Text fontSize={'xs'}>Error message: {status.error}</Text>
                  </GridItem>
                </>
              }
            </Grid>
          </StepsContent>
        </StepsRoot>
      </DialogBody>
    </DialogContent>
  )
}