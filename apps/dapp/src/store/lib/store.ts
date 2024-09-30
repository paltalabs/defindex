import { configureStore } from '@reduxjs/toolkit'
import walletSlice from './features/walletStore'
import strategiesSlice from './features/strategiesStore'
import type { WalletState } from './features/walletStore'
import type { StrategiesState } from './features/strategiesStore'


export const makeStore = () => {
  return configureStore({
    reducer: {
      wallet: walletSlice,
      strategies: strategiesSlice
    },
  })
}

export type AppStore = ReturnType<typeof makeStore>
export type RootState = ReturnType<AppStore['getState']> & {
  wallet: WalletState,
  strategies: StrategiesState
}
export type AppDispatch = AppStore['dispatch']