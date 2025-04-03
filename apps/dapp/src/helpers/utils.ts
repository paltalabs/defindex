export const parseCamelCase = (str: string) => {
  return str
    .replace(/\s(.)/g, (match) => match[1]!.toUpperCase())
    .replace(/\s+/g, '')
    .replace(/^(.)/, (match) => match.toLowerCase())
}
export const parsePascalCase = (str: string) => {
  return str
    .replace(/\s+(.)/g, (match, group1) => group1.toUpperCase())
    .replace(/^(.)/, (match) => match.toUpperCase());
}