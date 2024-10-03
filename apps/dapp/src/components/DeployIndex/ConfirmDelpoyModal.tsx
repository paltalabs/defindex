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
  useSteps,
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
import { Strategy } from "@/store/lib/features/vaultStore";
import { useSorobanReact } from "@soroban-react/core";

import { randomBytes } from "crypto";
import { StrategyMethod, useStrategyCallback } from "@/hooks/useStrategy";

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
  const strategies: Strategy[] = useAppSelector(state => state.newVault.strategies);
  const indexName = useAppSelector(state => state.newVault.name)
  const managerString = useAppSelector(state => state.newVault.manager)
  const emergencyManagerString = useAppSelector(state => state.newVault.emergencyManager)
  const feeReceiverString = useAppSelector(state => state.newVault.feeReceiver)

  const dispatch = useAppDispatch();
  const [chartData, setChartData] = useState<ChartData[]>([]);
  const [status, setStatus] = useState<Status>({
    isSuccess: false,
    hasError: false,
    network: undefined,
    message: undefined,
    txHash: undefined
  });

  const [loadingAssets, setLoadingAssets] = useState(true);
  const [assets, setAssets] = useState<string[]>([]);

  const [deployDisabled, setDeployDisabled] = useState(true);

  const strategyCallback = useStrategyCallback();

  useEffect(() => {
    if (
      strategies.length > 0
      && managerString !== ""
      && emergencyManagerString !== ""
      && feeReceiverString !== ""
      && assets.length > 0
      && loadingAssets === false
    ) {
      setDeployDisabled(false);
    } else {
      setDeployDisabled(true);
    }
  }, [strategies, managerString, emergencyManagerString, feeReceiverString])

  useEffect(() => {
    const fetchAssets = async () => {
      setLoadingAssets(true);
      try {
        const assetsPromises = strategies.map((param) =>
          strategyCallback(
            param.address,
            StrategyMethod.ASSET,
            undefined,
            false
          ).then((result) => {
            const resultScval = result as xdr.ScVal;
            const asset = scValToNative(resultScval);
            return asset;
          })
        );
        const assetsArray = await Promise.all(assetsPromises);
        setAssets(assetsArray);
        setLoadingAssets(false);
      } catch (error) {
        console.error(error);
        setLoadingAssets(false);
      }
    };

    fetchAssets();

  }, [strategies]);

  const deployDefindex = async () => {

    if (managerString === "" || managerString === undefined) {
      console.log("Set manager please")
      return
    }
    if (emergencyManagerString === "" || emergencyManagerString === undefined) {
      console.log("Set emergency manager please")
      return
    }
    if (feeReceiverString === "" || feeReceiverString === undefined) {
      console.log("Set fee receiver please")
      return
    }

    const emergencyManager = new Address(emergencyManagerString)
    const feeReceiver = new Address(feeReceiverString)
    const manager = new Address(managerString)
    const salt = randomBytes(32)

    const ratios = [1];

    const strategyParamsScVal = strategies.map((param) => {
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
      xdr.ScVal.scvVec(assets.map((token) => new Address(token).toScVal())),
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
                isDisabled={deployDisabled}
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