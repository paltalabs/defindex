import { createSlice, createAsyncThunk } from '@reduxjs/toolkit'
import type { PayloadAction } from '@reduxjs/toolkit'
import type { RootState } from '../store'
import { ChainMetadata } from '@soroban-react/types'
import { SelectedVault, VaultData, WalletState } from '../types'

// Define the initial state using that type
const initialState: WalletState = {
  address: '',
  selectedChain: {
    id: '',
    networkPassphrase: '',
    network: '',
    networkUrl: '',
  },
  vaults: {
    isLoading: true,
    createdVaults: [],
    hasError: false,
    selectedVault: undefined
  }
}


export const walletSlice = createSlice({
  name: 'wallet',
  initialState,
  reducers: {
    setAddress: (state, action: PayloadAction<string>) => {
      state.address = action.payload
    },
    setChain: (state, action: PayloadAction<ChainMetadata>) => {
      state.selectedChain = action.payload
    },
    resetWallet: (state) => {
      state.address = ''
      state.selectedChain = {
        id: '',
        networkPassphrase: '',
        network: '',
        networkUrl: '',
      }
    },
    pushVault: (state, action: PayloadAction<VaultData>) => {
      state.vaults.createdVaults.push(action.payload)
    },
    setIsVaultsLoading: (state, action: PayloadAction<boolean>) => {
      state.vaults.isLoading = action.payload
    },
    setSelectedVault: (state, action: PayloadAction<SelectedVault>) => {
      state.vaults.selectedVault = action.payload
    },
    setVaults: (state, action: PayloadAction<VaultData[]>) => {
      state.vaults.createdVaults = action.payload
    },
    setVaultTVL: (state, action: PayloadAction<{address:string, value: number}>) => {
      state.vaults.createdVaults.forEach(vault => {
        if (vault.address === action.payload.address) {
          vault.TVL = action.payload.value
        }
      })
    },
    resetSelectedVault: (state) => { 
      state.vaults.selectedVault = undefined
    },
    setVaultUserBalance: (state, action: PayloadAction<{address:string, vaule:number}>) => {
      state.vaults.createdVaults.forEach(vault => {
        if (vault.address === action.payload.address) {
          vault.userBalance = action.payload.vaule
        }
      })
    },
    setVaultFeeReceiver: (state, action: PayloadAction<string>) => {
      state.vaults.createdVaults.forEach(vault => {
        if (vault.address === state.vaults.selectedVault?.address) {
          vault.feeReceiver = action.payload
        }
      })
    },
    updateVaultData: (state, action: PayloadAction<Partial<VaultData>>) => {
      state.vaults.createdVaults.forEach(vault => {
        if (vault.address === action.payload.address) {
          Object.assign(vault, action.payload)
        }
      })
    },
    setStrategyTempAmount: (state, action: PayloadAction<{vaultAddress: string, strategyAddress: string, amount: number}>) => {
      state.vaults.selectedVault?.assets.forEach(asset => {
        asset.strategies.forEach(strategy => {
          if (strategy.address === action.payload.strategyAddress) {
            strategy.tempAmount = action.payload.amount
          }
        })
      })
    }
  },
})

export const { 
  setAddress, 
  setChain, 
  resetWallet, 
  pushVault, 
  setIsVaultsLoading, 
  setSelectedVault, 
  setVaults,
  setVaultTVL,
  resetSelectedVault,
  setVaultFeeReceiver,
  setVaultUserBalance,
  updateVaultData,
  setStrategyTempAmount
} = walletSlice.actions

// Other code such as selectors can use the imported `RootState` type
export const selectAddress = (state: RootState) => state.wallet.address
export const selectChainMetadata = (state: RootState) => state.wallet.selectedChain

export default walletSlice.reducer