import { createSlice, PayloadAction } from '@reduxjs/toolkit'
import type { RootState } from '../store'
import axios from 'axios'


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
export const getDefaultStrategies = async (network: string) => {
  try {
    const {data: remoteStrategies} = await axios.get(`https://raw.githubusercontent.com/paltalabs/defindex/refs/heads/main/public/${network}.contracts.json`)
    const strategies: Strategy[] = []
    for(let strategy in remoteStrategies.ids){
      console.log(strategy)
      if(strategy.includes('strategy')){
        const parsedName = strategy.split('_')[0]
        if(!parsedName) continue
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