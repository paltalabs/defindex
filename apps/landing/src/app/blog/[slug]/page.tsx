import BlogCard from '@/components/blog/BlogCard';
import BlogPost from '@/components/blog/BlogPost';
import { MDXComponents } from '@/components/blog/MDXComponents';
import GradientText from '@/components/common/GradientText';
import Footer from '@/components/globals/Footer';
import Navbar from '@/components/globals/navbar/Navbar';
import {
  getAllPostSlugs,
  getPostBySlug,
  getRelatedPosts,
} from '@/lib/blog';
import { Metadata } from 'next';
import { MDXRemote } from 'next-mdx-remote/rsc';
import { notFound } from 'next/navigation';
import rehypeAutolinkHeadings from 'rehype-autolink-headings';
import rehypePrismPlus from 'rehype-prism-plus';
import rehypeSlug from 'rehype-slug';
import remarkGfm from 'remark-gfm';

interface BlogPostPageProps {
  params: {
    slug: string;
  };
}

/**
 * Generate static params for all blog posts
 * This enables static generation at build time for better performance
 */
export async function generateStaticParams() {
  const slugs = await getAllPostSlugs();

  return slugs.map((slug) => ({
    slug,
  }));
}

/**
 * Generate dynamic metadata for SEO
 * Includes Open Graph and Twitter Card data
 */
export async function generateMetadata({
  params,
}: BlogPostPageProps): Promise<Metadata> {
  try {
    const post = await getPostBySlug(params.slug);
    const { frontmatter } = post;

    return {
      title: frontmatter.seoTitle || `${frontmatter.title} | DeFindex Blog`,
      description:
        frontmatter.seoDescription || frontmatter.excerpt,
      openGraph: {
        title: frontmatter.title,
        description: frontmatter.excerpt,
        type: 'article',
        publishedTime: frontmatter.date,
        modifiedTime: frontmatter.lastModified || frontmatter.date,
        authors: [frontmatter.author.name],
        images: [
          {
            url: frontmatter.coverImage,
            alt: frontmatter.coverImageAlt,
          },
        ],
        url: `https://defindex.io/blog/${params.slug}`,
      },
      twitter: {
        card: 'summary_large_image',
        title: frontmatter.title,
        description: frontmatter.excerpt,
        images: [frontmatter.coverImage],
      },
      alternates: {
        canonical: `https://defindex.io/blog/${params.slug}`,
      },
    };
  } catch (error) {
    return {
      title: 'Post Not Found | DeFindex Blog',
      description: 'The requested blog post could not be found.',
    };
  }
}

/**
 * Individual blog post page
 * Renders MDX content with custom components and shows related posts
 */
export default async function BlogPostPage({ params }: BlogPostPageProps) {
  let post;

  try {
    post = await getPostBySlug(params.slug);
  } catch (error) {
    notFound();
  }

  // Don't show unpublished posts
  if (!post.frontmatter.published) {
    notFound();
  }

  // Fetch related posts
  const relatedPosts = await getRelatedPosts(params.slug);

  // JSON-LD structured data for SEO
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
      '@id': `https://defindex.io/blog/${params.slug}`,
    },
  };

  return (
    <>
      {/* JSON-LD structured data */}
      <script
        type="application/ld+json"
        dangerouslySetInnerHTML={{ __html: JSON.stringify(jsonLd) }}
      />

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

          <main className="container mx-auto max-w-4xl px-4 py-12">
            {/* Blog Post Content */}
            <BlogPost post={post} url={`https://defindex.io/blog/${params.slug}`}>
              <MDXRemote
                source={post.content}
                components={MDXComponents}
                options={{
                  mdxOptions: {
                    remarkPlugins: [remarkGfm],
                    rehypePlugins: [
                      rehypePrismPlus,
                      rehypeSlug,
                      [
                        rehypeAutolinkHeadings,
                        {
                          behavior: 'wrap',
                          properties: {
                            className: ['anchor-link'],
                          },
                        },
                      ],
                    ],
                  },
                }}
              />
            </BlogPost>

            {/* Related Posts Section */}
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
