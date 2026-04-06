import createMDX from '@next/mdx';
import rehypeAutolinkHeadings from 'rehype-autolink-headings';
import rehypePrismPlus from 'rehype-prism-plus';
import rehypeSlug from 'rehype-slug';
import remarkFrontmatter from 'remark-frontmatter';
import remarkGfm from 'remark-gfm';
import remarkMdxFrontmatter from 'remark-mdx-frontmatter';

/** @type {import('next').NextConfig} */
const nextConfig = {
  // Configure MDX page extensions
  pageExtensions: ['js', 'jsx', 'md', 'mdx', 'ts', 'tsx'],

  async rewrites() {
    return [
      {
        source: "/relay-It6G/static/:path*",
        destination: "https://us-assets.i.posthog.com/static/:path*",
      },
      {
        source: "/relay-It6G/:path*",
        destination: "https://us.i.posthog.com/:path*",
      },
    ];
  },
  // This is required to support PostHog trailing slash API requests
  skipTrailingSlashRedirect: true, 

  // Configure image domains
  images: {
    remotePatterns: [
      {
        protocol: 'https',
        hostname: 'images.unsplash.com',
      },
      {
        protocol: 'https',
        hostname: 'pbs.twimg.com',
      },
      {
        protocol: 'https',
        hostname: 'ipfs.io',
      },
      {
        protocol: 'https',
        hostname: 'stellar.myfilebase.com',
      },
      {
        protocol: 'https',
        hostname: 'stablebonds.s3.us-west-2.amazonaws.com',
      },
      {
        protocol: 'https',
        hostname: 'xlmeme.com',
      },
      {
        protocol: 'https',
        hostname: 'app.glodollar.org',
      },
      {
        protocol: 'https',
        hostname: 'imagedelivery.net',
      },
      {
        protocol: 'https',
        hostname: 'cdn.ondo.finance',
      },
      {
        protocol: 'https',
        hostname: 'reflector.network',
      },
      {
        protocol: 'https',
        hostname: 'uploads-ssl.webflow.com',
      },
      {
        protocol: 'https',
        hostname: 'skyhitz.io',
      },
      {
        protocol: 'https',
        hostname: 'cdn.lu.meme',
      },
      {
        protocol: 'https',
        hostname: 'testnet.orbitcdp.finance',
      },
      {
        protocol: 'https',
        hostname: '424565.fs1.hubspotusercontent-na1.net',
      },
      {
        protocol: 'https',
        hostname: 'static.anclap.com',
      },
    ],
  },

  // Configure headers for security and SEO
  async headers() {
    return [
      {
        source: '/blog/:path*',
        headers: [
          {
            key: 'Content-Security-Policy',
            value: "default-src 'self'; script-src 'self' 'unsafe-inline' 'unsafe-eval'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:; font-src 'self' data:; connect-src 'self';"
          },
          {
            key: 'X-Content-Type-Options',
            value: 'nosniff'
          },
          {
            key: 'X-Frame-Options',
            value: 'DENY'
          },
          {
            key: 'X-XSS-Protection',
            value: '1; mode=block'
          }
        ]
      }
    ];
  }
};

// MDX configuration with plugins
const withMDX = createMDX({
  options: {
    remarkPlugins: [
      remarkGfm,
      remarkFrontmatter,
      [remarkMdxFrontmatter, { name: 'frontmatter' }],
    ],
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
});

export default withMDX(nextConfig);
