import React from "react";
import { useAppDispatch, useAppSelector } from "@/store/lib/storeHooks"
import {
  Box,
  Button,
  Link,
  ProgressCircleRoot,
  Text,
} from "@chakra-ui/react"
import {
  Address,
  nativeToScVal,
  scValToNative,
  xdr,
} from "@stellar/stellar-sdk";
import { pushVault } from '@/store/lib/features/walletStore'
import { useFactoryCallback, FactoryMethod } from '@/hooks/useFactory'
import { AccordionItems, FormControlInterface, VaultPreview } from "./VaultPreview";
import { DeploySteps } from "./DeploySteps";
import { useEffect, useState } from "react";
import { PiWarningCircleFill } from "react-icons/pi";
import { FaCheckCircle } from "react-icons/fa";
import { useSorobanReact } from "@soroban-react/core";

import { randomBytes } from "crypto";
import { LuExternalLink } from "react-icons/lu";
import { ProgressCircleRing } from "../ui/progress-circle";
import { DialogBody, DialogCloseTrigger, DialogFooter, DialogHeader, DialogTitle } from "../ui/dialog";
import { Asset, NewVaultState } from "@/store/lib/types";

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
  const sorobanContext = useSorobanReact();
  const { activeChain, address } = sorobanContext;
  const factory = useFactoryCallback();
  const newVault: NewVaultState = useAppSelector(state => state.newVault);
  //const strategies: Strategy[] = newVault.strategies;
  const indexName = useAppSelector(state => state.newVault.name)
  const indexSymbol = useAppSelector(state => state.newVault.symbol)
  const indexShare = useAppSelector(state => state.newVault.vaultShare)
  const managerString = useAppSelector(state => state.newVault.manager)
  const emergencyManagerString = useAppSelector(state => state.newVault.emergencyManager)
  const feeReceiverString = useAppSelector(state => state.newVault.feeReceiver)

  const dispatch = useAppDispatch();
  const [assets, setAssets] = useState<Asset[]>([]);
  const [status, setStatus] = useState<Status>({
    isSuccess: false,
    hasError: false,
    network: undefined,
    message: undefined,
    txHash: undefined
  });

  const [deployDisabled, setDeployDisabled] = useState(true);

  useEffect(() => {
    if (
      managerString !== ""
      && emergencyManagerString !== ""
      && feeReceiverString !== ""
      && !indexShare 
    ) {
      setDeployDisabled(false);
    } else {
      setDeployDisabled(true);
    }
  }, [managerString, emergencyManagerString, feeReceiverString])




  const handleCloseModal = async () => {
    setStatus({
      ...status,
      isSuccess: false,
      hasError: false,
      message: undefined,
      txHash: undefined
    });
    //setActiveStep(0);
    //await dispatch(resetStrategies())
    onClose();
  }

  /* useEffect(() => {
    const newChartData: ChartData[] = strategies.map((strategy: Strategy, index: number) => {
      return {
        id: index,
        label: strategy.name,
        address: strategy.address,
        value: strategy.share,
      }
    });
    const total = newChartData.reduce((acc: number, curr: ChartData) => acc + curr.value, 0)
    if (total == 100) {
      setChartData(newChartData);
      return;
    } else {
      newChartData.push({
        id: newChartData.length,
        label: 'Unassigned',
        value: 100 - newChartData.reduce((acc: number, curr: ChartData) => acc + curr.value, 0),
        address: undefined,
        color: '#e0e0e0'
      })
      setChartData(newChartData);
      return;
    }
  }, [strategies]); */

  const autoCloseModal = async () => {
    await new Promise(resolve => setTimeout(resolve, 30000))
    handleCloseModal();
  }

  useEffect(() => {
    if (status.isSuccess || status.hasError) {
      autoCloseModal();
    }
  }, [status.isSuccess, status.hasError])
  const activeStep: number = 0;
  const [buttonText, setButtonText] = useState<string>('')
  const [accordionValue, setAccordionValue] = useState<AccordionItems[]>([AccordionItems.STRATEGY_DETAILS])
  const [formControl, setFormControl] = useState<FormControlInterface>({
    manager: {
      isValid: undefined,
      value: undefined
    },
    emergencyManager: {
      isValid: undefined,
      value: undefined
    },
    feeReceiver: {
      isValid: undefined,
      value: undefined
    },
    vaultShare: 0
  })

  const handleNext = (accordionValue: AccordionItems[]) => {
    if (accordionValue.length === 0) {
      setAccordionValue([AccordionItems.STRATEGY_DETAILS])
    }
    if (accordionValue[0] === AccordionItems.STRATEGY_DETAILS) {
      setAccordionValue([AccordionItems.MANAGER_CONFIGS])
    }
    if (accordionValue[0] === AccordionItems.MANAGER_CONFIGS && formControl.manager.isValid && formControl.emergencyManager.isValid) {
      setAccordionValue([AccordionItems.FEES_CONFIGS])
    }
    if (accordionValue[0] === AccordionItems.FEES_CONFIGS && formControl.feeReceiver.isValid) {
      setAccordionValue([])
    }
  }
  useEffect(() => {
    if (managerString === '' || emergencyManagerString === '') {
      setButtonText('Fill manager info')
      return
    } else if (feeReceiverString === '' || indexShare === 0) {
      setButtonText('Fill fee settings')
      return
    } else {
      setButtonText('Deploy')
    }

  }, [managerString, emergencyManagerString, feeReceiverString, indexShare])
  const deployDefindex = async () => {
    if (managerString === '' || emergencyManagerString === '') {
      console.log('please fill manager config')
      setAccordionValue([AccordionItems.MANAGER_CONFIGS])
      return
    }
    if (feeReceiverString === '' || indexShare === 0) {
      console.log('please fill the fee reciever info')
      setAccordionValue([AccordionItems.FEES_CONFIGS])
      return
    }

    const vaultName = nativeToScVal(indexName, { type: "string" })
    const vaultSymbol = nativeToScVal(indexSymbol, { type: "string" })
    const vaultShare = nativeToScVal(indexShare, { type: "u32" })
    const emergencyManager = new Address(emergencyManagerString)
    const feeReceiver = new Address(feeReceiverString)
    const manager = new Address(managerString)
    const salt = randomBytes(32)

    /*
        pub struct AssetAllocation {
          pub address: Address,
          pub strategies: Vec<Strategy>,
        } 
        pub struct Strategy {
          pub address: Address,
          pub name: String,
          pub paused: bool,
        }
    */

    const assetParamsScVal = newVault.assets.map((asset) => {
      const strategyParamsScVal = asset.strategies.map((param) => {
        return xdr.ScVal.scvMap([
          new xdr.ScMapEntry({
            key: xdr.ScVal.scvSymbol('address'),
            val: new Address(param.address).toScVal(),
          }),
          new xdr.ScMapEntry({
            key: xdr.ScVal.scvSymbol('name'),
            val: nativeToScVal(param.name, { type: "string" }),
          }),
          new xdr.ScMapEntry({
            key: xdr.ScVal.scvSymbol('paused'),
            val: nativeToScVal(false, { type: "bool" }),
          }),
        ]);
      });
      const strategyParamsScValVec = xdr.ScVal.scvVec(strategyParamsScVal);
      return xdr.ScVal.scvMap([
        new xdr.ScMapEntry({
          key: xdr.ScVal.scvSymbol('address'),
          val: new Address(asset.address).toScVal(),
        }),
        new xdr.ScMapEntry({
          key: xdr.ScVal.scvSymbol('strategies'),
          val: strategyParamsScValVec,
        }),
      ]);
    });
    const assetParamsScValVec = xdr.ScVal.scvVec(assetParamsScVal);
    const amountsScVal = newVault.amounts.map((amount) => {
      return nativeToScVal((amount * Math.pow(10, 7)), { type: "i128" });
    });
    const amountsScValVec = xdr.ScVal.scvVec(amountsScVal);
     /*  fn create_defindex_vault(
      emergency_manager: address, 
      fee_receiver: address, 
      vault_share: u32, 
      vault_name: string, 
      vault_symbol: string, 
      manager: address, 
      assets: vec<AssetAllocation>, 
      salt: bytesn<32>) -> result<address,FactoryError>
 */
    //goToNext();
    let result: any;
    if (amountsScVal.length === 0) {
      const createDefindexParams: xdr.ScVal[] = [
        emergencyManager.toScVal(),
        feeReceiver.toScVal(),
        vaultShare,
        vaultName,
        vaultSymbol,
        manager.toScVal(),
        assetParamsScValVec,
        nativeToScVal(salt),
      ];
      try {
        result = await factory(
          FactoryMethod.CREATE_DEFINDEX_VAULT,
          createDefindexParams,
          true,
        )
      }
      catch (e: any) {
        console.error(e)
        //setActiveStep(3)
        setStatus({
          ...status,
          hasError: true,
          message: e.toString(),
          txHash: undefined,
        })
        return
      }
    } else {
      if (!address) throw new Error('Address not found')
      const caller = new Address(address);
      const createDefindexParams: xdr.ScVal[] = [
        caller.toScVal(),
        emergencyManager.toScVal(),
        feeReceiver.toScVal(),
        vaultShare,
        vaultName,
        vaultSymbol,
        manager.toScVal(),
        assetParamsScValVec,
        amountsScValVec,
        nativeToScVal(salt),
      ];
      try {
        result = await factory(
          FactoryMethod.CREATE_DEFINDEX_VAULT_DEPOSIT,
          createDefindexParams,
          true,
        )
      }
      catch (e: any) {
        console.error(e)
        //setActiveStep(3)
        setStatus({
          ...status,
          hasError: true,
          message: e.toString(),
          txHash: undefined,
        })
        return
      }
    }
    const parsedResult: string = scValToNative(result.returnValue);
    if (parsedResult.length !== 56) throw new Error('Invalid result')
    const tempVault: any = {
      ...newVault,
      address: parsedResult
    }
    dispatch(pushVault(tempVault));
    //setActiveStep(3);
    setStatus({
      ...status,
      isSuccess: true,
      hasError: false,
      message: 'DeFindex deployed successfully.',
      txHash: result.txHash
    });
    return result;
  }

  //to-do Use chakra-ui stepper component
  return (

    <>
      <DialogHeader>
        <DialogTitle>
          Deploying {indexName === "" ? 'new index' : indexName}
        </DialogTitle>
      </DialogHeader>
          <DialogCloseTrigger />
          <DialogBody>
            <DeploySteps activeStep={activeStep} hasError={status.hasError} />
            {activeStep == 0 && (
          <VaultPreview
            data={newVault.assets}
            accordionValue={accordionValue}
            setAccordionValue={setAccordionValue}
            formControl={formControl}
            setFormControl={setFormControl}
          />
            )}
            {activeStep == 1 && (
              <Box textAlign={'center'}>
                <Text mt={8}>Please, sign the transaction in your wallet.</Text>
                <ProgressCircleRoot>
                  <ProgressCircleRing />
                </ProgressCircleRoot>
              </Box>
            )}
            {(activeStep == 3 && status.hasError) && (
              <Box mt={8} textAlign={'center'}>
            <PiWarningCircleFill />
                <Text mt={4}>{`${status.message}`}</Text>
              </Box>
            )}
            {(activeStep == 3 && status.isSuccess === true && status.txHash != undefined) && (
              <>
                <Box mt={8} textAlign={'center'}>
              <FaCheckCircle />
                  <Text mt={4}>{`${status.message}`}</Text>
                </Box>
                <Box mt={8} textAlign={'center'}>
                  <Link mt={4} href={`https://stellar.expert/explorer/${activeChain?.name?.toLowerCase()}/tx/${status.txHash}`}>
                    View on explorer <LuExternalLink />
                  </Link>
                </Box>
              </>
            )}
          </DialogBody>

          <DialogFooter>
        {(activeStep == 0 && !status.hasError) && (
          <Button
            aria-label='add_strategy'
            colorScheme='green'
            onClick={deployDefindex}>
            {buttonText}
          </Button>
        )}
      </DialogFooter>
    </>

  )
}