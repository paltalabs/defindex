import React, { useEffect } from 'react'
import {
  Slider,
  SliderTrack,
  SliderFilledTrack,
  SliderThumb,
  Button,
  Tooltip,
  Grid,
  GridItem,
  Input,
  InputGroup,
  InputRightAddon,
  IconButton
} from '@chakra-ui/react'
import { DeleteIcon } from '@chakra-ui/icons'
import { useAppDispatch, useAppSelector } from '@/store/lib/storeHooks'
import { setStrategyValue, removeStrategy } from '@/store/lib/features/strategiesStore'


function ItemSlider({
  address = 'Address',
  value = 0,
  name = "Soroswap strategy"
}: {
  address: string,
  value: number,
  name?: string,
}) {
  const dispatch = useAppDispatch()
  const [showTooltip, setShowTooltip] = React.useState(false)

  const totalValues = useAppSelector(state => state.strategies.totalValues)
  const [inputValue, setInputValue] = React.useState<number | string>(value)

  const setVal = (val: number) => {
    const total = totalValues! - value + val
    if (total <= 100) {
      setInputValue(val)
      dispatch(setStrategyValue({ address, value: val }))
    } else {
      setMax()
    }
  }

  const setMax = async () => {
    const rest = 100 - totalValues!
    const newVal = value + rest
    setInputValue(newVal)
    dispatch(setStrategyValue({ address, value: newVal }))
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
    dispatch(setStrategyValue({ address, value: 0 }))
    dispatch(removeStrategy({ address: address, value: 0 }))
  }

  useEffect(() => {
    setInputValue(value)
  }, [value])

  return (
    <Grid templateColumns="repeat(12, 1fr)" alignItems={'center'} my={4}>
      <GridItem colSpan={8} display={'flex'} alignItems={'center'}>
        <h3>{name ? name : address}</h3>
        <IconButton
          aria-label='delete__button'
          mx={2}
          onClick={handleDelete}
          icon={<DeleteIcon />}
          variant='outline'
          colorScheme='red'
          size={'xs'}
        />
      </GridItem>
      <GridItem colSpan={1} colStart={12} justifySelf={'end'} alignContent={'end'}>
        <InputGroup>
          <Input
            px={2}
            type='number'
            min={0}
            placeholder={value.toString()}
            onInput={handleValueInput}
            onBlur={handleBlur}
            onKeyDown={handleEnter}
            value={inputValue} />
          <InputRightAddon px={1}>%</InputRightAddon>
        </InputGroup>
      </GridItem>
      <GridItem colSpan={12} mt={4}>
        <Slider
          aria-label='slider-ex-5'
          id='slider'
          defaultValue={value}
          value={value}
          min={0}
          max={100}
          colorScheme='green'
          maxWidth={'100%'}
          onChange={(v) => { setVal(v) }}
          onMouseEnter={() => setShowTooltip(true)}
          onMouseLeave={() => setShowTooltip(false)}
          onChangeEnd={(val) => setVal(val)}>
          <SliderTrack boxShadow={'sm'}>
            <SliderFilledTrack boxShadow={'dark-lg'} />
          </SliderTrack>
          <Tooltip
            hasArrow
            bg='green.500'
            color='white'
            placement='top'
            isOpen={showTooltip}
            label={`${value}%`}
          >
            <SliderThumb />
          </Tooltip>
        </Slider>
      </GridItem>
      <GridItem colSpan={1} colStart={12} mt={4} justifySelf={'end'}>
        <Button
          onClick={() => { setMax() }}
          colorScheme={'green'}
          size={'lg'}
        >
          Set Max
        </Button>
      </GridItem>
    </Grid>
  )
}

export default ItemSlider