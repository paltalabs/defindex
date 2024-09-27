import { createSlice, createAsyncThunk } from '@reduxjs/toolkit'
import type { PayloadAction } from '@reduxjs/toolkit'
import type { RootState } from '../store'
import { ChainMetadata } from '@soroban-react/types'
import indexes from '@/constants/constants.json'
import { Networks } from '@stellar/stellar-sdk'
// Define a type for the slice state
export interface Index {
  address: string;
  index: string;
  share: number;
}
export interface IndexData {
  address: string;
  balance: number;
  name: string;
  shares: Index[]
}

interface SelectedIndex extends IndexData {
  method: string;
}
export interface WalletState {
  address: string;
  selectedChain: ChainMetadata;
  indexes: {
    isLoading: boolean;
    createdIndexes: IndexData[];
    hasError: boolean;
    selectedIndex: SelectedIndex | undefined;
  }
}


const getDefaultIndexes = async (network: string) => {
  const filteredIndexes = indexes.filter(index => {
    switch (network) {
      case Networks.TESTNET:
        console.log('fetching testnet indexes')
        return index.network === 'testnet'
      case Networks.PUBLIC:
        console.log('fetching public indexes')
        return index.network === 'public'
      default:
        console.log('fetching testnet indexes')
        return index.network === 'testnet'
    }
  })
  if (filteredIndexes.length === 0) {
    return [indexes[0]?.indexes]
  }
  if (!filteredIndexes[0]?.indexes) return

  await new Promise(resolve => setTimeout(resolve, 1500))
  return filteredIndexes[0]?.indexes
}

export const fetchDefaultAddresses = createAsyncThunk(
  'wallet/fetchDefaultIndexes',
  async (network: string) => {
    console.log('fetching default indexes from', network)
    const defaultIndexes = await getDefaultIndexes(network)
    const defaultAdresses = defaultIndexes?.map((index: any) => {
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
  indexes: {
    isLoading: true,
    createdIndexes: [],
    hasError: false,
    selectedIndex: undefined
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
    pushIndex: (state, action: PayloadAction<IndexData>) => {
      state.indexes.createdIndexes.push(action.payload)
    },
    setIsIndexesLoading: (state, action: PayloadAction<boolean>) => {
      state.indexes.isLoading = action.payload
    },
    setSelectedIndex: (state, action: PayloadAction<SelectedIndex>) => {
      state.indexes.selectedIndex = action.payload
    }
  },
  extraReducers(builder) {
    builder.addCase(fetchDefaultAddresses.pending, (state) => {
      state.indexes.isLoading = true
    })
    builder.addCase(fetchDefaultAddresses.fulfilled, (state, action) => {
      state.indexes.isLoading = false
      state.indexes.createdIndexes = action.payload!
    })
    builder.addCase(fetchDefaultAddresses.rejected, (state) => {
      state.indexes.isLoading = false
      state.indexes.hasError = true
    })
  },
})

export const { setAddress, setChain, resetWallet, pushIndex, setIsIndexesLoading, setSelectedIndex } = walletSlice.actions

// Other code such as selectors can use the imported `RootState` type
export const selectAddress = (state: RootState) => state.wallet.address
export const selectChainMetadata = (state: RootState) => state.wallet.selectedChain

export default walletSlice.reducer