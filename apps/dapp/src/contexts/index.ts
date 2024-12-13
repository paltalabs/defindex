import React from 'react';

export enum TransactionStatusModalStatus {
  SUCCESS = 'success',
  ERROR = 'error',
  PENDING = 'pending',
}
export enum TransactionStatusModalOperation {
  DEPLOY_VAULT = 'deploy_vault',
  DEPOSIT = 'deposit',
  WITHDRAW = 'withdraw',
  EMERGENCY_WITHDRAW = 'emergency_withdraw',
}

type ToggleModalProps = {
  isOpen: boolean
  setIsOpen: (value: boolean) => void
};

type TransactionStatusModalProps = {
  isOpen: boolean
  setIsOpen: (value: boolean) => void
  step: number
  setStep: (value: number) => void
  status: TransactionStatusModalStatus | ''
  setStatus: (value: TransactionStatusModalStatus | '') => void
  operation: TransactionStatusModalOperation | ''
  setOperation: (value: TransactionStatusModalOperation | '') => void
  error: string
  setError: (value: string) => void
  txHash: string
  setTxHash: (value: string) => void
  resetModal: () => void
  initModal: () => void
  handleSuccess: (txHash: string) => void
  handleError: (error: string) => void
}
export type ModalContextType = {
  transactionStatusModal: TransactionStatusModalProps,
  deployVaultModal: ToggleModalProps,
  inspectVaultModal: ToggleModalProps,
  interactWithVaultModal: ToggleModalProps,
  editVaultModal: ToggleModalProps,
  rebalanceVaultModal: ToggleModalProps,
  investStrategiesModal: ToggleModalProps,

};
export const ModalContext = React.createContext<ModalContextType>({
  transactionStatusModal:{
    isOpen: false,
    setIsOpen: () => {},
    step: 0,
    setStep: () => {},
    status: TransactionStatusModalStatus.PENDING,
    setStatus: (value: TransactionStatusModalStatus |'') => {},
    operation: '',
    setOperation: (value: TransactionStatusModalOperation | '') => {},
    error: '',
    setError: (value: string) => {},
    txHash: '',
    setTxHash: (value: string) => {},
    resetModal: () => {},
    initModal: () => {},
    handleSuccess: (txHash: string) => {},
    handleError: (error: string) => {},
  },
  deployVaultModal: {
    isOpen: false,
    setIsOpen: () => {},
  },
  inspectVaultModal: {
    isOpen: false,
    setIsOpen: () => {},
  },
  interactWithVaultModal: {
    isOpen: false,
    setIsOpen: () => {},
  },
  editVaultModal: {
    isOpen: false,
    setIsOpen: () => {},
  },
  rebalanceVaultModal: {
    isOpen: false,
    setIsOpen: () => {},
  },
  investStrategiesModal: {
    isOpen: false,
    setIsOpen: () => {},
  },
});