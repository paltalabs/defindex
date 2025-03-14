import React, { useContext, useEffect, useState } from "react";
import { useSorobanReact } from "@soroban-react/core";
import {
  Address,
  nativeToScVal,
  scValToNative,
  xdr,
} from "@stellar/stellar-sdk";
import { randomBytes } from "crypto";

import { useAppDispatch, useAppSelector } from "@/store/lib/storeHooks"
import { pushVault } from '@/store/lib/features/walletStore'
import { NewVaultState, VaultData } from "@/store/lib/types";

import { useFactoryCallback, FactoryMethod } from '@/hooks/useFactory'
import { ModalContext, TransactionStatusModalStatus } from "@/contexts";

import { AccordionItems, FormControlInterface, VaultPreview } from "./VaultPreview";
import { DialogBody, DialogCloseTrigger, DialogFooter, DialogHeader, DialogTitle } from "../ui/dialog";
import { Button } from "@chakra-ui/react"
import { resetNewVault } from "@/store/lib/features/vaultStore";
import { useVault } from "@/hooks/useVault";
import { getAssetAmountsSCVal, getAssetParamsSCVal, getCreateDeFindexVaultParams } from "@/helpers/vault";

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
  const feeReceiverString = useAppSelector(state => state.newVault.feeReceiver)
  const { transactionStatusModal: txModal, deployVaultModal: deployModal } = useContext(ModalContext);
  const dispatch = useAppDispatch();
  const { getFees } = useVault()

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
    deployModal.setIsOpen(false)
    txModal.initModal();

    const vaultName = nativeToScVal(indexName, { type: "string" })
    const vaultSymbol = nativeToScVal(indexSymbol, { type: "string" })
    const vaultShare = nativeToScVal(indexShare, { type: "u32" })
    const emergencyManager = new Address(emergencyManagerString)
    const feeReceiver = new Address(feeReceiverString)
    const manager = new Address(managerString)
    const salt = randomBytes(32)


    const assetParamsScVal = getAssetParamsSCVal(newVault.assets);
    const assetParamsScValVec = xdr.ScVal.scvVec(assetParamsScVal);
    const amountsScVal = getAssetAmountsSCVal(newVault.assets);
    const amountsScValVec = xdr.ScVal.scvVec(amountsScVal);
    /*

    roles: Map<u32, Address>,
    vault_fee: u32,
    assets: Vec<AssetStrategySet>,
    soroswap_router: Address,
    name_symbol: Map<String, String>,
    upgradable: bool,
*/
    let result: any;
    /* 
      emergency_manager: Keypair,
      rebalance_manager: Keypair,
      fee_receiver: Keypair,
      manager: Keypair,
      vault_fee: number,
      vault_name: string,
      vault_symbol: string,
      asset_allocations: xdr.ScVal[],
      router_address: Address,
      upgradable: boolean,
    */
    const routerAddress = 'CC6WRJYMZA574TOXNO2ZWU4HIXJ5OLKGB7JF556RKMZPSV2V62SLBTPK';
    if (newVault.assets[0]?.amount === undefined) {
      const vault_params: xdr.ScVal[] = getCreateDeFindexVaultParams(
        emergencyManager.toString(),
        emergencyManager.toString(),
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
      investedFunds: [{ address: '', amount: 0 }],
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