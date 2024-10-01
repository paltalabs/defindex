import React from "react";
import { useAppDispatch, useAppSelector } from "@/store/lib/storeHooks"
import {
  Box,
  Button,
  CircularProgress,
  Link,
  Modal,
  ModalBody,
  ModalCloseButton,
  ModalContent,
  ModalFooter,
  ModalHeader,
  ModalOverlay,
  Text,
  useSteps
} from "@chakra-ui/react"
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
import { WarningIcon, CheckCircleIcon, ExternalLinkIcon } from '@chakra-ui/icons'
import { Strategy } from "@/store/lib/features/strategiesStore";
import { useSorobanReact } from "@soroban-react/core";

import { randomBytes } from "crypto";


interface Status {
  isSuccess: boolean,
  hasError: boolean,
  message: string | undefined,
  network: "public" | "testnet" | undefined,
  txHash: string | undefined
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
  const sorobanContext = useSorobanReact();
  const { activeChain } = sorobanContext;
  const factory = useFactoryCallback();
  const strategies: Strategy[] = useAppSelector(state => state.strategies.strategies);
  const indexName = useAppSelector(state => state.strategies.strategyName)
  const dispatch = useAppDispatch();
  const [chartData, setChartData] = useState<ChartData[]>([]);
  const [status, setStatus] = useState<Status>({
    isSuccess: false,
    hasError: false,
    network: undefined,
    message: undefined,
    txHash: undefined
  });
  const deployDefindex = async () => {

    const emergencyManager = new Address('GAFS3TLVM2GO66QMOZJHJFP463K3ZKAPGU23WBMCPPFXIG7OUDMDDNTM')
    const feeReceiver = new Address('GCWCI55WCOFF73ZL7NQAKJG4TTFPLE4Y23Z7KDXYLSF5Y3LX5XH7UNES')
    const manager = new Address('GCRSJ7BPRVHE3SCQMS7XRDPAPCUYNZ4EK5X7OA5UIUDTSN7DP2SLMTQJ')
    const salt = randomBytes(32)

    const xlm_address = "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC"

    const tokens = [xlm_address];
    const ratios = [1];

    const strategyParamsRaw = [
      {
        name: "Strategy 1",
        address: "CDIBYFBYBV3D3DQNSUDMVQNWYCQPYKSOLKSEOF3WEQOIXB56K54Q4G6W", //TODO: Use a deployed strategy address here
      },
    ];

    const strategyParamsScVal = strategyParamsRaw.map((param) => {
      return xdr.ScVal.scvMap([
        new xdr.ScMapEntry({
          key: xdr.ScVal.scvSymbol('address'),
          val: new Address(param.address).toScVal(),
        }),
        new xdr.ScMapEntry({
          key: xdr.ScVal.scvSymbol('name'),
          val: nativeToScVal(param.name, { type: "string" }),
        }),
      ]);
    });

    const strategyParamsScValVec = xdr.ScVal.scvVec(strategyParamsScVal);

    const createDefindexParams: xdr.ScVal[] = [
      emergencyManager.toScVal(),
      feeReceiver.toScVal(),
      manager.toScVal(),
      xdr.ScVal.scvVec(tokens.map((token) => new Address(token).toScVal())),
      xdr.ScVal.scvVec(ratios.map((ratio) => nativeToScVal(ratio, { type: "u32" }))),
      strategyParamsScValVec,
      nativeToScVal(salt),
    ];


    goToNext();
    let result: any;
    try {
      result = await factory(
        FactoryMethod.CREATE_DEFINDEX_VAULT,
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
        message: e.toString(),
        txHash: undefined,
      })
      return
    }
    console.log(result.txHash)
    const parsedResult = scValToNative(result.returnValue);
    dispatch(pushIndex(parsedResult));
    setActiveStep(3);
    setStatus({
      ...status,
      isSuccess: true,
      hasError: false,
      message: 'Index deployed successfully.',
      txHash: result.txHash
    });
    return result;
  }

  const handleCloseModal = async () => {
    setStatus({
      ...status,
      isSuccess: false,
      hasError: false,
      message: undefined,
      txHash: undefined
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
    await new Promise(resolve => setTimeout(resolve, 30000))
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
            {(activeStep == 3 && status.isSuccess === true && status.txHash != undefined) && (
              <>
                <Box mt={8} textAlign={'center'}>
                  <CheckCircleIcon boxSize={'4em'} color={'green'} />
                  <Text mt={4}>{`${status.message}`}</Text>
                </Box>
                <Box mt={8} textAlign={'center'}>
                  <Link mt={4} href={`https://stellar.expert/explorer/${activeChain?.name?.toLowerCase()}/tx/${status.txHash}`} isExternal>
                    View on explorer <ExternalLinkIcon mx='2px' />
                  </Link>
                </Box>
              </>
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