import { getRemoteConfig } from '@/helpers/getRemoteConfig';
import { createSlice, PayloadAction } from '@reduxjs/toolkit';
import type { RootState } from '../store';
import { Asset, NewVaultState, Strategy, VaultData } from '../types';
// Define the initial state using that type
const initialState: NewVaultState = {
  address: "",
  emergencyManager: "",
  rebalanceManager: "",
  feeReceiver: "",
  manager: "",
  name: "",
  symbol: "",
  vaultShare: 0,
  assets: [],
  TVL: 0,
  upgradable: true,
}

//Filtrar Strategies por network y retornar array de Strategies
export const getDefaultStrategies = async (network: string) => {
  try {
    const remoteStrategies: any = await getRemoteConfig(network)
    const strategies: Strategy[] = []
    for (let strategy in remoteStrategies) {
      if (strategy.includes('strategy')) {
        const parsedName = strategy.split('_')[0]
        const parsedSymbol = strategy.split('_')[1]?.length! <= 4 ? strategy.split('_')[1] : ''
        if (!parsedName) continue
        const prettierName = parsedName.charAt(0).toUpperCase() + parsedName.slice(1) + ' ' + (parsedSymbol ? parsedSymbol.toUpperCase() : '')
        strategies.push({
          address: remoteStrategies[strategy],
          name: parsedName ? prettierName : '',
          paused: false,
          tempAmount: 0
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
    setRebalanceManager: ((state, action: PayloadAction<string>) => {
      state.rebalanceManager = action.payload;
    }),
    setFeeReceiver: ((state, action: PayloadAction<string>) => {
      state.feeReceiver = action.payload;
    }),
    setVaultShare: ((state, action: PayloadAction<number>) => {
      state.vaultShare = action.payload;
    }),
    pushAsset: ((state, action: PayloadAction<Asset>) => {
      const assetIndex = state.assets.findIndex(asset => asset.address === action.payload.address);
      if (assetIndex === -1) {
        state.assets.push(action.payload);
      } else if (assetIndex !== -1) {
        action.payload.strategies.forEach(strategy => {
          state.assets[assetIndex]!.strategies.push(strategy);
        });
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
    setAssetAmount: ((state, action: PayloadAction<{address:string, amount:number}>) => {
      const assetIndex = state.assets.findIndex(asset => asset.address === action.payload.address);
      if (assetIndex !== -1) {
        state.assets[assetIndex]!.amount = Number(state.assets[assetIndex]!.amount || 0) + Number(action.payload.amount);
      }
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
      state.address = "";
      state.emergencyManager = "";
      state.feeReceiver = "";
      state.manager = "";
      state.name = "";
      state.symbol = "";
      state.vaultShare = 0;
      state.assets = [];
      state.TVL = 0;
    }),
    removeStrategy: ((state, action: PayloadAction<Strategy>) => {
      state.assets.forEach(asset => {
        const strategy = asset.strategies.find(strategy => strategy.address === action.payload.address);
        if(asset.amount && strategy) {
          asset.amount -= strategy.tempAmount;
        }
        asset.strategies = asset.strategies.filter(strategy => strategy.address !== action.payload.address);
      });
    }),
    setUpgradable: ((state, action: PayloadAction<boolean>) => {
      state.upgradable = action.payload;
    }),
  }
})

export const {
  setName,
  setSymbol,
  setManager,
  setEmergencyManager,
  setRebalanceManager,
  setFeeReceiver,
  setVaultShare,
  pushAsset,
  removeAsset,
  resetAssets,
  openEditVault,
  resetNewVault,
  removeStrategy,
  setAssetAmount,
  setUpgradable,
} = newVaultSlice.actions

// Other code such as selectors can use the imported `RootState` type
export const selectAsset = (state: RootState) => state.newVault.assets
export const selectTotalValues = (state: RootState) => state.newVault.TVL

export default newVaultSlice.reducer