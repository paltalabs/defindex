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
            {data.map((adapter: ChartData, index: number) => (
              <Tr key={index} sx={{ cursor: 'default' }}>
                <Td>{adapter.label}</Td>
                <Td sx={{ cursor: 'pointer' }} textAlign={'center'}>
                  <Tooltip label={adapter.address}>
                    {adapter.address ? shortenAddress(adapter.address) : '-'}
                  </Tooltip>
                </Td>
                <Td textAlign={'end'}>{adapter.value}%</Td>
              </Tr>
            ))}
          </Tbody>
        </Table>
      </TableContainer>
    </>
  )
}