import { useSorobanReact } from "@soroban-react/core";
import {
  Address,
  scValToNative,
  xdr,
} from "@stellar/stellar-sdk";

import { useContext, useEffect, useState } from "react";

import { useAppDispatch, useAppSelector } from "@/store/lib/storeHooks"
import { pushVault } from '@/store/lib/features/walletStore'
import { NewVaultState, VaultData } from "@/store/lib/types";

import { ModalContext, TransactionStatusModalStatus } from "@/contexts";
import { FactoryMethod, useFactoryCallback } from '@/hooks/useFactory';

import { DialogBody, DialogCloseTrigger, DialogFooter, DialogHeader, DialogTitle } from "../ui/dialog";
import { AccordionItems, FormControlInterface, VaultPreview } from "./VaultPreview";
import { Button } from "@chakra-ui/react"
import { resetNewVault } from "@/store/lib/features/vaultStore";
import { useVault } from "@/hooks/useVault";
import { getAssetParamsSCVal, getCreateDeFindexVaultDepositParams, getCreateDeFindexVaultParams } from "@/helpers/vault";
import { soroswapRouter } from "@/constants/constants";

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
  const { getInvestedFunds } = useVault();
  const newVault: NewVaultState = useAppSelector(state => state.newVault);
  const indexName = useAppSelector(state => state.newVault.name)
  const indexSymbol = useAppSelector(state => state.newVault.symbol)
  const indexShare = useAppSelector(state => state.newVault.vaultShare)
  const managerString = useAppSelector(state => state.newVault.manager)
  const emergencyManagerString = useAppSelector(state => state.newVault.emergencyManager)
  const rebalanceManagerString = useAppSelector(state => state.newVault.rebalanceManager)
  const feeReceiverString = useAppSelector(state => state.newVault.feeReceiver)
  const [routerAddress, setRouterAddress] = useState<string>('')
  const { transactionStatusModal: txModal, deployVaultModal: deployModal } = useContext(ModalContext);
  const dispatch = useAppDispatch();
  const { getFees } = useVault()

  const [deployDisabled, setDeployDisabled] = useState(true);

  useEffect(() => {
    if (
      managerString !== ""
      && emergencyManagerString !== ""
      && rebalanceManagerString !== ""
      && feeReceiverString !== ""
      && !indexShare
    ) {
      setDeployDisabled(false);
    } else {
      setDeployDisabled(true);
    }
  }, [managerString, emergencyManagerString, rebalanceManagerString, feeReceiverString])

  const autoCloseModal = async () => {
    await new Promise(resolve => setTimeout(resolve, 30000))
    txModal.resetModal();
    onClose();
  }

  useEffect(() => {
    if (txModal.status !== TransactionStatusModalStatus.PENDING) {
      autoCloseModal();
    }
  }, [txModal.status])
  const [buttonText, setButtonText] = useState<string>('')
  const [accordionValue, setAccordionValue] = useState<AccordionItems[]>([AccordionItems.STRATEGY_DETAILS])
  const [formControl, setFormControl] = useState<FormControlInterface>({
    emergencyManager: {
      isValid: undefined,
      value: undefined
    },
    feeReceiver: {
      isValid: undefined,
      value: undefined
    },
    manager: {
      isValid: undefined,
      value: undefined
    },
    rebalanceManager: {
      isValid: undefined,
      value: undefined
    },
    upgradable: true,
    vaultShare: 0
  })

  useEffect(() => {
    if (managerString === '' || emergencyManagerString === '' || rebalanceManagerString === '') {
      setButtonText('Fill manager info')
      return
    } else if (feeReceiverString === '' || indexShare === 0) {
      setButtonText('Fill fee settings')
      return
    } else {
      setButtonText('Deploy')
    }

  }, [managerString, emergencyManagerString, rebalanceManagerString, feeReceiverString, indexShare])

  useEffect(() => {
    switch (activeChain?.id.toLowerCase()) {
      case 'testnet':
        setRouterAddress(soroswapRouter.testnet)
        break;
      case 'public':
        setRouterAddress(soroswapRouter.mainnet)
        break;
      default:
        setRouterAddress(soroswapRouter.testnet)
        break;
    }
  }, [activeChain?.id])

  const deployDefindex = async () => {
    let result: any;

    if (managerString === '' || emergencyManagerString === '' || rebalanceManagerString === '') {
      console.log('please fill manager config')
      setAccordionValue([AccordionItems.MANAGER_CONFIGS])
      return
    }
    if (feeReceiverString === '' || indexShare === 0) {
      console.log('please fill the fee reciever info')
      setAccordionValue([AccordionItems.FEES_CONFIGS])
      return
    }
    deployModal.setIsOpen(false)
    txModal.initModal();

    const emergencyManager = new Address(emergencyManagerString)
    const rebalanceManager = new Address(rebalanceManagerString)
    const feeReceiver = new Address(feeReceiverString)
    const manager = new Address(managerString)
    const assetParamsScVal = getAssetParamsSCVal(newVault.assets);

    if (newVault.assets[0]?.amount === undefined) {
      const vault_params: xdr.ScVal[] = getCreateDeFindexVaultParams(
        emergencyManager.toString(),
        rebalanceManager.toString(),
        feeReceiver.toString(),
        manager.toString(),
        indexShare,
        indexName,
        indexSymbol,
        assetParamsScVal,
        routerAddress,
        true
      );
      try {
        result = await factory(
          FactoryMethod.CREATE_DEFINDEX_VAULT,
          vault_params,
          true,
        )
      }
      catch (e: any) {
        console.error(e)
        dispatch(resetNewVault());
        txModal.handleError(e.toString());
        return
      }
    } else if (newVault.assets[0]?.amount! > 0) {
      if (!address) throw new Error('Address not found')
      const createDefindexDepositParams: xdr.ScVal[] = getCreateDeFindexVaultDepositParams(
        address,
        emergencyManager.toString(),
        feeReceiver.toString(),
        rebalanceManager.toString(),
        manager.toString(),
        indexShare,
        indexName,
        indexSymbol,
        assetParamsScVal,
        routerAddress,
        true,
        newVault.assets
      )
      try {
        result = await factory(
          FactoryMethod.CREATE_DEFINDEX_VAULT_DEPOSIT,
          createDefindexDepositParams,
          true,
        )
      }
      catch (e: any) {
        console.error(e)
        dispatch(resetNewVault());
        txModal.handleError(e.toString());
        return
      }
    }
    const parsedResult: string = scValToNative(result.returnValue);
    if (parsedResult.length !== 56) throw new Error('Invalid result')
    const idleFunds = newVault.assets.map((asset, index) => {
      return {
        address: asset.address,
        amount: newVault.assets[index]?.amount || 0
      }
    })
    const investedFunds = await getInvestedFunds(parsedResult);
    const fees = await getFees(parsedResult)
    const tempVault: VaultData = {
      ...newVault,
      address: parsedResult,
      emergencyManager: emergencyManagerString,
      feeReceiver: feeReceiverString,
      manager: managerString,
      TVL: 0,
      totalSupply: 0,
      idleFunds: idleFunds,
      investedFunds: investedFunds || [{ address: '', amount: 0 }],
      fees: fees,
    }
    await txModal.handleSuccess(result.txHash);
    dispatch(pushVault(tempVault));
    dispatch(resetNewVault());
    return result;
  }

  return (
    <>
      <DialogHeader>
        <DialogTitle>
          Deploying {indexName === "" ? 'new index' : indexName}
        </DialogTitle>
      </DialogHeader>
      <DialogCloseTrigger />
      <DialogBody>
        <VaultPreview
          data={newVault.assets}
          accordionValue={accordionValue}
          setAccordionValue={setAccordionValue}
          formControl={formControl}
          setFormControl={setFormControl}
        />
      </DialogBody>

      <DialogFooter>
        <Button
          aria-label='add_strategy'
          colorScheme='green'
          onClick={deployDefindex}>
          {buttonText}
        </Button>
      </DialogFooter>
    </>
  )
}