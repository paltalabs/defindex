import { configureStore } from '@reduxjs/toolkit'
import  walletSlice  from './features/walletStore'
import adaptersSlice from './features/adaptersStore'

export const makeStore = () => {
  return configureStore({
    reducer: {
      wallet: walletSlice,
      adapters: adaptersSlice,
    },
  })
}

export type AppStore = ReturnType<typeof makeStore>
export type RootState = ReturnType<AppStore['getState']>
export type AppDispatch = AppStore['dispatch']