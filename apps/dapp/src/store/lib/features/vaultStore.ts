import { createSlice, PayloadAction } from '@reduxjs/toolkit'
import type { RootState } from '../store'
import axios from 'axios'
import { getRemoteConfig } from '@/helpers/getRemoteConfig';


export interface Strategy {
  address: string;
  name?: string;
  value: number;
}

export interface NewVaultState {
  address: string;
  emergencyManager?: string;
  feeReceiver?: string;
  manager?: string;
  name: string;
  strategies: Strategy[];
  totalValues?: number;
}

// Define the initial state using that type
const initialState: NewVaultState = {
  address: "",
  emergencyManager: "",
  feeReceiver: "",
  manager: "",
  name: "",
  strategies: [
    {
      address: "",
      value: 0
    }
  ],
  totalValues: 0,
}

//Filtrar Strategies por network y retornar array de Strategies
export const getDefaultStrategies = async (network: string) => {
  try {
    const remoteStrategies = await getRemoteConfig(network)
    const strategies: Strategy[] = []
    for (let strategy in remoteStrategies.ids) {
      if (strategy.includes('strategy')) {
        const parsedName = strategy.split('_')[0]
        if (!parsedName) continue
        const prettierName = parsedName.charAt(0).toUpperCase() + parsedName.slice(1)
        strategies.push({
          address: remoteStrategies.ids[strategy],
          name: parsedName ? prettierName : '',
          value: 0
        })
      }
    }
    return strategies
  } catch (error) {
    console.error(error)
    return []
  }
}



export const newVaultSlice = createSlice({
  name: 'Strategies',
  initialState,
  reducers: {
    pushStrategy: (state, action: PayloadAction<Strategy>) => {
      state.strategies.push(action.payload)
      state.totalValues = state.strategies.reduce((acc, Strategy) => acc + Strategy.value, 0)
    },
    resetStrategies: (state) => {
      state.strategies = []
      state.name = ""
      state.totalValues = 0
    },
    removeStrategy: (state, action: PayloadAction<Strategy>) => {
      state.strategies = state.strategies.filter(Strategy => Strategy.address !== action.payload.address)
    },
    setStrategyValue: (state, action: PayloadAction<Strategy>) => {
      state.strategies = state.strategies.map(strategy => {
        if (strategy.address === action.payload.address) {
          return {
            ...strategy,
            value: action.payload.value
          }
        }
        return strategy
      })
      state.totalValues = state.strategies.reduce((acc, Strategy) => acc + Strategy.value, 0)
    },
    resetStrategyValue: (state, action: PayloadAction<Strategy>) => {
      state.strategies = state.strategies.map(strategy => {
        if (strategy.address === action.payload.address) {
          return {
            ...strategy,
            value: 0
          }
        }
        return strategy
      })
      state.totalValues = state.strategies.reduce((acc, Strategy) => acc + Strategy.value, 0)
    },
    setName: ((state, action: PayloadAction<string>) => {
      state.name = action.payload;
    }),
    setManager: ((state, action: PayloadAction<string>) => {
      state.manager = action.payload;
    }),
    setEmergencyManager: ((state, action: PayloadAction<string>) => {
      state.emergencyManager = action.payload;
    }),
    setFeeReceiver: ((state, action: PayloadAction<string>) => {
      state.feeReceiver = action.payload;
    }),
  }
})

export const {
  pushStrategy,
  resetStrategies,
  removeStrategy,
  setStrategyValue,
  resetStrategyValue,
  setName,
  setManager,
  setEmergencyManager,
  setFeeReceiver
} = newVaultSlice.actions

// Other code such as selectors can use the imported `RootState` type
export const selectStrategies = (state: RootState) => state.newVault.strategies
export const selectTotalValues = (state: RootState) => state.newVault.totalValues

export default newVaultSlice.reducer