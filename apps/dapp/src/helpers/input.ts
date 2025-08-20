export const decimalRegex = /^(\d+(\.\d{0,7})?|\.\d{1,7})$/
export const parseNumericInput = (input: string, decimals: number) => {
  const decimalRegex = new RegExp(`^(\\d+)?(\\.\\d{0,${decimals}})?$`);
  if (!decimalRegex.test(input)) return;
  if (input.startsWith('0') && input.length > 1 && !input.includes('.')) {
    return input.slice(1);
  }
  if (input.startsWith('.')) {
    return 0 + input;
  }
  else return input;
}
