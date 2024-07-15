import {
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

export const IndexPreview = ({ data }: { data: any }) => {
  return (
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
          {data.map((adapter: any, index: number) => (
            <Tr key={index} sx={{ cursor: 'default' }}>
              <Td>{adapter.name}</Td>
              <Tooltip label={adapter.address}>
                <Td sx={{ cursor: 'pointer' }} textAlign={'center'}>{shortenAddress(adapter.address)}</Td>
              </Tooltip>
              <Td textAlign={'end'}>{adapter.value}%</Td>
            </Tr>
          ))}
        </Tbody>
      </Table>
    </TableContainer>
  )
}