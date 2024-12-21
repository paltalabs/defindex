import fs from 'fs';
import path from 'path';

function countLinesInFile(filePath: string): number {
  try {
    const content = fs.readFileSync(filePath, 'utf8');
    const lines = content.split('\n');
    let lineCount = 0;
    let inFunctionParams = false;
    let openParens = 0;

    for (let i = 0; i < lines.length; i++) {
      const trimmedLine = lines[i].trim();
      
      // Skip empty lines and comments
      if (trimmedLine.length === 0 || 
          trimmedLine.startsWith('//') || 
          trimmedLine.startsWith('/*') || 
          trimmedLine.startsWith('*')) {
        continue;
      }

      // Count open and close parentheses
      if (trimmedLine.includes('(')) {
        openParens += trimmedLine.split('(').length - 1;
        inFunctionParams = true;
      }
      if (trimmedLine.includes(')')) {
        openParens -= trimmedLine.split(')').length - 1;
        if (openParens <= 0) {
          inFunctionParams = false;
          // Only count this as one line if we were in function parameters
          lineCount++;
          continue;
        }
      }

      // If we're not in function parameters, count normally
      if (!inFunctionParams) {
        lineCount++;
      }
    }
    
    return lineCount;
  } catch (error) {
    console.error(`Error reading file ${filePath}:`, error);
    return 0;
  }
}

function countLinesInDirectory(dirPath: string, extensions: string[] = ['.rs']): number {
  let totalLines = 0;

  try {
    const files = fs.readdirSync(dirPath);

    for (const file of files) {
      const fullPath = path.join(dirPath, file);
      const stat = fs.statSync(fullPath);

      if (file === 'test' || file.includes('test.rs')) {
        continue;
      }

      if (stat.isDirectory()) {
        totalLines += countLinesInDirectory(fullPath, extensions);
      } else if (extensions.some(ext => file.endsWith(ext))) {
        const lines = countLinesInFile(fullPath);
        console.log(`${fullPath}: ${lines} lines`);
        totalLines += lines;
      }
    }
  } catch (error) {
    console.error(`Error reading directory ${dirPath}:`, error);
  }

  return totalLines;
}

// Count lines for each component
const components = [
  { name: 'Factory', path: 'factory/src' },
  { name: 'Vault', path: 'vault/src' },
  { name: 'Blend Strategy', path: 'strategies/blend/src' }
];

let grandTotal = 0;

for (const component of components) {
  console.log(`\n=== ${component.name} ===`);
  const lines = countLinesInDirectory(component.path);
  console.log(`Total lines in ${component.name}: ${lines}`);
  grandTotal += lines;
}

console.log(`\nTotal lines across all components: ${grandTotal}`); 