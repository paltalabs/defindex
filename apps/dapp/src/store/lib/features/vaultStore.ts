import { createSlice, PayloadAction } from '@reduxjs/toolkit'
import type { RootState } from '../store'
import { getRemoteConfig } from '@/helpers/getRemoteConfig';
import { Strategy } from './walletStore';


export interface NewVaultState {
  address: string;
  emergencyManager: string;
  feeReceiver: string;
  manager: string;
  vaultShare: number;
  name: string;
  symbol: string;
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
  symbol: "",
  vaultShare: 0,
  strategies: [
    {
      address: "",
      index: "",
      name: "",
      share: 0
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
          index: strategies.length.toString(),
          name: parsedName ? prettierName : '',
          share: 0,
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
      state.totalValues = state.strategies.reduce((acc, Strategy) => acc + Strategy.share, 0)
    },
    resetStrategies: (state) => {
      state.strategies = []
      state.name = ""
      state.totalValues = 0
    },
    removeStrategy: (state, action: PayloadAction<Partial<Strategy>>) => {
      state.strategies = state.strategies.filter(Strategy => Strategy.address !== action.payload.address)
    },
    setStrategyValue: (state, action: PayloadAction<Partial<Strategy>>) => {
      state.strategies = state.strategies.map(strategy => {
        if (strategy.address === action.payload.address) {
          return {
            ...strategy,
            share: action.payload.share!
          }
        }
        return strategy
      })
      state.totalValues = state.strategies.reduce((acc, Strategy) => acc + Strategy.share, 0)
    },
    resetStrategyValue: (state, action: PayloadAction<Strategy>) => {
      state.strategies = state.strategies.map(strategy => {
        if (strategy.address === action.payload.address) {
          return {
            ...strategy,
            share: 0
          }
        }
        return strategy
      })
      state.totalValues = state.strategies.reduce((acc, Strategy) => acc + Strategy.share, 0)
    },
    setName: ((state, action: PayloadAction<string>) => {
      state.name = action.payload;
    }),
    setSymbol: ((state, action: PayloadAction<string>) => {
      state.symbol = action.payload;
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
    setVaultShare: ((state, action: PayloadAction<number>) => {
      state.vaultShare = action.payload;
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
  setSymbol,
  setManager,
  setEmergencyManager,
  setFeeReceiver,
  setVaultShare
} = newVaultSlice.actions

// Other code such as selectors can use the imported `RootState` type
export const selectStrategies = (state: RootState) => state.newVault.strategies
export const selectTotalValues = (state: RootState) => state.newVault.totalValues

export default newVaultSlice.reducer