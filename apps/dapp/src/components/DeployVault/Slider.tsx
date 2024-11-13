import React, { useEffect } from 'react'
import {
  Grid,
  GridItem,
  Input,
  IconButton,
  HStack,
  Button
} from '@chakra-ui/react'
import { FaRegTrashCan } from "react-icons/fa6";
import { useAppDispatch, useAppSelector } from '@/store/lib/storeHooks'
//import { setStrategyValue, removeStrategy } from '@/store/lib/features/vaultStore'
import { InputGroup } from '../ui/input-group'
import { Slider } from '../ui/slider'


function ItemSlider({
  address = 'Address',
  share = 0,
  name = "Soroswap strategy"
}: {
  address: string,
    share: number,
  name?: string,
}) {
  const dispatch = useAppDispatch()
  const totalShares = useAppSelector(state => state.newVault.totalValues)
  const [inputValue, setInputValue] = React.useState<number | string>(share)
  const setVal = (val: number) => {
    const total = totalShares! - share + val
    if (total <= 100) {
      setInputValue(val)
      // dispatch(setStrategyValue({ address, share: val }))
    } else {
      setMax()
    }
  }

  const setMax = async () => {
    const rest = 100 - totalShares!
    const newVal = share + rest
    setInputValue(newVal)
    //dispatch(setStrategyValue({ address, share: newVal }))
  }

  const handleValueInput = (e: any) => {
    const val = parseInt(e.target.value)
    const startWithZero = e.target.value.startsWith('0')
    if (val <= 100 && startWithZero == true) {
      setInputValue(Math.floor(val / 1))
    } else if (val <= 100 && startWithZero == false) {
      setInputValue(val)
    }
    else if (val > 100) {
      setInputValue(val)
    }
    else if (e.target.value == '') {
      setInputValue('')
    }

  }

  const handleBlur = (e: any) => {
    const val = parseInt(e.target.value)
    if (val > 100 && inputValue == '') {
      setVal(0)
    }
    setVal(val)
  }

  const handleEnter = (e: any) => {
    if (e.key === 'Enter') {
      const val = parseInt(e.target.value)
      if (isNaN(val) && inputValue.toString().length === 0) {
        console.log(true)
        setVal(0)
      }
      setVal(val)
    }
  }

  const handleDelete = () => {
    //dispatch(setStrategyValue({ address, share: 0 }))
    //dispatch(removeStrategy({ address: address, share: 0 }))
  }

  useEffect(() => {
    setInputValue(share)
  }, [share])

  return (
    <>
      <Grid templateColumns="repeat(12, 1fr)" alignItems={'center'} my={4}>
      <GridItem colSpan={8} display={'flex'} alignItems={'center'}>
        <h3>{name ? name : address}</h3>
        <IconButton
          aria-label='delete__button'
          mx={2}
          onClick={handleDelete}
          variant='outline'
          colorScheme='red'
          size={'xs'}
        >
            <FaRegTrashCan />
        </IconButton>
      </GridItem>
      <GridItem colSpan={1} colStart={12} justifySelf={'end'} alignContent={'end'}>
          <HStack>
            <InputGroup endElement={'%'}>
              <Input
                px={2}
                type='number'
                min={0}
                width={'82px'}
                placeholder={share.toString()}
                onInput={handleValueInput}
                onBlur={handleBlur}
                onKeyDown={handleEnter}
                value={inputValue}
              />
            </InputGroup>
          </HStack>
        </GridItem>
        <GridItem colSpan={12} mt={4}>
          <Slider
            min={0}
            max={100}
            defaultValue={[0]}
            maxWidth={'100%'}
            marks={[0, 25, 50, 75, 100]}
            value={[inputValue as number]}
            onValueChange={(val: any) => setVal(val.value[0])}
          />
        </GridItem>
        <GridItem colSpan={1} colStart={12} mt={4} justifySelf={'end'}>
          <Button
            onClick={() => { setMax() }}
            size={'xs'}
          >
            Set Max
          </Button>
        </GridItem>
      </Grid>
    </>
  )
}

export default ItemSlider