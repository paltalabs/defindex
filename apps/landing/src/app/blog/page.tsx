import BlogSearch from '@/components/blog/BlogSearch';
import CategoryFilter from '@/components/blog/CategoryFilter';
import Footer from '@/components/globals/Footer';
import Navbar from '@/components/globals/navbar/Navbar';
import { getAllCategories, getPostPreviews } from '@/lib/blog';
import { Metadata } from 'next';
import Link from 'next/link';

/**
 * Metadata for the blog listing page
 * Optimized for SEO and social sharing
 */
export const metadata: Metadata = {
  title: 'Blog | DeFindex - DeFi Insights, Tutorials & Updates',
  description:
    'Explore the latest DeFi insights, technical tutorials, and updates from DeFindex. Learn about yield optimization, vault strategies, and blockchain development.',
  openGraph: {
    title: 'DeFindex Blog - DeFi Insights & Tutorials',
    description:
      'Explore the latest DeFi insights, technical tutorials, and updates from DeFindex.',
    type: 'website',
    url: 'https://defindex.io/blog',
  },
  twitter: {
    card: 'summary_large_image',
    title: 'DeFindex Blog - DeFi Insights & Tutorials',
    description:
      'Explore the latest DeFi insights, technical tutorials, and updates from DeFindex.',
  },
};

/**
 * Blog listing page
 * Displays all published blog posts with search and category filtering
 */
export default async function BlogPage() {
  // Fetch all posts and categories
  const posts = await getPostPreviews();
  const categories = await getAllCategories();

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

        <main className="container mx-auto max-w-full px-4 py-12">
          {/* Header Section */}
          <header className="text-center mb-12 px-4 align-middle">
            <Link href="/" className="inline-block mb-4">
              <img
                className="h-[48px] md:h-[64px] lg:h-[84px] mx-auto"
                src="/images/defindex.svg"
                alt="DeFindex"
                style={{filter: 'drop-shadow(0 2px 4px rgba(0,0,0,0.6))'}}
              />
            </Link>
            <p className="text-cyan-100 font-inter text-base sm:text-lg md:text-lg max-w-2xl mx-auto leading-relaxed">
              Insights, tutorials, and updates from the world of decentralized
              finance. Stay informed about the latest in DeFi yield optimization
              and vault strategies.
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

            {/* Main Content - Search and Posts */}
            <section className="lg:col-span-3">
              {posts.length > 0 ? (
                <BlogSearch posts={posts} />
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
                    No posts yet
                  </h2>
                  <p className="text-cyan-300 font-inter">
                    Check back soon for exciting content about DeFi and yield
                    optimization!
                  </p>
                </div>
              )}
            </section>
          </div>

          {/* RSS Feed Link */}
          <div className="mt-12 text-center">
            <a
              href="/rss.xml"
              target="_blank"
              rel="noopener noreferrer"
              className="inline-flex items-center gap-2 px-6 py-3 bg-cyan-900/50 border border-cyan-800/30 rounded-xl text-cyan-200 hover:border-lime-200/50 hover:text-lime-200 transition-all font-inter"
            >
              <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                <path d="M6.18 15.64a2.18 2.18 0 0 1 2.18 2.18C8.36 19 7.38 20 6.18 20C5 20 4 19 4 17.82a2.18 2.18 0 0 1 2.18-2.18M4 4.44A15.56 15.56 0 0 1 19.56 20h-2.83A12.73 12.73 0 0 0 4 7.27V4.44m0 5.66a9.9 9.9 0 0 1 9.9 9.9h-2.83A7.07 7.07 0 0 0 4 12.93V10.1z" />
              </svg>
              Subscribe via RSS
            </a>
          </div>
        </main>

        <Footer />
      </div>
    </div>
  );
}
