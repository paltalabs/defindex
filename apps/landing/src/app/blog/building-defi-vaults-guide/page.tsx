import BlogCard from '@/components/blog/BlogCard';
import BlogPost from '@/components/blog/BlogPost';
import GradientText from '@/components/common/GradientText';
import Footer from '@/components/globals/Footer';
import Navbar from '@/components/globals/navbar/Navbar';
import { getPostBySlug, getRelatedPosts } from '@/lib/blog';
import { Metadata } from 'next';
import Link from 'next/link';
import { notFound } from 'next/navigation';
import Content from './content.mdx';

const slug = 'building-defi-vaults-guide';

export async function generateMetadata(): Promise<Metadata> {
  try {
    const post = await getPostBySlug(slug);
    const { frontmatter: fm } = post;

    return {
      title: fm.seoTitle || `${fm.title} | DeFindex Blog`,
      description: fm.seoDescription || fm.excerpt,
      openGraph: {
        title: fm.title,
        description: fm.excerpt,
        type: 'article',
        publishedTime: fm.date,
        modifiedTime: fm.lastModified || fm.date,
        authors: [fm.author.name],
        images: [
          {
            url: fm.coverImage,
            alt: fm.coverImageAlt,
          },
        ],
        url: `https://defindex.io/blog/${slug}`,
      },
      twitter: {
        card: 'summary_large_image',
        title: fm.title,
        description: fm.excerpt,
        images: [fm.coverImage],
      },
      alternates: {
        canonical: `https://defindex.io/blog/${slug}`,
      },
    };
  } catch (error) {
    return {
      title: 'Post Not Found | DeFindex Blog',
      description: 'The requested blog post could not be found.',
    };
  }
}

export default async function BuildingDeFiVaultsGuidePage() {
  let post;

  try {
    post = await getPostBySlug(slug);
  } catch (error) {
    notFound();
  }

  if (!post.frontmatter.published) {
    notFound();
  }

  const relatedPosts = await getRelatedPosts(slug);

  const jsonLd = {
    '@context': 'https://schema.org',
    '@type': 'BlogPosting',
    headline: post.frontmatter.title,
    description: post.frontmatter.excerpt,
    image: post.frontmatter.coverImage,
    datePublished: post.frontmatter.date,
    dateModified: post.frontmatter.lastModified || post.frontmatter.date,
    author: {
      '@type': 'Organization',
      name: post.frontmatter.author.name,
    },
    publisher: {
      '@type': 'Organization',
      name: 'DeFindex',
      logo: {
        '@type': 'ImageObject',
        url: 'https://defindex.io/logo.png',
      },
    },
    mainEntityOfPage: {
      '@type': 'WebPage',
      '@id': `https://defindex.io/blog/${slug}`,
    },
  };

  return (
    <>
      <script
        type="application/ld+json"
        dangerouslySetInnerHTML={{ __html: JSON.stringify(jsonLd) }}
      />

      <div className="min-h-screen w-full bg-[#043036] relative overflow-hidden">
        <div className="absolute inset-0 bg-gradient-to-b from-dark via-darkCyan to-dark opacity-50" />
        <div className="absolute inset-0 w-full h-full">
          <div className="absolute -top-1/4 -left-1/4 w-[800px] h-[800px] bg-cyan-500/20 rounded-full blur-3xl" />
          <div className="absolute top-0 -right-1/4 w-[700px] h-[700px] bg-purple/15 rounded-full blur-3xl" />
          <div className="absolute bottom-0 left-1/4 w-[600px] h-[600px] bg-lime-200/10 rounded-full blur-3xl" />
          <div className="absolute top-1/3 right-1/4 w-[500px] h-[500px] bg-cyan-400/10 rounded-full blur-3xl" />
        </div>
        <div className="relative z-10">
          <Navbar />

          <main className="container mx-auto max-w-4xl px-4 pt-28 pb-12">
            <Link
              href="/blog"
              className="inline-flex items-center gap-2 mb-8 px-4 py-2 bg-cyan-900/50 border border-cyan-800/30 rounded-lg text-cyan-200 hover:border-lime-200/50 hover:text-lime-200 transition-all duration-300 hover:scale-105 group cursor-pointer"
            >
              <svg
                className="w-5 h-5 transition-transform duration-300 group-hover:-translate-x-1"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M15 19l-7-7 7-7"
                />
              </svg>
              <span className="font-inter text-sm">Back to Blog</span>
            </Link>

            <BlogPost post={post} url={`https://defindex.io/blog/${slug}`}>
              <Content />
            </BlogPost>

            {relatedPosts.length > 0 && (
              <section className="mt-12">
                <GradientText
                  as="h2"
                  variant="green"
                  className="text-lg md:text-xl mb-6 font-familjen-grotesk font-bold"
                >
                  Related Articles
                </GradientText>
                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                  {relatedPosts.map((relatedPost) => (
                    <BlogCard key={relatedPost.slug} post={relatedPost} />
                  ))}
                </div>
              </section>
            )}
          </main>

          <Footer />
        </div>
      </div>
    </>
  );
}
