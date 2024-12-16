'use client'

import React, { ReactNode, useEffect } from "react"
import { ModalContext, ModalContextType, TransactionStatusModalOperation, TransactionStatusModalStatus } from '@/contexts'
import { useDispatch } from 'react-redux'
import { resetNewVault } from '@/store/lib/features/vaultStore'
import { resetSelectedVault } from '@/store/lib/features/walletStore'


export const ModalProvider = ({
  children,
}: {
  children: ReactNode
}) => {
  const dispatch = useDispatch()

  const [isDeployVaultModalOpen, setIsDeployVaultModalOpen] = React.useState<boolean>(false)
  const [isInspectVaultModalOpen, setIsInspectVaultModalOpen] = React.useState<boolean>(false)
  const [isInteractWithVaultModalOpen, setIsInteractWithVaultModalOpen] = React.useState<boolean>(false)
  const [isEditVaultModalOpen, setIsEditVaultModalOpen] = React.useState<boolean>(false)
  const [isRebalanceModalOpen, setIsRebalanceModalOpen] = React.useState<boolean>(false)
  const [isInvestStrategiesModalOpen, setIsInvestStrategiesModalOpen] = React.useState<boolean>(false)

  const [isTransactionStatusModalOpen, setIsTransactionStatusModalOpen] = React.useState<boolean>(false)
  const [transactionStatusModalStep, setTransactionStatusModalStep] = React.useState<number>(0)
  const [transactionStatusModalStatus, setTransactionStatusModalStatus] = React.useState<TransactionStatusModalStatus | ''>(TransactionStatusModalStatus.PENDING)
  const [transactionStatusModalOperation, setTransactionStatusModalOperation] = React.useState<TransactionStatusModalOperation | ''>('')
  const [transactionStatusModalError, setTransactionStatusModalError] = React.useState<string>('')
  const [txHash, setTxHash] = React.useState<string>('')

  const handleResetModal = () => {
    setIsTransactionStatusModalOpen(false)
    setTransactionStatusModalStep(0)
    setTransactionStatusModalStatus(TransactionStatusModalStatus.PENDING)
    setTransactionStatusModalOperation('')
    setTransactionStatusModalError('')
    setTxHash('')
    setIsInspectVaultModalOpen(false)
  }

  const handleFirstStep = setTimeout(() => setTransactionStatusModalStep(1), 3000)
  useEffect(() => {
    if (isTransactionStatusModalOpen && transactionStatusModalStep === 0 && transactionStatusModalStatus === TransactionStatusModalStatus.PENDING) {
      handleFirstStep
    } else if (transactionStatusModalStatus !== TransactionStatusModalStatus.PENDING) {
      clearTimeout(handleFirstStep)
      setTransactionStatusModalStep(2)
    }
  }, [isTransactionStatusModalOpen, transactionStatusModalStep, transactionStatusModalStatus])

  const handleInitModal = () => {
    handleResetModal()
    setIsTransactionStatusModalOpen(true)
  }

  const handleError = (error: string) => {
    clearTimeout(handleFirstStep)
    setTransactionStatusModalError(error)
    setTransactionStatusModalStatus(TransactionStatusModalStatus.ERROR)
    setTransactionStatusModalStep(2)
    setTimeout(() => handleResetModal(), 8000)
  }

  const handleSuccess = (txHash: string) => {
    clearTimeout(handleFirstStep)
    setTxHash(txHash)
    setTransactionStatusModalStatus(TransactionStatusModalStatus.SUCCESS)
    setTransactionStatusModalStep(2)
    dispatch(resetNewVault())
    dispatch(resetSelectedVault())
    setTimeout(() => handleResetModal(), 5000)
  }

  const modalContextValue: ModalContextType = {
    transactionStatusModal: {
      isOpen: isTransactionStatusModalOpen,
      setIsOpen: setIsTransactionStatusModalOpen,
      step: transactionStatusModalStep,
      setStep: setTransactionStatusModalStep,
      status: transactionStatusModalStatus,
      setStatus: setTransactionStatusModalStatus,
      operation: transactionStatusModalOperation,
      setOperation: setTransactionStatusModalOperation,
      error: transactionStatusModalError,
      setError: setTransactionStatusModalError,
      txHash: txHash,
      setTxHash: setTxHash,
      resetModal: handleResetModal,
      initModal: handleInitModal,
      handleSuccess: handleSuccess,
      handleError: handleError,
    },
    deployVaultModal: {
      isOpen: isDeployVaultModalOpen,
      setIsOpen: setIsDeployVaultModalOpen,
    },
    inspectVaultModal: {
      isOpen: isInspectVaultModalOpen,
      setIsOpen: setIsInspectVaultModalOpen,
    },
    interactWithVaultModal: {
      isOpen: isInteractWithVaultModalOpen,
      setIsOpen: setIsInteractWithVaultModalOpen,
    },
    editVaultModal: {
      isOpen: isEditVaultModalOpen,
      setIsOpen: setIsEditVaultModalOpen,
    },
    rebalanceVaultModal: {
      isOpen: isRebalanceModalOpen,
      setIsOpen: setIsRebalanceModalOpen,
    },
    investStrategiesModal: {
      isOpen: isInvestStrategiesModalOpen,
      setIsOpen: setIsInvestStrategiesModalOpen,
    },
  }

  return (
    <ModalContext.Provider value={modalContextValue}>
      {children}
    </ModalContext.Provider>
  )
}