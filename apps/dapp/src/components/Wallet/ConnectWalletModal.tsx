import {
  Modal,
  ModalOverlay,
  ModalContent,
  ModalHeader,
  ModalBody,
  Button,
  Grid,
  GridItem,
  CircularProgress,
  Text
} from '@chakra-ui/react';
import { Image } from '@chakra-ui/react';
import { useSorobanReact } from '@soroban-react/core';
import { useEffect, useState } from 'react';
import { connectors } from '@/providers/soroban-react-provider';

const buildWalletsStatus = () => {
  return connectors.map((w) => ({
    isInstalled: false,
    isLoading: true,
    name: w.id,
    connector: w,
  }));
};

export const ConnectWalletModal = ({
  isOpen,
  onClose,
}: {
  isOpen: boolean,
  onClose: () => void,
}) => {
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
      onClose();
    } else {
      window.open(selectedConnector.downloadUrls?.browserExtension, '_blank');
    }
  }

  const handleDisconnect = () => {
    if (!disconnect) return;
    disconnect();
    onClose();
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
      return (<CircularProgress isIndeterminate size='8px' />);
    } else if (!!!(selectedWallet?.isLoading) && selectedWallet?.isInstalled) {
      return (<Text fontWeight={400} fontSize='md' color='green.400'>Connect</Text>);
    } else {
      return (<Text fontWeight={400} fontSize='md' color='yellow.500'>Install</Text>);
    }
  }
  return (
    <>
      <Modal isOpen={isOpen} onClose={onClose} isCentered>
        <ModalOverlay />
        <ModalContent>
          <ModalHeader pb={1}>{!address ? 'Connect wallet' : 'About wallet'}</ModalHeader>
          {!address && (
            <ModalBody mb={4}>
              {connectors.map((connector, index) => (
                <Button
                  key={index}
                  onClick={() => {
                    handleConnect(index)
                  }}
                  w={'100%'}
                  my={2}
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
            </ModalBody>
          )}
          {address && (
            <ModalBody>
              <p>Connected with {address}</p>
              <Button onClick={handleDisconnect}>Disconnect</Button>
            </ModalBody>
          )}
        </ModalContent>
      </Modal>
    </>
  )
}

