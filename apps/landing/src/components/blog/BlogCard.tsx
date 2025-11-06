import GradientText from '@/components/common/GradientText';
import { formatDate, getCategoryUrl, getPostUrl, getTagUrl } from '@/lib/blog-client';
import type { BlogPostPreview } from '@/lib/blog-types';
import Image from 'next/image';
import Link from 'next/link';

interface BlogCardProps {
  post: BlogPostPreview;
  featured?: boolean;
}

/**
 * BlogCard component for displaying blog post previews
 * Uses glassmorphic design with hover effects matching the site aesthetic
 *
 * @param post - Blog post preview data
 * @param featured - Whether this is a featured post (larger display)
 */
export default function BlogCard({ post, featured = false }: BlogCardProps) {
  const {
    slug,
    title,
    excerpt,
    date,
    category,
    tags,
    coverImage,
    coverImageAlt,
    author,
    readingTime,
  } = post;

  return (
    <article
      className={`
        group relative flex flex-col h-full
        bg-cyan-950/30 backdrop-blur-sm
        border border-cyan-800/30 rounded-2xl
        overflow-hidden
        hover:border-lime-200/50
        transition-all duration-300
        ${featured ? 'md:flex-row' : ''}
      `}
    >
      {/* Cover Image */}
      <Link
        href={getPostUrl(slug)}
        className={`relative overflow-hidden ${featured ? 'md:w-1/2' : 'w-full h-48'}`}
      >
        <Image
          src={coverImage}
          alt={coverImageAlt}
          width={800}
          height={450}
          className="w-full h-full object-cover group-hover:scale-105 transition-transform duration-300"
        />
        {/* Featured badge */}
        {post.featured && (
          <div className="absolute top-4 left-4 bg-gradient-to-r from-lime-200 to-purple px-3 rounded-full">
            <span className="text-dark font-manrope font-bold text-xs uppercase">
              Featured
            </span>
          </div>
        )}
      </Link>

      {/* Content */}
      <div className={`p-6 flex flex-col ${featured ? 'md:w-1/2' : ''}`}>
        {/* Category Badge */}
        <Link
          href={getCategoryUrl(category)}
          className="w-fit mb-3 px-4 bg-cyan-900/50 border border-cyan-800/30 rounded-full hover:border-lime-200/50 transition-colors h-[32px] justify-center flex items-center"
        >
          <span className="text-lime-200 font-manrope font-bold text-xs uppercase">
            {category}
          </span>
        </Link>

        {/* Title */}
        <Link href={getPostUrl(slug)} className="group/title">
          <GradientText
            as="h3"
            variant="green"
            className={`${
              featured ? 'text-xl md:text-2xl' : 'text-lg md:text-lg'
            } mb-3 font-familjen-grotesk font-bold group-hover/title:opacity-80 transition-opacity`}
          >
            {title}
          </GradientText>
        </Link>

        {/* Excerpt */}
        <p className="text-cyan-100 font-inter mb-4 flex-grow line-clamp-3">
          {excerpt}
        </p>

        {/* Tags */}
        <div className="flex flex-wrap gap-2 mb-4">
          {tags.slice(0, 3).map((tag) => (
            <Link
              key={tag}
              href={getTagUrl(tag)}
              className="px-2 bg-cyan-900/30 border border-cyan-800/20 rounded text-cyan-300 hover:text-lime-200 hover:border-cyan-700 transition-colors text-xs font-inter"
            >
              #{tag}
            </Link>
          ))}
          {tags.length > 3 && (
            <span className="px-2 text-cyan-400 text-xs font-inter">
              +{tags.length - 3} more
            </span>
          )}
        </div>

        {/* Meta Info */}
        <div className="flex items-center justify-between pt-4 border-t border-cyan-800/30">
          {/* Author & Date */}
          <div className="flex items-center gap-3">
            {author.avatar && (
              <Image
                src={author.avatar}
                alt={author.name}
                width={32}
                height={32}
                className="rounded-full border border-cyan-800/30"
              />
            )}
            <div className="flex flex-col">
              <span className="text-cyan-100 font-inter text-sm font-medium">
                {author.name}
              </span>
              <span className="text-cyan-400 font-inter text-xs">
                {formatDate(date)}
              </span>
            </div>
          </div>

          {/* Reading Time */}
          <div className="flex items-center gap-1 text-cyan-400 text-xs font-inter">
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
            <span>{readingTime}</span>
          </div>
        </div>
      </div>
    </article>
  );
}
