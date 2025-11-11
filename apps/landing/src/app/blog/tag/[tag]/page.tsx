import BlogCard from '@/components/blog/BlogCard';
import CategoryFilter from '@/components/blog/CategoryFilter';
import GradientText from '@/components/common/GradientText';
import Footer from '@/components/globals/Footer';
import Navbar from '@/components/globals/navbar/Navbar';
import { getAllCategories, getAllTags, getPostsByTag } from '@/lib/blog';
import { Metadata } from 'next';
import { notFound } from 'next/navigation';

interface TagPageProps {
  params: {
    tag: string;
  };
}

/**
 * Generate static params for all tags
 * Enables static generation at build time
 */
export async function generateStaticParams() {
  const tags = await getAllTags();

  return tags.map((tag) => ({
    tag: tag.slug,
  }));
}

/**
 * Generate dynamic metadata for tag pages
 */
export async function generateMetadata({
  params,
}: TagPageProps): Promise<Metadata> {
  // Normalize tag slug back to display name
  const tagName = params.tag
    .split('-')
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(' ');

  return {
    title: `#${tagName} Articles | DeFindex Blog`,
    description: `Browse all articles tagged with #${tagName} on DeFindex. Explore DeFi insights, tutorials, and updates.`,
    openGraph: {
      title: `#${tagName} Articles | DeFindex Blog`,
      description: `Browse all articles tagged with #${tagName} on DeFindex.`,
      type: 'website',
      url: `https://defindex.io/blog/tag/${params.tag}`,
    },
    twitter: {
      card: 'summary_large_image',
      title: `#${tagName} Articles | DeFindex Blog`,
      description: `Browse all articles tagged with #${tagName} on DeFindex.`,
    },
  };
}

/**
 * Tag filter page
 * Shows all posts with a specific tag
 */
export default async function TagPage({ params }: TagPageProps) {
  // Normalize tag slug back to display name
  const tagName = params.tag
    .split('-')
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(' ');

  // Fetch posts for this tag
  const posts = await getPostsByTag(tagName);

  // If no posts found, return 404
  if (posts.length === 0) {
    // Check if tag exists at all
    const allTags = await getAllTags();
    const tagExists = allTags.some(
      (t) => t.name.toLowerCase() === tagName.toLowerCase()
    );

    if (!tagExists) {
      notFound();
    }
  }

  const categories = await getAllCategories();

  // Convert to previews
  const postPreviews = posts.map((post) => ({
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

  return (
    <div className="min-h-screen w-full bg-[#043036] relative overflow-hidden">
      {/* Background gradient effects */}
      <div className="absolute inset-0 bg-gradient-to-b from-dark via-darkCyan to-dark opacity-50" />
      <div className="absolute inset-0 w-full h-full">
        <div className="absolute -top-1/4 -left-1/4 w-[800px] h-[800px] bg-cyan-500/20 rounded-full blur-3xl" />
        <div className="absolute top-0 -right-1/4 w-[700px] h-[700px] bg-purple/15 rounded-full blur-3xl" />
        <div className="absolute bottom-0 left-1/4 w-[600px] h-[600px] bg-lime-200/10 rounded-full blur-3xl" />
        <div className="absolute top-1/3 right-1/4 w-[500px] h-[500px] bg-cyan-400/10 rounded-full blur-3xl" />
      </div>
      <div className="relative z-10">
        <Navbar />

        <main className="container mx-auto max-w-7xl px-4 py-12">
          {/* Header Section */}
          <header className="text-center mb-12 px-4">
            <div className="inline-block px-4 sm:px-6 py-2 bg-cyan-900/50 border border-cyan-800/30 rounded-full mb-4">
              <span className="text-lime-200 font-manrope font-bold text-base sm:text-lg">
                #{tagName}
              </span>
            </div>
            <GradientText
              as="h1"
              variant="purple"
              className="text-3xl sm:text-4xl md:text-5xl lg:text-6xl mb-4 font-familjen-grotesk font-bold leading-tight"
            >
              Tagged Articles
            </GradientText>
            <p className="text-cyan-100 font-inter text-base sm:text-lg md:text-xl">
              Showing {postPreviews.length}{' '}
              {postPreviews.length === 1 ? 'article' : 'articles'} with this tag
            </p>
          </header>

          {/* Main Content Grid */}
          <div className="grid grid-cols-1 lg:grid-cols-4 gap-8">
            {/* Sidebar - Category Filter */}
            <aside className="lg:col-span-1">
              <div className="sticky top-24">
                <CategoryFilter categories={categories} />
              </div>
            </aside>

            {/* Main Content - Posts */}
            <section className="lg:col-span-3">
              {postPreviews.length > 0 ? (
                <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                  {postPreviews.map((post) => (
                    <BlogCard key={post.slug} post={post} />
                  ))}
                </div>
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
                      d="M7 7h.01M7 3h5c.512 0 1.024.195 1.414.586l7 7a2 2 0 010 2.828l-7 7a2 2 0 01-2.828 0l-7-7A1.994 1.994 0 013 12V7a4 4 0 014-4z"
                    />
                  </svg>
                  <h2 className="text-2xl font-familjen-grotesk font-bold text-lime-200 mb-2">
                    No articles with this tag yet
                  </h2>
                  <p className="text-cyan-300 font-inter mb-4">
                    Check back soon for new content!
                  </p>
                  <a
                    href="/blog"
                    className="inline-block px-6 py-3 bg-cyan-900/50 border border-cyan-800/30 rounded-xl text-lime-200 hover:border-lime-200/50 transition-all font-manrope font-bold"
                  >
                    View All Articles
                  </a>
                </div>
              )}
            </section>
          </div>
        </main>

        <Footer />
      </div>
    </div>
  );
}
