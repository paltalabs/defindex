"use client"
import { useState } from "react"
import { Asset, AssetContext, AssetContextType } from "@/contexts"
import useMounted from "@/hooks/useMounted"


export const StrategiesProvider = ({ children }: { children: React.ReactNode }) => {
  const [assets, setAssets] = useState<Asset[]>([]);
  const isMounted = useMounted();

  const AssetContextValue: AssetContextType = {
    assets,
    setAssets
  }



  if (!isMounted) return null;
  return (
    <AssetContext.Provider value={AssetContextValue}>
      {children}
    </AssetContext.Provider>
  )
}