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
import { Asset, NewVaultState, VaultData } from "@/store/lib/types";

import { useFactoryCallback, FactoryMethod } from '@/hooks/useFactory'
import { ModalContext, TransactionStatusModalStatus } from "@/contexts";

import { AccordionItems, FormControlInterface, VaultPreview } from "./VaultPreview";
import { DialogBody, DialogCloseTrigger, DialogFooter, DialogHeader, DialogTitle } from "../ui/dialog";
import { Button } from "@chakra-ui/react"
import { resetNewVault } from "@/store/lib/features/vaultStore";
import { useVault } from "@/hooks/useVault";

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
    const amountsScVal = newVault.assets.map((asset, index) => {
      const parsedAmount = newVault.assets[index]?.amount || 0;
      const truncatedAmount = Math.floor(parsedAmount * 1e7) / 1e7;
      const convertedAmount = Number(truncatedAmount) * Math.pow(10, 7)
      if (newVault.assets[index]?.amount === 0) return nativeToScVal(0, { type: "i128" });
      return nativeToScVal(convertedAmount, { type: "i128" });
    });
   /*  const amountsScVal = newVault.amounts.map((amount) => {
      return nativeToScVal((amount * Math.pow(10, 7)), { type: "i128" });
    }); */
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
    let result: any;


    if (newVault.assets[0]?.amount === undefined) {
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
    const tempVault: VaultData = {
      ...newVault,
      address: parsedResult,
      emergencyManager: emergencyManagerString,
      feeReceiver: feeReceiverString,
      manager: managerString,
      TVL: 0,
      totalSupply: 0,
      idleFunds: idleFunds,
      investedFunds: investedFunds ?? [],
    }
    await txModal.handleSuccess(result.txHash);
    dispatch(pushVault(tempVault));
    dispatch(resetNewVault());
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