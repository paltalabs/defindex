import BlogCard from '@/components/blog/BlogCard';
import CategoryFilter from '@/components/blog/CategoryFilter';
import GradientText from '@/components/common/GradientText';
import Footer from '@/components/globals/Footer';
import Navbar from '@/components/globals/navbar/Navbar';
import {
  getAllCategories,
  getPostsByCategory
} from '@/lib/blog';
import { BLOG_CATEGORIES } from '@/lib/blog-types';
import { Metadata } from 'next';
import { notFound } from 'next/navigation';

interface CategoryPageProps {
  params: Promise<{
    category: string;
  }>;
}

/**
 * Generate static params for all categories
 * Enables static generation at build time
 */
export async function generateStaticParams() {
  const categories = await getAllCategories();

  return categories.map((category) => ({
    category: category.slug,
  }));
}

/**
 * Generate dynamic metadata for category pages
 */
export async function generateMetadata({
  params,
}: CategoryPageProps): Promise<Metadata> {
  const { category } = await params;
  // Normalize category slug to match enum
  const categoryName = category
    .split('-')
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(' ');

  // Check if it's a valid category
  const isValid = BLOG_CATEGORIES.some(
    (cat) => cat.toLowerCase() === categoryName.toLowerCase()
  );

  if (!isValid) {
    return {
      title: 'Category Not Found | DeFindex Blog',
      description: 'The requested category could not be found.',
    };
  }

  return {
    title: `${categoryName} Articles | DeFindex Blog`,
    description: `Browse all ${categoryName.toLowerCase()} articles on DeFindex. Learn about DeFi, yield optimization, and blockchain development.`,
    openGraph: {
      title: `${categoryName} Articles | DeFindex Blog`,
      description: `Browse all ${categoryName.toLowerCase()} articles on DeFindex.`,
      type: 'website',
      url: `https://defindex.io/blog/category/${category}`,
    },
    twitter: {
      card: 'summary_large_image',
      title: `${categoryName} Articles | DeFindex Blog`,
      description: `Browse all ${categoryName.toLowerCase()} articles on DeFindex.`,
    },
  };
}

/**
 * Category filter page
 * Shows all posts in a specific category
 */
export default async function CategoryPage({ params }: CategoryPageProps) {
  const { category } = await params;
  // Normalize category slug to match enum
  const categoryName = category
    .split('-')
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(' ');

  // Check if it's a valid category
  const isValid = BLOG_CATEGORIES.some(
    (cat) => cat.toLowerCase() === categoryName.toLowerCase()
  );

  if (!isValid) {
    notFound();
  }

  // Fetch posts for this category
  const posts = await getPostsByCategory(categoryName);
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
            <GradientText
              as="h1"
              variant="green"
              className="text-3xl sm:text-4xl md:text-5xl lg:text-6xl mb-4 font-familjen-grotesk font-bold leading-tight"
            >
              {categoryName}
            </GradientText>
            <p className="text-cyan-100 font-inter text-base sm:text-lg md:text-xl">
              Showing {postPreviews.length}{' '}
              {postPreviews.length === 1 ? 'article' : 'articles'} in this
              category
            </p>
          </header>

          {/* Main Content Grid */}
          <div className="grid grid-cols-1 lg:grid-cols-4 gap-8">
            {/* Sidebar - Category Filter */}
            <aside className="lg:col-span-1">
              <div className="sticky top-24">
                <CategoryFilter
                  categories={categories}
                  currentCategory={categoryName}
                />
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
                      d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
                    />
                  </svg>
                  <h2 className="text-2xl font-familjen-grotesk font-bold text-lime-200 mb-2">
                    No articles in this category yet
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
