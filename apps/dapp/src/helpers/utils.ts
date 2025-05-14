export const parseCamelCase = (str: string) => {
  return str
    .replace(/[_\s](.)/g, (match, group1) => group1.toUpperCase())
    .replace(/[_\s]+/g, '')
    .replace(/^(.)/, (match) => match.toLowerCase());
}
export const parsePascalCase = (str: string) => {
  return str
    .replace(/[_\s]+(.)/g, (match, group1) => group1.toUpperCase())
    .replace(/^(.)/, (match) => match.toUpperCase());
}