import { createSlice, PayloadAction } from '@reduxjs/toolkit'
import { getRemoteConfig } from '@/helpers/getRemoteConfig';
import type { RootState } from '../store'
import { Asset, NewVaultState, Strategy, VaultData } from '../types';
// Define the initial state using that type
const initialState: NewVaultState = {
  address: "",
  emergencyManager: "",
  feeReceiver: "",
  manager: "",
  name: "",
  symbol: "",
  vaultShare: 0,
  assets: [],
  amounts: [],
  TVL: 0,
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
          paused: false,
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
    pushAsset: ((state, action: PayloadAction<Asset>) => {
      const alreadyExists = state.assets.find(asset => asset.address === action.payload.address && asset.strategies.length === action.payload.strategies.length)
      if(alreadyExists) {
        console.warn('Asset already exists')
        return;
      } else {
        state.assets.push(action.payload);
      }
    }),
    resetAssets: ((state) => {
      state.assets = [];
    }),
    removeAsset: ((state, action: PayloadAction<string>) => {
      state.assets = state.assets.filter(asset => asset.address !== action.payload);
    }),
    pushStrategy: ((state, action: PayloadAction<Strategy>) => {
      state.assets.find(asset => asset.address === action.payload.address)?.strategies.push(action.payload);
    }),
    pushAmount: ((state, action: PayloadAction<number>) => {
      state.amounts?.push(action.payload);
    }),
    removeAmountByIndex: ((state, action: PayloadAction<number>) => {
      state.amounts?.splice(action.payload, 1);
    }),
    openEditVault: ((state, action: PayloadAction<VaultData>) => {
      state.name = action.payload.name;
      state.manager = action.payload.manager;
      state.emergencyManager = action.payload.emergencyManager;
      state.feeReceiver = action.payload.feeReceiver;
      state.assets = action.payload.assets;
      state.TVL = action.payload.TVL;
    }),
    resetNewVault: ((state) => {
      state = initialState;
    }),
  }
})

export const {
  setName,
  setSymbol,
  setManager,
  setEmergencyManager,
  setFeeReceiver,
  setVaultShare,
  pushAsset,
  pushAmount,
  removeAsset,
  removeAmountByIndex,
  resetAssets,
  openEditVault,
  resetNewVault
} = newVaultSlice.actions

// Other code such as selectors can use the imported `RootState` type
export const selectAsset = (state: RootState) => state.newVault.assets
export const selectTotalValues = (state: RootState) => state.newVault.TVL

export default newVaultSlice.reducer