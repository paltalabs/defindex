import { Card } from '@chakra-ui/react'
import React from 'react'
import './BackgroundCard.css'

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
    <Card.Root p={10} borderRadius={'24px'} className="background-card">
      <Card.Header>
        <Card.Title truncate fontSize={titleFontSize} fontWeight={titleFontWeight}>{title}</Card.Title>
      </Card.Header>
      <Card.Body>
        {children}
      </Card.Body>
    </Card.Root >
  )
}

export default BackgroundCard
