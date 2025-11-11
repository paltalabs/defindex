'use client';

import { Prism as SyntaxHighlighter } from 'react-syntax-highlighter';
import { vscDarkPlus } from 'react-syntax-highlighter/dist/esm/styles/prism';

interface CodeBlockProps {
  language: string;
  code: string;
}

/**
 * Client-side code block component with syntax highlighting
 * This component is rendered only on the client to avoid hydration issues
 */
export default function CodeBlock({ language, code }: CodeBlockProps) {
  return (
    <SyntaxHighlighter
      language={language}
      style={vscDarkPlus}
      customStyle={{
        margin: 0,
        padding: '1.5rem',
        background: 'transparent',
        fontSize: '0.875rem',
      }}
      showLineNumbers
      wrapLines
      PreTag="div"
    >
      {code}
    </SyntaxHighlighter>
  );
}
