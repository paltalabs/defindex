import CreateIndex from '@/components/CreateIndex/CreateIndex'
import DepositToIndex from '@/components/DepositToIndex/DepositToIndex'
import { HomePageTitle } from '@/components/home/HomePageTitle'
import { CenterBody } from '@/components/layout/CenterBody'
import { ChainInfo } from '@/components/web3/ChainInfo'
import { ConnectButton } from '@/components/web3/ConnectButton'
import { GreeterContractInteractions } from '@/components/web3/GreeterContractInteractions'
import { useSorobanReact } from '@soroban-react/core'
import type { NextPage } from 'next'
import 'twin.macro'

const HomePage: NextPage = () => {
  // Display `useInkathon` error messages (optional)
  // const { error } = useInkathon()
  // useEffect(() => {
  //   if (!error) return
  //   toast.error(error.message)
  // }, [error])
  const { address } = useSorobanReact();
  return (
    <>
      {/* Top Bar */}
      {/* <HomeTopBar /> */}

      <CenterBody tw="mt-20 mb-10 px-5">
        {/* Title */}
        <HomePageTitle />
        {address == undefined ? <ConnectButton /> :
          <>
            <div tw='mt-10 flex flex-col w-4/6 content-center gap-4'>
              <CreateIndex />
              <DepositToIndex />
            </div>
          </>}



      </CenterBody>
    </>
  )
}

export default HomePage
