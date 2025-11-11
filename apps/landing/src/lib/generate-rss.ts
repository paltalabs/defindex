import { Feed } from 'feed';
import fs from 'fs';
import path from 'path';
import { getAllPosts } from './blog';

/**
 * Generate RSS feed for the blog
 * Creates an RSS 2.0 feed with all published posts
 * Should be run during build time (postbuild script)
 */
export async function generateRSSFeed() {
  const posts = await getAllPosts();
  const siteUrl = process.env.NEXT_PUBLIC_SITE_URL || 'https://defindex.io';

  // Initialize feed
  const feed = new Feed({
    title: 'DeFindex Blog',
    description:
      'Latest updates, tutorials, and insights about DeFi yield optimization, vault strategies, and blockchain development from DeFindex.',
    id: `${siteUrl}/blog`,
    link: `${siteUrl}/blog`,
    language: 'en',
    image: `${siteUrl}/og-image.jpg`,
    favicon: `${siteUrl}/favicon.ico`,
    copyright: `All rights reserved ${new Date().getFullYear()}, DeFindex`,
    updated: posts.length > 0 ? new Date(posts[0].frontmatter.date) : new Date(),
    generator: 'DeFindex Blog Generator',
    feedLinks: {
      rss2: `${siteUrl}/rss.xml`,
      json: `${siteUrl}/feed.json`,
      atom: `${siteUrl}/atom.xml`,
    },
    author: {
      name: 'DeFindex Team',
      email: 'hello@defindex.io',
      link: siteUrl,
    },
  });

  // Add each post to the feed
  posts.forEach((post) => {
    const postUrl = `${siteUrl}/blog/${post.slug}`;

    // Handle image URL - check if it's already a complete URL
    let coverImage = post.frontmatter.coverImage.startsWith('http')
      ? post.frontmatter.coverImage
      : `${siteUrl}${post.frontmatter.coverImage}`;

    // Escape ampersands in URLs for XML compliance
    coverImage = coverImage.replace(/&/g, '&amp;');

    // Create HTML content with image
    const contentWithImage = `<img src="${coverImage}" alt="${post.frontmatter.coverImageAlt}" /><br/><br/>${post.content}`;

    feed.addItem({
      title: post.frontmatter.title,
      id: postUrl,
      link: postUrl,
      description: post.frontmatter.excerpt,
      content: contentWithImage,
      author: [
        {
          name: post.frontmatter.author.name,
          email: 'hello@defindex.io', // Generic team email
        },
      ],
      date: new Date(post.frontmatter.date),
      // Don't use image property as it creates malformed enclosure elements
      category: [
        {
          name: post.frontmatter.category,
        },
        ...post.frontmatter.tags.map((tag) => ({
          name: tag,
        })),
      ],
    });
  });

  // Write RSS feed to public directory
  const rssPath = path.join(process.cwd(), 'public/rss.xml');
  fs.writeFileSync(rssPath, feed.rss2());

  // Write Atom feed to public directory
  const atomPath = path.join(process.cwd(), 'public/atom.xml');
  fs.writeFileSync(atomPath, feed.atom1());

  // Write JSON feed to public directory
  const jsonPath = path.join(process.cwd(), 'public/feed.json');
  fs.writeFileSync(jsonPath, feed.json1());

  console.log('✅ RSS feed generated successfully!');
  console.log(`   - RSS 2.0: ${rssPath}`);
  console.log(`   - Atom 1.0: ${atomPath}`);
  console.log(`   - JSON Feed: ${jsonPath}`);

  return feed;
}

// Allow running this script directly
if (require.main === module) {
  generateRSSFeed()
    .then(() => {
      console.log('RSS feed generation complete');
      process.exit(0);
    })
    .catch((error) => {
      console.error('Error generating RSS feed:', error);
      process.exit(1);
    });
}
