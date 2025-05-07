import { Box, Card } from '@chakra-ui/react'
import React from 'react'

function BackgroundCard({
  title,
  titleFontSize = '2xl',
  titleFontWeight = 'bold',
  children,
}: {
  title?: string
  titleFontSize?: string
  titleFontWeight?: string
  children?: React.ReactNode
}) {
  return (
    <Card.Root variant={'elevated'} p={10} borderRadius={'24px'}>
      <Card.Header>
        <Card.Title fontSize={titleFontSize} fontWeight={titleFontWeight}>{title}</Card.Title>
      </Card.Header>
      <Card.Body>
        {children}
      </Card.Body>
    </Card.Root >
  )
}

export default BackgroundCard
