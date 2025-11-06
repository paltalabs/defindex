'use client';
import GradientText from '@/components/common/GradientText';
import { formatDate, getCategoryUrl, getTagUrl } from '@/lib/blog-client';
import type { BlogPost as BlogPostType } from '@/lib/blog-types';
import Image from 'next/image';
import Link from 'next/link';
import ShareButtons from './ShareButtons';

interface BlogPostProps {
  post: BlogPostType;
  children: React.ReactNode;
  url?: string;
}

/**
 * BlogPost layout component for individual blog post pages
 * Provides a consistent layout with header, meta info, and content area
 *
 * @param post - Full blog post data including frontmatter
 * @param children - MDX content rendered as React components
 * @param url - Full URL of the post for sharing
 */
export default function BlogPost({ post, children, url }: BlogPostProps) {
  const { frontmatter, readingTime } = post;
  const postUrl = url || `https://defindex.io/blog/${post.slug}`;

  return (
    <article className="w-full">
      {/* Header Section with Cover Image */}
      <header className="relative w-full mb-8">
        {/* Cover Image */}
        <div className="relative w-full h-64 md:h-96 rounded-2xl overflow-hidden border border-cyan-800/30 mb-6">
          <Image
            src={frontmatter.coverImage}
            alt={frontmatter.coverImageAlt}
            fill
            sizes="(max-width: 768px) 100vw, (max-width: 1200px) 80vw, 1200px"
            className="object-cover"
            priority
          />
          {/* Gradient Overlay for better text readability */}
          <div className="absolute inset-0 bg-gradient-to-t from-dark via-dark/50 to-transparent" />
        </div>

        {/* Category Badge */}
        <Link
          href={getCategoryUrl(frontmatter.category)}
          className="inline-block mb-4 px-4 py-2 bg-cyan-900/50 border border-cyan-800/30 rounded-full hover:border-lime-200/50 transition-colors"
        >
          <span className="text-lime-200 font-manrope font-bold text-sm uppercase">
            {frontmatter.category}
          </span>
        </Link>

        {/* Title */}
        <GradientText
          as="h1"
          variant="primary"
          className="text-lg md:text-xl lg:text-2xl mb-6 font-familjen-grotesk font-bold"
        >
          {frontmatter.title}
        </GradientText>

        {/* Meta Information */}
        <div className="flex flex-wrap items-center gap-4 mb-6 text-cyan-200">
          {/* Author */}
          <div className="flex items-center gap-2">
            {frontmatter.author.avatar && (
              <Image
                src={frontmatter.author.avatar}
                alt={frontmatter.author.name}
                width={40}
                height={40}
                className="rounded-full border border-cyan-800/30"
              />
            )}
            <span className="font-inter font-medium">
              {frontmatter.author.name}
            </span>
          </div>

          <span className="text-cyan-600">•</span>

          {/* Date */}
          <time dateTime={frontmatter.date} className="font-inter">
            {formatDate(frontmatter.date)}
          </time>

          <span className="text-cyan-600">•</span>

          {/* Reading Time */}
          <div className="flex items-center gap-1">
            <svg
              className="w-4 h-4"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
              />
            </svg>
            <span className="font-inter">{readingTime}</span>
          </div>
        </div>

        {/* Tags */}
        <div className="flex flex-wrap gap-2">
          {frontmatter.tags.map((tag) => (
            <Link
              key={tag}
              href={getTagUrl(tag)}
              className="px-3 py-1 bg-cyan-900/30 border border-cyan-800/20 rounded-full text-cyan-300 hover:text-lime-200 hover:border-cyan-700 transition-colors text-sm font-inter"
            >
              #{tag}
            </Link>
          ))}
        </div>
      </header>

      {/* Content Section */}
      <div className="bg-cyan-950/30 backdrop-blur-sm border border-cyan-800/30 rounded-2xl p-6 md:p-10">
        {/* MDX Content with custom styling */}
        <div className="prose prose-invert prose-lime max-w-none">
          {children}
        </div>
      </div>

      {/* Footer Section */}
      <footer className="mt-8 p-6 bg-cyan-950/30 backdrop-blur-sm border border-cyan-800/30 rounded-2xl">
        {/* Last Modified Date */}
        {frontmatter.lastModified && (
          <p className="text-cyan-400 text-sm font-inter mb-4">
            Last updated: {formatDate(frontmatter.lastModified)}
          </p>
        )}

        {/* Share Section */}
        <div className="flex items-center justify-between flex-wrap gap-4">
          <p className="text-cyan-100 font-inter font-medium">
            Share this article:
          </p>
          <ShareButtons title={frontmatter.title} url={postUrl} />
        </div>
      </footer>
    </article>
  );
}
