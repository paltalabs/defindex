'use client';

import { useState, useEffect } from 'react';
import { searchPosts } from '@/lib/blog-client';
import type { BlogPostPreview } from '@/lib/blog-types';
import BlogCard from './BlogCard';

interface BlogSearchProps {
  posts: BlogPostPreview[];
}

/**
 * BlogSearch component - Client-side search functionality
 * Filters posts in real-time based on user query
 * Searches through title, excerpt, and tags
 *
 * @param posts - Array of blog post previews to search through
 */
export default function BlogSearch({ posts }: BlogSearchProps) {
  const [query, setQuery] = useState('');
  const [results, setResults] = useState<BlogPostPreview[]>(posts);

  // Update search results when query changes
  useEffect(() => {
    if (!query.trim()) {
      setResults(posts);
      return;
    }

    // Use the searchPosts utility function
    const filtered = searchPosts(posts, query) as BlogPostPreview[];
    setResults(filtered);
  }, [query, posts]);

  return (
    <div className="w-full">
      {/* Search Input */}
      <div className="relative mb-8">
        <div className="absolute inset-y-0 left-4 flex items-center pointer-events-none">
          <svg
            className="w-5 h-5 text-cyan-400"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
            />
          </svg>
        </div>
        <input
          type="search"
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          placeholder="Search articles by title, content, or tags..."
          className="w-full pl-12 pr-4 py-4 bg-cyan-950/50 border border-cyan-800/30 rounded-xl text-white placeholder-cyan-400 focus:border-lime-200 focus:outline-none focus:ring-2 focus:ring-lime-200/20 transition-all font-inter"
          aria-label="Search blog posts"
        />
        {query && (
          <button
            onClick={() => setQuery('')}
            className="absolute inset-y-0 right-4 flex items-center text-cyan-400 hover:text-lime-200 transition-colors"
            aria-label="Clear search"
          >
            <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        )}
      </div>

      {/* Results Count */}
      <div className="mb-6">
        <p className="text-cyan-200 font-inter">
          {query ? (
            <>
              Found <span className="text-lime-200 font-bold">{results.length}</span>{' '}
              {results.length === 1 ? 'article' : 'articles'} matching &quot;{query}&quot;
            </>
          ) : (
            <>
              Showing <span className="text-lime-200 font-bold">{results.length}</span>{' '}
              {results.length === 1 ? 'article' : 'articles'}
            </>
          )}
        </p>
      </div>

      {/* Results Grid */}
      {results.length > 0 ? (
        <>
          {/* Bento Grid - First 6 posts */}
          {results.length >= 1 && (
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mb-6">
              {results.slice(0, 6).map((post, index) => {
                // Featured post (first with featured: true) gets larger size
                const isFeatured = index === 0 && post.featured;
                return (
                  <div
                    key={post.slug}
                    className={`${
                      isFeatured
                        ? 'md:col-span-2 lg:col-span-2'
                        : ''
                    }`}
                  >
                    <BlogCard post={post} featured={isFeatured} />
                  </div>
                );
              })}
            </div>
          )}

          {/* Regular Grid - Remaining posts (7+) */}
          {results.length > 6 && (
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mt-6">
              {results.slice(6).map((post) => (
                <BlogCard key={post.slug} post={post} featured={false} />
              ))}
            </div>
          )}
        </>
      ) : (
        <div className="text-center py-12 bg-cyan-950/30 backdrop-blur-sm border border-cyan-800/30 rounded-2xl">
          <svg
            className="w-16 h-16 mx-auto mb-4 text-cyan-600"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M9.172 16.172a4 4 0 015.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
            />
          </svg>
          <h3 className="text-xl font-familjen-grotesk font-bold text-lime-200 mb-2">
            No articles found
          </h3>
          <p className="text-cyan-300 font-inter">
            Try adjusting your search query or browse all articles
          </p>
          <button
            onClick={() => setQuery('')}
            className="mt-4 px-6 py-2 bg-cyan-900/50 border border-cyan-800/30 rounded-lg text-lime-200 hover:border-lime-200/50 transition-colors font-manrope font-bold"
          >
            Clear search
          </button>
        </div>
      )}
    </div>
  );
}
