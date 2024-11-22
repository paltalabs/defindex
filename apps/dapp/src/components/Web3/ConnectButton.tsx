import React, { useEffect } from "react"
import { Button, Grid, GridItem, Image, ProgressCircleRoot, Text } from '@chakra-ui/react'
import { useSorobanReact } from '@soroban-react/core'
import { useState } from 'react'
import { connectors } from "@/providers/soroban-react-provider"
import { ProgressCircleRing } from "../ui/progress-circle"
import {
  DialogBackdrop,
  DialogBody,
  DialogContent,
  DialogHeader,
  DialogRoot,
  DialogTrigger,
  DialogCloseTrigger,
  DialogTitle
} from '@/components/ui/dialog';

const buildWalletsStatus = () => {
  return connectors.map((w) => ({
    isInstalled: false,
    isLoading: true,
    name: w.id,
    connector: w,
  }));
};

export const ConnectButton = () => {
  const [open, setOpen] = useState<boolean>(false)
  const { address, setActiveConnectorAndConnect, disconnect, connectors } = useSorobanReact()
  const [walletsStatus, setWalletsStatus] = useState<{
    name: string;
    isInstalled: boolean;
    isLoading: boolean;
  }[]>(buildWalletsStatus());

  const handleConnect = (index: number) => {
    if (!connectors) return;
    const selectedConnector = connectors[index];
    if (!selectedConnector) return;
    const isInstalled = walletsStatus.find((w) => w.name === selectedConnector.id)?.isInstalled;
    if (isInstalled && setActiveConnectorAndConnect) {
      setActiveConnectorAndConnect(selectedConnector);
    } else {
      window.open(selectedConnector.downloadUrls?.browserExtension, '_blank');
    }
    setOpen(false);
  }

  const handleDisconnect = () => {
    if (!disconnect) return;
    setOpen(false);
    disconnect();
  }

  useEffect(() => {
    const newWalletsStatus = walletsStatus.map(async (walletStatus) => {
      const contextConnector = connectors.find((c) => c.id === walletStatus.name);

      if (contextConnector) {
        let connected = await contextConnector.isConnected();

        return { name: walletStatus.name, isInstalled: connected, isLoading: false };
      }

      return { ...walletStatus, isLoading: false };
    });

    Promise.all(newWalletsStatus).then((updatedWalletsStatus) => {
      setWalletsStatus(updatedWalletsStatus as any);
    });
  }, []);

  const getWalletStatus = (name: string) => {
    const selectedWallet = walletsStatus.find((w) => {
      if (w.name === name) {
        return w;
      };
    });
    if (selectedWallet?.isLoading) {
      return (
        <ProgressCircleRoot>
          <ProgressCircleRing />
        </ProgressCircleRoot>);
    } else if (!!!(selectedWallet?.isLoading) && selectedWallet?.isInstalled) {
      return (<Text fontWeight={400} fontSize='md' color='green.400'>Connect</Text>);
    } else {
      return (<Text fontWeight={400} fontSize='md' color='yellow.500'>Install</Text>);
    }
  }
  return (
    <>
      <DialogRoot open={open} size={"lg"} onOpenChange={(e) => { setOpen(e.open) }} placement={'center'}>
        <DialogBackdrop />
        <DialogTrigger asChild >
          <Button colorScheme='green' rounded={18} mb={{ base: 4, md: 0 }} mx={4}>
            {address ? 'Disconnect' : 'Connect'}
          </Button>
        </DialogTrigger>
        <DialogContent>
          <DialogCloseTrigger />
          <DialogHeader pb={1}>
            <DialogTitle>
              {!address ? 'Connect wallet' : 'About wallet'}
            </DialogTitle>
          </DialogHeader>


          {!address && (
            <DialogBody mb={4}>
              {connectors.map((connector, index) => (
                <Button
                  key={index}
                  onClick={() => {
                    handleConnect(index)
                  }}
                  w={'100%'}
                  my={2}
                  backgroundColor={'gray.900'}
                  color={'gray.100'}
                >
                  <Grid templateColumns='repeat(20, 1fr)' justifyContent={'space-between'} alignContent={'center'}>
                    <GridItem colStart={1} colEnd={3}>
                      <Image boxSize='20px' src={connector.iconUrl as string} alt={connector.name} />
                    </GridItem>
                    <GridItem>
                      <p>
                        {connector.name}
                      </p>
                    </GridItem>
                    <GridItem colStart={20} colEnd={20}>
                      {getWalletStatus(connector.id)}
                    </GridItem>
                  </Grid>
                </Button>
              ))}
            </DialogBody>
          )}
          {address && (
            <DialogBody>
              <p>Connected with {address}</p>
              <Button onClick={handleDisconnect}>Disconnect</Button>
            </DialogBody>
          )}

        </DialogContent>
      </DialogRoot>
    </>
  )
}

export default ConnectButton
