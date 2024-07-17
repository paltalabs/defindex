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

              <Td sx={{ cursor: 'pointer' }} textAlign={'center'}>
                <Tooltip label={adapter.address}>
                  <p>{shortenAddress(adapter.address)}</p>
                </Tooltip>
              </Td>

              <Td textAlign={'end'}>{adapter.value}%</Td>
            </Tr>
          ))}
        </Tbody>
      </Table>
    </TableContainer>
  )
}