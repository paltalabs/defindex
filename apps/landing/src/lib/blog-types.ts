import { z } from 'zod';

/**
 * Categories available for blog posts
 */
export const BLOG_CATEGORIES = ['Tutorial', 'News', 'DeFi', 'Technical', 'Updates'] as const;

/**
 * Zod schema for validating blog post frontmatter
 * Ensures all required fields are present and properly formatted
 */
export const PostFrontmatterSchema = z.object({
  title: z.string().min(1, 'Title is required').max(100, 'Title must be less than 100 characters'),
  slug: z.string().regex(/^[a-z0-9-]+$/, 'Slug must contain only lowercase letters, numbers, and hyphens'),
  date: z.string().datetime('Date must be in ISO 8601 format'),
  lastModified: z.string().datetime('Last modified must be in ISO 8601 format').optional(),
  excerpt: z.string().min(50, 'Excerpt must be at least 50 characters').max(300, 'Excerpt must be less than 300 characters'),
  author: z.object({
    name: z.string().min(1, 'Author name is required'),
    avatar: z.string().optional(),
  }),
  category: z.enum(BLOG_CATEGORIES),
  tags: z.array(z.string()).min(1, 'At least one tag is required').max(5, 'Maximum 5 tags allowed'),
  coverImage: z.string().min(1, 'Cover image is required'),
  coverImageAlt: z.string().min(1, 'Cover image alt text is required'),
  published: z.boolean().default(false),
  featured: z.boolean().default(false),
  seoTitle: z.string().max(60, 'SEO title must be less than 60 characters').optional(),
  seoDescription: z.string().max(160, 'SEO description must be less than 160 characters').optional(),
});

/**
 * TypeScript type inferred from the Zod schema
 */
export type PostFrontmatter = z.infer<typeof PostFrontmatterSchema>;

/**
 * Complete blog post type including content and metadata
 */
export interface BlogPost {
  slug: string;
  frontmatter: PostFrontmatter;
  content: string;
  readingTime: string;
  wordCount: number;
}

/**
 * Simplified post type for listings (without full content)
 */
export interface BlogPostPreview {
  slug: string;
  title: string;
  excerpt: string;
  date: string;
  category: typeof BLOG_CATEGORIES[number];
  tags: string[];
  coverImage: string;
  coverImageAlt: string;
  author: {
    name: string;
    avatar?: string;
  };
  readingTime: string;
  featured: boolean;
}

/**
 * Category with post count
 */
export interface CategoryWithCount {
  name: typeof BLOG_CATEGORIES[number];
  count: number;
  slug: string;
}

/**
 * Tag with post count
 */
export interface TagWithCount {
  name: string;
  count: number;
  slug: string;
}
