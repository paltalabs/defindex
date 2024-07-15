import { useAppDispatch, useAppSelector } from "@/store/lib/storeHooks"
import { 
  Box, 
  Button, 
  CircularProgress, 
  IconButton, 
  Modal, 
  ModalBody, 
  ModalCloseButton, 
  ModalContent, 
  ModalFooter, 
  ModalHeader, 
  ModalOverlay, 
  Text, 
  useSteps } from "@chakra-ui/react"
import {
  Address,
  nativeToScVal,
  scValToNative,
  xdr,
} from "@stellar/stellar-sdk";
import { pushIndex } from '@/store/lib/features/walletStore'
import { useFactoryCallback, FactoryMethod } from '@/hooks/useFactory'
import { IndexPreview } from "./IndexPreview";
import { DeploySteps } from "./DeploySteps";
import { useState } from "react";
import { WarningIcon, CheckCircleIcon } from '@chakra-ui/icons'

interface Status {
  isSuccess: boolean,
  hasError: boolean,
  message: string | undefined,
}
export const ConfirmDelpoyModal = ({ isOpen, onClose }: { isOpen: boolean, onClose: () => void }) => {
  const { goToNext, setActiveStep, activeStep } = useSteps({
    index: 0
  })
  const factory = useFactoryCallback()
  const adapters = useAppSelector(state => state.adapters.adapters)
  const dispatch = useAppDispatch();
  const [status, setStatus] = useState<Status>({
    isSuccess: false,
    hasError: false,
    message: undefined
  })
  const deployDefindex = async () => {
    const adapterAddressPairScVal = adapters.map((adapter, index) => {
      return xdr.ScVal.scvMap([
        new xdr.ScMapEntry({
          key: xdr.ScVal.scvSymbol("address"),
          val: (new Address(adapter.address)).toScVal(),

        }),
        new xdr.ScMapEntry({
          key: xdr.ScVal.scvSymbol("index"),
          val: xdr.ScVal.scvU32(index),
        }),
        new xdr.ScMapEntry({
          key: xdr.ScVal.scvSymbol("share"),
          val: xdr.ScVal.scvU32(adapter.value),
        }),
      ]);
    });

    const adapterAddressesScVal = xdr.ScVal.scvVec(adapterAddressPairScVal);

    const createDefindexParams: xdr.ScVal[] = [adapterAddressesScVal];
    console.log('deploying Defindex')
    goToNext()
    const result: any = await factory(
      FactoryMethod.CREATE_DEFINDEX,
      createDefindexParams,
      true,
    )
    const parsedResult = scValToNative(result.returnValue);
    dispatch(pushIndex(parsedResult))
    setActiveStep(3)
    setStatus({
      isSuccess: true,
      hasError: false,
      message: 'Index deployed successfully.'
    })
    return result;
  }
  const handleDeploy = async () => {
    console.log('deploying Defindex')
    try {
      await deployDefindex()
    } catch (e) {
      setActiveStep(3)
      setStatus({
        ...status,
        hasError: true,
        message: 'Could not deploy this index, if the problem persist please contact support.'
      })
    }
    // const result: any = await factory(
    //   FactoryMethod.CREATE_DEFINDEX,
    //   createDefindexParams,
    //   true,
    // )
    // const parsedResult = scValToNative(result.returnValue);
    // dispatch(pushIndex(parsedResult))
    // return result;
  }
  const handleCloseModal = () => {
    setStatus({
      isSuccess: false,
      hasError: false,
      message: undefined
    })
    setActiveStep(0)
    onClose()
  }

  return (
    <>
      <Modal isOpen={isOpen} onClose={handleCloseModal} isCentered>
        <ModalOverlay />
        <ModalContent minW={'40vw'}>
          <ModalHeader>Deploying new Index</ModalHeader>
          <ModalCloseButton />
          <ModalBody>
            <DeploySteps activeStep={activeStep} hasError={status.hasError} />
            {activeStep == 0 && (
              <IndexPreview data={adapters} />
            )}
            {activeStep == 1 && (
              <Box textAlign={'center'}>
                <Text mt={8}>Please, sign the transaction in your wallet.</Text>
                <CircularProgress mt={8} isIndeterminate color='green.500' />
              </Box>
            )}
            {(activeStep == 3 && status.hasError) && (
              <Box mt={8} textAlign={'center'}>
                <WarningIcon boxSize={'4em'} color={'red'} />
                <Text mt={4}>{`${status.message}`}</Text>
              </Box>
            )}
            {(activeStep == 3 && !!!status.hasError) && (
              <Box mt={8} textAlign={'center'}>
                <CheckCircleIcon boxSize={'4em'} color={'green'} />
                <Text mt={4}>{`${status.message}`}</Text>
              </Box>
            )}
          </ModalBody>

          <ModalFooter>
            {(activeStep == 0 && !status.hasError) && (
              <Button
                aria-label='add_adapter'
                colorScheme='green'
                onClick={handleDeploy}>
                Deploy
              </Button>
            )}
          </ModalFooter>
        </ModalContent>
      </Modal>
    </>
  )
}