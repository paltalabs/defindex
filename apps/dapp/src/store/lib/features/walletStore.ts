import { createSlice, createAsyncThunk } from '@reduxjs/toolkit'
import type { PayloadAction } from '@reduxjs/toolkit'
import type { RootState } from '../store'
import { ChainMetadata } from '@soroban-react/types'
import vaults from '@/constants/constants.json'
import { Networks } from '@stellar/stellar-sdk'
import { SelectedVault, VaultData, WalletState } from '../types'

const getDefaultVaults = async (network: string) => {
  const filteredVaults = vaults.filter(vault => {
    switch (network) {
      case Networks.TESTNET:
        console.log('fetching testnet indexes')
        return vault.network === 'testnet'
      case Networks.PUBLIC:
        console.log('fetching public indexes')
        return vault.network === 'public'
      default:
        console.log('fetching testnet indexes')
        return vault.network === 'testnet'
    }
  })
  if (filteredVaults.length === 0) {
    return [vaults[0]?.vaults]
  }
  if (!filteredVaults[0]?.vaults) return

  await new Promise(resolve => setTimeout(resolve, 1500))
  return filteredVaults[0]?.vaults
}

export const fetchDefaultAddresses = createAsyncThunk(
  'wallet/fetchDefaultVaults',
  async (network: string) => {
    const defaultVaults = await getDefaultVaults(network)
    const defaultAdresses = defaultVaults?.map((index: any) => {
      return index
    })
    return defaultAdresses
  }
)
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
    setVaultTVL: (state, action: PayloadAction<number>) => {
      state.vaults.createdVaults.forEach(vault => {
        if (vault.address === state.vaults.selectedVault?.address) {
          vault.TVL = action.payload
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
  },
  extraReducers(builder) {
    builder.addCase(fetchDefaultAddresses.pending, (state) => {
      state.vaults.isLoading = true
    })
    builder.addCase(fetchDefaultAddresses.fulfilled, (state, action) => {
      state.vaults.isLoading = false
      state.vaults.createdVaults = action.payload!
    })
    builder.addCase(fetchDefaultAddresses.rejected, (state) => {
      state.vaults.isLoading = false
      state.vaults.hasError = true
    })
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
  setVaultUserBalance,
  setVaultFeeReceiver
} = walletSlice.actions

// Other code such as selectors can use the imported `RootState` type
export const selectAddress = (state: RootState) => state.wallet.address
export const selectChainMetadata = (state: RootState) => state.wallet.selectedChain

export default walletSlice.reducer