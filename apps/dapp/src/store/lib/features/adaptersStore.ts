import { createSlice, PayloadAction } from '@reduxjs/toolkit'
import type { RootState } from '../store'
import adapters from '@/constants/constants.json'
import { Networks } from '@stellar/stellar-sdk'

export interface Adapter {
  address: string;
  value: number;
  name?: string;
}

interface AdaptersState {
  adapters: Adapter[];
  totalValues?: number;
}


// Define the initial state using that type
const initialState: AdaptersState = {
  adapters: [
    {
      address: "CDYZTM26XJ7FHPFIQPS2CDAPXHIWZKW36QFPXKAZQ7TSKXS6TUBLUXEM",
      value: 82
    }
  ],
  totalValues: 82
}

//Filtrar adapters por network y retornar array de adapters
export const getDefaultAdapters = (network: string) => {
  const filteredAdapters = adapters.filter(adapter => {
    switch (network) {
      case Networks.TESTNET:
        return adapter.network === 'testnet'
      case Networks.PUBLIC:
        return adapter.network === 'public'
      default:
        return adapter.network === 'testnet'
    }
  })
  if(filteredAdapters.length === 0){
    return [adapters[0]]
  }
  return filteredAdapters
}



export const adaptersSlice = createSlice({
  name: 'Adapters',
  initialState,
  reducers: {
    pushAdapter: (state, action: PayloadAction<Adapter>) => {
      state.adapters.push(action.payload)
      state.totalValues = state.adapters.reduce((acc, adapter) => acc + adapter.value, 0)
    },
    resetAdapters: (state) => {
      state.adapters = []
      state.totalValues = 0
    },
    removeAdapter: (state, action: PayloadAction<Adapter>) => {
      state.adapters = state.adapters.filter(adapter => adapter.address !== action.payload.address)
    },
    setAdapterValue: (state, action: PayloadAction<Adapter>) => {
      state.adapters = state.adapters.map(adapter => {
        if (adapter.address === action.payload.address) {
          return {
            ...adapter,
            value: action.payload.value
          }
        }
        return adapter
      })
      state.totalValues = state.adapters.reduce((acc, adapter) => acc + adapter.value, 0)
    },
    resetAdapterValue: (state, action: PayloadAction<Adapter>) => {
      state.adapters = state.adapters.map(adapter => {
        if (adapter.address === action.payload.address) {
          return {
            ...adapter,
            value: 0
          }
        }
        return adapter
      })
      state.totalValues = state.adapters.reduce((acc, adapter) => acc + adapter.value, 0)
    },
  }
})

export const { pushAdapter, resetAdapters, removeAdapter, setAdapterValue, resetAdapterValue } = adaptersSlice.actions

// Other code such as selectors can use the imported `RootState` type
export const selectAdapters = (state: RootState) => state.adapters.adapters
export const selectTotalValues = (state: RootState) => state.adapters.totalValues

export default adaptersSlice.reducer