import { configureStore } from '@reduxjs/toolkit'
import walletSlice from './features/walletStore'
import newVaultSlice from './features/vaultStore'
import type { WalletState } from './features/walletStore'
import type { NewVaultState } from './features/vaultStore'


export const makeStore = () => {
  return configureStore({
    reducer: {
      wallet: walletSlice,
      newVault: newVaultSlice
    },
  })
}

export type AppStore = ReturnType<typeof makeStore>
export type RootState = ReturnType<AppStore['getState']> & {
  wallet: WalletState,
  newVault: NewVaultState
}
export type AppDispatch = AppStore['dispatch']