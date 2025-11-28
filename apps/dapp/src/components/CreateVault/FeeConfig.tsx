import { VaultContext } from '@/contexts'
import { isValidAddress } from '@/helpers/address'
import { decimalRegex, parseNumericInput } from '@/helpers/input'
import { useContext, useEffect, useState } from 'react'
import { FormField } from '../ui/CustomInputFields'
import { VaultConfigSection } from './VaultConfigSection'

export function FeeConfig() {
  const vaultContext = useContext(VaultContext);
  const [showWarning, setShowWarning] = useState(false)

  useEffect(() => {
    const handleWarning = () => {
      if (vaultContext && vaultContext.newVault.feePercent > 50) {
        console.log('show warning')
        setShowWarning(true)
      }
      else {
        setShowWarning(false)
      }
    }
    handleWarning()
  }, [vaultContext])

  const handleInput = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (!decimalRegex.test(e.target.value) && e.target.value != '') return
    if (e.target.value == '') {
      vaultContext?.setNewVault({
        ...vaultContext.newVault,
        feePercent: 0,
      })
      return
    }
    if (parseFloat(e.target.value) >= 100) {
      e.target.value = '100'
    }
    vaultContext?.setNewVault({
      ...vaultContext.newVault,
      feePercent: Number(parseNumericInput(e.target.value, 2) || vaultContext.newVault.feePercent),
    })
  }

  const handleFeeReceiver = (e: React.ChangeEvent<HTMLInputElement>) => {
    vaultContext?.setNewVault({
      ...vaultContext.newVault,
      feeReceiver: e.target.value,
    })
  }

  return (
    <VaultConfigSection title="Fee Config">
      <FormField
        label="Fee receiver"
        placeholder="Fee receiver address"
        value={vaultContext!.newVault.feeReceiver}
        onChange={handleFeeReceiver}
        invalid={!isValidAddress(vaultContext!.newVault.feeReceiver!)}
        errorMessage={!vaultContext!.newVault.feeReceiver || !isValidAddress(vaultContext!.newVault.feeReceiver) ? 'Invalid address' : ''}
      />
      <FormField
        label="Fee percentage"
        placeholder="Percentage"
        type='number'
        min={0}
        max={100}
        value={parseNumericInput(vaultContext!.newVault.feePercent.toString(), 2) || 0}
        onChange={handleInput}
        invalid={showWarning}
        errorMessage={'Too high fees could lead to issues'}
      />
    </VaultConfigSection>
  );
}
