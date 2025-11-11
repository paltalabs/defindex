import type { MDXComponents } from 'mdx/types';
import Image from 'next/image';
import Link from 'next/link';
import GradientText from '@/components/common/GradientText';
import dynamic from 'next/dynamic';

// Import CodeBlock as client-side only to avoid hydration issues
const CodeBlock = dynamic(() => import('@/components/blog/CodeBlock'), {
  ssr: false,
  loading: () => (
    <div className="relative my-6 rounded-xl overflow-hidden bg-gradient-to-br from-cyan-900 to-cyan-950 border border-cyan-800/30 p-6">
      <div className="animate-pulse bg-cyan-800/30 h-24 rounded" />
    </div>
  ),
});

/**
 * Custom heading components using GradientText from the design system
 */
const CustomH1 = ({ children, ...props }: React.HTMLAttributes<HTMLHeadingElement>) => (
  <GradientText
    as="h1"
    variant="primary"
    className="text-lg md:text-xl lg:text-xl mt-8 mb-4 font-familjen-grotesk font-bold"
    {...props}
  >
    {children}
  </GradientText>
);

const CustomH2 = ({ children, ...props }: React.HTMLAttributes<HTMLHeadingElement>) => (
  <GradientText
    as="h2"
    variant="green"
    className="text-lg md:text-xl lg:text-xl mt-6 mb-3 font-familjen-grotesk font-bold"
    {...props}
  >
    {children}
  </GradientText>
);

const CustomH3 = ({ children, ...props }: React.HTMLAttributes<HTMLHeadingElement>) => (
  <GradientText
    as="h3"
    variant="purple"
    className="text-base md:text-lg lg:text-lg mt-4 mb-2 font-familjen-grotesk font-bold"
    {...props}
  >
    {children}
  </GradientText>
);

const CustomH4 = (props: React.HTMLAttributes<HTMLHeadingElement>) => (
  <h4
    className="text-base md:text-base lg:text-base mt-4 mb-2 font-familjen-grotesk font-bold text-lime-200"
    {...props}
  />
);

const CustomH5 = (props: React.HTMLAttributes<HTMLHeadingElement>) => (
  <h5
    className="text-sm md:text-base lg:text-base mt-3 mb-2 font-familjen-grotesk font-bold text-lime-200"
    {...props}
  />
);

const CustomH6 = (props: React.HTMLAttributes<HTMLHeadingElement>) => (
  <h6
    className="text-sm md:text-sm lg:text-sm mt-2 mb-1 font-familjen-grotesk font-bold text-lime-200"
    {...props}
  />
);

/**
 * Custom paragraph component
 */
const CustomParagraph = (props: React.HTMLAttributes<HTMLParagraphElement>) => (
  <p className="font-inter text-cyan-100 leading-relaxed mb-4 text-sm md:text-base" {...props} />
);

/**
 * Custom code block component with syntax highlighting
 */
interface CodeProps {
  inline?: boolean;
  className?: string;
  children?: React.ReactNode;
}

// Helper function to extract text from React children
const getTextFromChildren = (children: React.ReactNode): string => {
  if (typeof children === 'string') {
    return children;
  }

  if (typeof children === 'number') {
    return String(children);
  }

  if (Array.isArray(children)) {
    return children.map(getTextFromChildren).join('');
  }

  if (children && typeof children === 'object' && 'props' in children) {
    const element = children as { props: { children?: React.ReactNode } };
    return getTextFromChildren(element.props.children);
  }

  return '';
};

const CustomCode = ({ inline, className, children, ...props }: CodeProps) => {
  // Extract language from className (format: language-javascript)
  const match = /language-(\w+)/.exec(className || '');
  const lang = match ? match[1] : '';

  // Inline code
  if (inline) {
    return (
      <code
        className="px-2 py-1 rounded bg-cyan-900/50 text-purple border border-cyan-800/30 font-mono text-sm"
        {...props}
      >
        {children}
      </code>
    );
  }

  // Extract text content from children
  const codeContent = getTextFromChildren(children);

  // Block code with syntax highlighting
  return (
    <div className="relative my-6 rounded-xl overflow-hidden bg-gradient-to-br from-cyan-900 to-cyan-950 border border-cyan-800/30 not-prose">
      <CodeBlock language={lang} code={codeContent.replace(/\n$/, '')} />
    </div>
  );
};

/**
 * Custom image component using Next.js Image
 */
interface ImageProps extends React.ImgHTMLAttributes<HTMLImageElement> {
  src?: string;
  alt?: string;
}

const CustomImage = (props: ImageProps) => {
  const { src = '', alt = '' } = props;

  return (
    <div className="relative w-full h-auto my-8 rounded-xl overflow-hidden border border-cyan-800/30">
      <Image
        src={src}
        alt={alt}
        width={800}
        height={450}
        className="w-full h-auto"
      />
    </div>
  );
};

/**
 * Custom link component
 */
const CustomLink = (props: React.AnchorHTMLAttributes<HTMLAnchorElement>) => {
  const href = props.href || '';
  const isExternal = href.startsWith('http');

  return (
    <Link
      href={href}
      className="text-lime-200 underline hover:text-lime-300 transition-colors"
      target={isExternal ? '_blank' : undefined}
      rel={isExternal ? 'noopener noreferrer' : undefined}
      {...props}
    />
  );
};

/**
 * Custom blockquote component
 */
const CustomBlockquote = (props: React.BlockquoteHTMLAttributes<HTMLQuoteElement>) => (
  <blockquote
    className="border-l-4 border-lime-200 pl-4 my-6 italic text-cyan-200 bg-cyan-950/30 py-3 rounded-r-lg"
    {...props}
  />
);

/**
 * Custom list components
 */
const CustomUl = (props: React.HTMLAttributes<HTMLUListElement>) => (
  <ul className="list-disc list-inside my-4 space-y-2 text-cyan-100 font-inter" {...props} />
);

const CustomOl = (props: React.OlHTMLAttributes<HTMLOListElement>) => (
  <ol className="list-decimal list-inside my-4 space-y-2 text-cyan-100 font-inter" {...props} />
);

const CustomLi = (props: React.LiHTMLAttributes<HTMLLIElement>) => (
  <li className="text-cyan-100 leading-relaxed" {...props} />
);

/**
 * Custom table components
 */
const CustomTable = (props: React.TableHTMLAttributes<HTMLTableElement>) => (
  <div className="overflow-x-auto my-6">
    <table
      className="min-w-full border border-cyan-800/30 rounded-lg overflow-hidden"
      {...props}
    />
  </div>
);

const CustomThead = (props: React.HTMLAttributes<HTMLTableSectionElement>) => (
  <thead className="bg-cyan-900/50" {...props} />
);

const CustomTh = (props: React.ThHTMLAttributes<HTMLTableCellElement>) => (
  <th
    className="px-4 py-3 text-left text-lime-200 font-manrope font-bold border-b border-cyan-800/30"
    {...props}
  />
);

const CustomTd = (props: React.TdHTMLAttributes<HTMLTableCellElement>) => (
  <td
    className="px-4 py-3 text-cyan-100 font-inter border-b border-cyan-800/30"
    {...props}
  />
);

/**
 * Custom horizontal rule
 */
const CustomHr = (props: React.HTMLAttributes<HTMLHRElement>) => (
  <hr className="my-8 border-t border-cyan-800/30" {...props} />
);

/**
 * Custom strong (bold) text
 */
const CustomStrong = (props: React.HTMLAttributes<HTMLElement>) => (
  <strong className="font-bold text-lime-200" {...props} />
);

/**
 * Custom emphasis (italic) text
 */
const CustomEm = (props: React.HTMLAttributes<HTMLElement>) => (
  <em className="italic text-cyan-200" {...props} />
);

/**
 * MDX components function for @next/mdx
 * This function is called by @next/mdx to get custom component mappings
 */
export function useMDXComponents(components: MDXComponents): MDXComponents {
  return {
    h1: CustomH1,
    h2: CustomH2,
    h3: CustomH3,
    h4: CustomH4,
    h5: CustomH5,
    h6: CustomH6,
    p: CustomParagraph,
    code: CustomCode,
    img: CustomImage,
    a: CustomLink,
    blockquote: CustomBlockquote,
    ul: CustomUl,
    ol: CustomOl,
    li: CustomLi,
    table: CustomTable,
    thead: CustomThead,
    th: CustomTh,
    td: CustomTd,
    hr: CustomHr,
    strong: CustomStrong,
    em: CustomEm,
    ...components,
  };
}
