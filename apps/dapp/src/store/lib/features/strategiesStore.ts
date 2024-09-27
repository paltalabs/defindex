import { createSlice, PayloadAction } from '@reduxjs/toolkit'
import type { RootState } from '../store'
import strategies from '@/constants/constants.json'
import { Networks } from '@stellar/stellar-sdk'

export interface Strategy {
  address: string;
  value: number;
  name?: string;
}

export interface StrategiesState {
  strategies: Strategy[];
  strategyName: string;
  totalValues?: number;
}


// Define the initial state using that type
const initialState: StrategiesState = {
  strategies: [
    {
      address: "",
      value: 0
    }
  ],
  strategyName: "",
  totalValues: 0
}

//Filtrar Strategies por network y retornar array de Strategies
export const getDefaultStrategies = (network: string) => {
  const filteredStrategies = strategies.filter(strategy => {
    switch (network) {
      case Networks.TESTNET:
        return strategy.network === 'testnet'
      case Networks.PUBLIC:
        return strategy.network === 'public'
      default:
        return strategy.network === 'testnet'
    }
  })
  if (filteredStrategies.length === 0) {
    return [strategies[0]]
  }
  return filteredStrategies
}



export const strategiesSlice = createSlice({
  name: 'Strategies',
  initialState,
  reducers: {
    pushStrategy: (state, action: PayloadAction<Strategy>) => {
      state.strategies.push(action.payload)
      state.totalValues = state.strategies.reduce((acc, Strategy) => acc + Strategy.value, 0)
    },
    resetStrategies: (state) => {
      state.strategies = []
      state.strategyName = ""
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
    setStrategyName: ((state, action: PayloadAction<string>) => {
      state.strategyName = action.payload;
    })
  }
})

export const {
  pushStrategy,
  resetStrategies,
  removeStrategy,
  setStrategyValue,
  resetStrategyValue,
  setStrategyName } = strategiesSlice.actions

// Other code such as selectors can use the imported `RootState` type
export const selectStrategies = (state: RootState) => state.strategies.strategies
export const selectTotalValues = (state: RootState) => state.strategies.totalValues

export default strategiesSlice.reducer