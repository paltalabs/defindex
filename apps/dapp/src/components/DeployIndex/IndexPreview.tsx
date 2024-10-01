import {
  Box,
  Table,
  Thead,
  Tbody,
  Tr,
  Th,
  Td,
  TableContainer,
  Tooltip,
  Text,
  Grid,
  GridItem,
  Input,
} from '@chakra-ui/react'
import { shortenAddress } from '@/helpers/shortenAddress'
import { PieChart } from '@mui/x-charts'
import { ChartData } from './ConfirmDelpoyModal'
import { setEmergencyManager, setFeeReceiver, setManager } from '@/store/lib/features/vaultStore'
import { useAppDispatch, useAppSelector } from '@/store/lib/storeHooks'


export const IndexPreview = ({ data }: { data: ChartData[] }) => {
  const dispatch = useAppDispatch()
  const handleManagerChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    dispatch(setManager(event.target.value))
    // Your logic here using the address
  };

  const handleEmergencyManagerChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    dispatch(setEmergencyManager(event.target.value))
    // Your logic here using the address
  };
  const handleFeeReceiverChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    dispatch(setFeeReceiver(event.target.value))
    // Your logic here using the address
  };

  return (
    <>
      <Box display='flex' my={4}>
        <PieChart
          series={[
            {
              data: data,
            },
          ]}
          width={500}
          height={200}
        />
      </Box>
      <Text fontSize='lg' fontWeight='bold' mb={2}>
        Strategies
      </Text>
      <TableContainer>
        <Table variant={'simple'}>
          <Thead>
            <Tr>
              <Th>Name</Th>
              <Th textAlign={'center'}>Address</Th>
              <Th textAlign={'end'}>Percentage</Th>
            </Tr>
          </Thead>
          <Tbody>
            {data.map((strategy: ChartData, index: number) => (
              <Tr key={index} sx={{ cursor: 'default' }}>
                <Td>{strategy.label}</Td>
                <Td sx={{ cursor: 'pointer' }} textAlign={'center'}>
                  <Tooltip label={strategy.address}>
                    {strategy.address ? shortenAddress(strategy.address) : '-'}
                  </Tooltip>
                </Td>
                <Td textAlign={'end'}>{strategy.value}%</Td>
              </Tr>
            ))}
          </Tbody>
        </Table>
      </TableContainer>
      <Box height="20px" />
      <Grid
        templateColumns={'repeat(8, 2fr)'}
        templateRows={'repeat(5, 1fr)'}
        alignSelf={'end'}
        alignContent={'center'}
        mb={4}
      >
        <GridItem colStart={2} colSpan={3} rowStart={1}>
          <Text mt={4}>Manager</Text>
        </GridItem>
        <GridItem colStart={5} colSpan={3} rowStart={1}>
          <Input onChange={handleManagerChange} placeholder='GAFS3TLVM...'></Input>
        </GridItem>
        <GridItem colStart={2} colSpan={3} rowStart={3}>
          <Text mt={4}>Emergency Manager</Text>
        </GridItem>
        <GridItem colStart={5} colSpan={3} rowStart={3}>
          <Input onChange={handleEmergencyManagerChange} placeholder='GAFS3TLVM...'></Input>
        </GridItem>
        <GridItem colStart={2} colSpan={3} rowStart={5}>
          <Text mt={4}>Fee Receiver</Text>
        </GridItem>
        <GridItem colStart={5} colSpan={3} rowStart={5}>
          <Input onChange={handleFeeReceiverChange} placeholder='GAFS3TLVM...'></Input>
        </GridItem>
      </Grid>
    </>
  )
}
