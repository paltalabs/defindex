import type { BlogPost, BlogPostPreview } from './blog-types';

/**
 * Client-side utility functions for blog
 * These functions don't use Node.js modules and can be used in client components
 */

/**
 * Search posts by query (title, excerpt, tags)
 * This is a client-side search utility for filtering posts
 */
export function searchPosts(
  posts: BlogPost[] | BlogPostPreview[],
  query: string
): (BlogPost | BlogPostPreview)[] {
  if (!query.trim()) {
    return posts;
  }

  const lowerQuery = query.toLowerCase();

  return posts.filter((post) => {
    const title = 'title' in post ? post.title : post.frontmatter.title;
    const excerpt = 'excerpt' in post ? post.excerpt : post.frontmatter.excerpt;
    const tags = 'tags' in post ? post.tags : post.frontmatter.tags;

    return (
      title.toLowerCase().includes(lowerQuery) ||
      excerpt.toLowerCase().includes(lowerQuery) ||
      tags.some((tag) => tag.toLowerCase().includes(lowerQuery))
    );
  });
}

/**
 * Format date for display
 */
export function formatDate(dateString: string): string {
  const date = new Date(dateString);
  return date.toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
  });
}

/**
 * Get post URL
 */
export function getPostUrl(slug: string): string {
  return `/blog/${slug}`;
}

/**
 * Get category URL
 */
export function getCategoryUrl(category: string): string {
  const slug = category.toLowerCase().replace(/\s+/g, '-');
  return `/blog/category/${slug}`;
}

/**
 * Get tag URL
 */
export function getTagUrl(tag: string): string {
  const slug = tag.toLowerCase().replace(/\s+/g, '-');
  return `/blog/tag/${slug}`;
}
