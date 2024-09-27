import { useAppDispatch, useAppSelector } from "@/store/lib/storeHooks"
import { 
  Box, 
  Button, 
  CircularProgress, 
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
import { useEffect, useState } from "react";
import { WarningIcon, CheckCircleIcon } from '@chakra-ui/icons'
import { resetStrategies, Strategy } from "@/store/lib/features/strategiesStore";

interface Status {
  isSuccess: boolean,
  hasError: boolean,
  message: string | undefined,
}

export interface ChartData {
  id: number
  label: string
  value: number
  address?: string
  color?: string
}

export const ConfirmDelpoyModal = ({ isOpen, onClose }: { isOpen: boolean, onClose: () => void }) => {
  const { goToNext, setActiveStep, activeStep } = useSteps({
    index: 0
  });
  const factory = useFactoryCallback();
  const strategies: Strategy[] = useAppSelector(state => state.strategies.strategies);
  const indexName = useAppSelector(state => state.strategies.strategyName)
  const dispatch = useAppDispatch();
  const [chartData, setChartData] = useState<ChartData[]>([]);
  const [status, setStatus] = useState<Status>({
    isSuccess: false,
    hasError: false,
    message: undefined
  });
  const deployDefindex = async () => {
    const strategyAddressPairScVal = strategies.map((strategy, index) => {
      return xdr.ScVal.scvMap([
        new xdr.ScMapEntry({
          key: xdr.ScVal.scvSymbol("address"),
          val: (new Address(strategy.address)).toScVal(),

        }),
        new xdr.ScMapEntry({
          key: xdr.ScVal.scvSymbol("index"),
          val: xdr.ScVal.scvU32(index),
        }),
        new xdr.ScMapEntry({
          key: xdr.ScVal.scvSymbol("share"),
          val: xdr.ScVal.scvU32(strategy.value),
        }),
      ]);
    });

    const strategyAddressesScVal = xdr.ScVal.scvVec(strategyAddressPairScVal);

    const createDefindexParams: xdr.ScVal[] = [strategyAddressesScVal];

    goToNext();
    let result: any;
    try {
      result = await factory(
        FactoryMethod.CREATE_DEFINDEX,
        createDefindexParams,
        true,
      )
    }
    catch (e: any) {
      console.error(e)
      setActiveStep(3)
      setStatus({
        ...status,
        hasError: true,
        message: e.toString()
      })
      return
    }
    const parsedResult = scValToNative(result.returnValue);
    dispatch(pushIndex(parsedResult));
    setActiveStep(3);
    setStatus({
      isSuccess: true,
      hasError: false,
      message: 'Index deployed successfully.'
    });
    return result;
  }

  const handleCloseModal = async () => {
    setStatus({
      isSuccess: false,
      hasError: false,
      message: undefined
    });
    setActiveStep(0);
    //await dispatch(resetStrategies())
    onClose();
  }

  useEffect(() => {
    const newChartData: ChartData[] = strategies.map((strategy: any, index: number) => {
      return {
        id: index,
        label: strategy.name,
        address: strategy.address,
        value: strategy.value,
      }
    });
    const total = newChartData.reduce((acc: number, curr: any) => acc + curr.value, 0)
    if (total == 100) {
      setChartData(newChartData);
      return;
    } else {
      newChartData.push({
        id: newChartData.length,
        label: 'Unassigned',
        value: 100 - newChartData.reduce((acc: number, curr: any) => acc + curr.value, 0),
        address: undefined,
        color: '#e0e0e0'
      })
      setChartData(newChartData);
      return;
    }
  }, [strategies]);

  const autoCloseModal = async () => {
    await new Promise(resolve => setTimeout(resolve, 5000))
    handleCloseModal();
  }

  useEffect(() => {
    if (status.isSuccess || status.hasError) {
      autoCloseModal();
    }
  }, [status.isSuccess, status.hasError])
  return (
    <>
      <Modal isOpen={isOpen} onClose={handleCloseModal} isCentered>
        <ModalOverlay />
        <ModalContent minW={'40vw'}>
          <ModalHeader>Deploying {indexName === "" ? 'new index' : indexName}</ModalHeader>
          <ModalCloseButton />
          <ModalBody>
            <DeploySteps activeStep={activeStep} hasError={status.hasError} />
            {activeStep == 0 && (
              <IndexPreview data={chartData} />
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
                aria-label='add_strategy'
                colorScheme='green'
                onClick={deployDefindex}>
                Deploy
              </Button>
            )}
          </ModalFooter>
        </ModalContent>
      </Modal>
    </>
  )
}