import fs from 'fs';
import path from 'path';
import matter from 'gray-matter';
import readingTime from 'reading-time';
import {
  PostFrontmatterSchema,
  type BlogPost,
  type BlogPostPreview,
  type CategoryWithCount,
  type TagWithCount,
  BLOG_CATEGORIES
} from './blog-types';

const BLOG_DIR = path.join(process.cwd(), 'content/blog');

/**
 * Check if blog directory exists
 */
function ensureBlogDirExists(): boolean {
  return fs.existsSync(BLOG_DIR);
}

/**
 * Get all published blog posts sorted by date (newest first)
 */
export async function getAllPosts(): Promise<BlogPost[]> {
  if (!ensureBlogDirExists()) {
    console.warn('Blog directory does not exist:', BLOG_DIR);
    return [];
  }

  const files = fs.readdirSync(BLOG_DIR).filter(file => file.endsWith('.mdx'));

  if (files.length === 0) {
    return [];
  }

  const posts = await Promise.all(
    files.map(async (filename) => {
      const slug = filename.replace('.mdx', '');
      try {
        const post = await getPostBySlug(slug);
        return post;
      } catch (error) {
        console.error(`Error parsing post ${slug}:`, error);
        return null;
      }
    })
  );

  return posts
    .filter((post): post is BlogPost => post !== null && post.frontmatter.published)
    .sort((a, b) => new Date(b.frontmatter.date).getTime() - new Date(a.frontmatter.date).getTime());
}

/**
 * Get a single post by slug with reading time calculation
 */
export async function getPostBySlug(slug: string): Promise<BlogPost> {
  const filePath = path.join(BLOG_DIR, `${slug}.mdx`);

  if (!fs.existsSync(filePath)) {
    throw new Error(`Post not found: ${slug}`);
  }

  const fileContent = fs.readFileSync(filePath, 'utf-8');
  const { data, content } = matter(fileContent);

  // Validate frontmatter with Zod schema
  const frontmatter = PostFrontmatterSchema.parse(data);

  // Calculate reading time
  const stats = readingTime(content);

  return {
    slug,
    frontmatter,
    content,
    readingTime: stats.text,
    wordCount: stats.words,
  };
}

/**
 * Get all post slugs for static generation
 */
export async function getAllPostSlugs(): Promise<string[]> {
  if (!ensureBlogDirExists()) {
    return [];
  }

  const files = fs.readdirSync(BLOG_DIR).filter(file => file.endsWith('.mdx'));
  return files.map(file => file.replace('.mdx', ''));
}

/**
 * Get post previews (without full content) for listings
 */
export async function getPostPreviews(): Promise<BlogPostPreview[]> {
  const posts = await getAllPosts();

  return posts.map(post => ({
    slug: post.slug,
    title: post.frontmatter.title,
    excerpt: post.frontmatter.excerpt,
    date: post.frontmatter.date,
    category: post.frontmatter.category,
    tags: post.frontmatter.tags,
    coverImage: post.frontmatter.coverImage,
    coverImageAlt: post.frontmatter.coverImageAlt,
    author: post.frontmatter.author,
    readingTime: post.readingTime,
    featured: post.frontmatter.featured,
  }));
}

/**
 * Get all unique categories with post counts
 */
export async function getAllCategories(): Promise<CategoryWithCount[]> {
  const posts = await getAllPosts();

  const categoryCounts = posts.reduce((acc, post) => {
    const category = post.frontmatter.category;
    acc[category] = (acc[category] || 0) + 1;
    return acc;
  }, {} as Record<string, number>);

  return BLOG_CATEGORIES
    .map(category => ({
      name: category,
      count: categoryCounts[category] || 0,
      slug: category.toLowerCase().replace(/\s+/g, '-'),
    }))
    .filter(cat => cat.count > 0)
    .sort((a, b) => b.count - a.count);
}

/**
 * Get all unique tags with post counts
 */
export async function getAllTags(): Promise<TagWithCount[]> {
  const posts = await getAllPosts();

  const tagCounts = posts.reduce((acc, post) => {
    post.frontmatter.tags.forEach(tag => {
      acc[tag] = (acc[tag] || 0) + 1;
    });
    return acc;
  }, {} as Record<string, number>);

  return Object.entries(tagCounts)
    .map(([name, count]) => ({
      name,
      count,
      slug: name.toLowerCase().replace(/\s+/g, '-'),
    }))
    .sort((a, b) => b.count - a.count);
}

/**
 * Get posts by category
 */
export async function getPostsByCategory(category: string): Promise<BlogPost[]> {
  const posts = await getAllPosts();
  const normalizedCategory = category.toLowerCase().replace(/-/g, ' ');

  return posts.filter(post =>
    post.frontmatter.category.toLowerCase() === normalizedCategory
  );
}

/**
 * Get posts by tag
 */
export async function getPostsByTag(tag: string): Promise<BlogPost[]> {
  const posts = await getAllPosts();
  const normalizedTag = tag.toLowerCase().replace(/-/g, ' ');

  return posts.filter(post =>
    post.frontmatter.tags.some(t => t.toLowerCase() === normalizedTag)
  );
}

/**
 * Get featured posts
 */
export async function getFeaturedPosts(): Promise<BlogPost[]> {
  const posts = await getAllPosts();
  return posts.filter(post => post.frontmatter.featured);
}

/**
 * Get recent posts (limit to n posts)
 */
export async function getRecentPosts(limit: number = 5): Promise<BlogPostPreview[]> {
  const previews = await getPostPreviews();
  return previews.slice(0, limit);
}

// Export client utilities for convenience
export { searchPosts } from './blog-client';

/**
 * Get related posts based on shared tags and category
 */
export async function getRelatedPosts(currentSlug: string, limit: number = 3): Promise<BlogPostPreview[]> {
  const currentPost = await getPostBySlug(currentSlug);
  const allPosts = await getAllPosts();

  // Filter out current post
  const otherPosts = allPosts.filter(post => post.slug !== currentSlug);

  // Score posts based on shared tags and category
  const scoredPosts = otherPosts.map(post => {
    let score = 0;

    // Same category: +3 points
    if (post.frontmatter.category === currentPost.frontmatter.category) {
      score += 3;
    }

    // Shared tags: +1 point per tag
    const sharedTags = post.frontmatter.tags.filter(tag =>
      currentPost.frontmatter.tags.includes(tag)
    );
    score += sharedTags.length;

    return { post, score };
  });

  // Sort by score and return top N
  const topPosts = scoredPosts
    .sort((a, b) => b.score - a.score)
    .slice(0, limit)
    .map(({ post }) => post);

  // Convert to previews
  return topPosts.map(post => ({
    slug: post.slug,
    title: post.frontmatter.title,
    excerpt: post.frontmatter.excerpt,
    date: post.frontmatter.date,
    category: post.frontmatter.category,
    tags: post.frontmatter.tags,
    coverImage: post.frontmatter.coverImage,
    coverImageAlt: post.frontmatter.coverImageAlt,
    author: post.frontmatter.author,
    readingTime: post.readingTime,
    featured: post.frontmatter.featured,
  }));
}

// Export client utilities for convenience
export { formatDate, getPostUrl, getCategoryUrl, getTagUrl } from './blog-client';
