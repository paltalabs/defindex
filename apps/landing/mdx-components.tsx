import type { MDXComponents } from 'mdx/types';

/**
 * This file is required for Next.js App Router MDX support
 * It allows you to customize how MDX content is rendered
 */
export function useMDXComponents(components: MDXComponents): MDXComponents {
  return {
    ...components,
  };
}
