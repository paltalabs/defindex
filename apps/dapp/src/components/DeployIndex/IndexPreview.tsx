import {
  Box,
  Table,
  Thead,
  Tbody,
  Tr,
  Th,
  Td,
  TableContainer,
  Tooltip
} from '@chakra-ui/react'
import { shortenAddress } from '@/helpers/shortenAddress'
import { PieChart } from '@mui/x-charts'
import { ChartData } from './ConfirmDelpoyModal'

export const IndexPreview = ({ data }: { data: ChartData[] }) => {
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
    </>
  )
}