# DeFindex Landing Page

This is the official landing page for [DeFindex](https://defindex.io), a Yield-as-a-Service platform built on Stellar. The site includes a blog system for sharing DeFi insights, tutorials, and updates.

## Tech Stack

- **Next.js 14.2.15** - React framework with App Router
- **TypeScript 5** - Type safety
- **Tailwind CSS 3.4** - Utility-first styling
- **MDX** - Markdown with React components for blog posts
- **Zod** - Schema validation for blog frontmatter

## Getting Started

### Development

First, install dependencies:

```bash
yarn install
```

Then, run the development server:

```bash
yarn dev
```

Open [http://localhost:3000](http://localhost:3000) to view the site.

### Building for Production

Build the project:

```bash
yarn build
```

This will:
1. Compile the Next.js application
2. Generate static pages for blog posts
3. Create RSS/Atom/JSON feeds

Start the production server:

```bash
yarn start
```

## Blog System

The DeFindex blog is a fully-featured MDX-based content management system with:

- 📝 **MDX Support** - Write posts in Markdown with React components
- 🔍 **Search** - Real-time client-side search
- 🏷️ **Categories & Tags** - Organize content effectively
- 📊 **Reading Time** - Automatic calculation
- 🔗 **RSS Feeds** - RSS 2.0, Atom 1.0, and JSON Feed formats
- 🎨 **Custom Design** - Fully integrated with DeFindex design system
- 🔒 **Security** - Frontmatter validation, CSP headers, component whitelisting

### Blog Structure

```
/content/blog/              # Blog posts (MDX files)
/public/blog/images/        # Blog images
/src/app/blog/              # Blog routes
/src/components/blog/       # Blog components
/src/lib/blog.ts           # Blog utilities
```

### Creating a New Blog Post

1. **Create a new MDX file** in `/content/blog/`:

```bash
touch content/blog/my-new-post.mdx
```

2. **Add frontmatter** at the top of the file:

```mdx
---
title: "Your Post Title"
slug: "your-post-title"
date: "2025-01-20T10:00:00.000Z"
excerpt: "A compelling 50-300 character summary of your post that will appear in listings and search results."
author:
  name: "DeFindex Team"
category: "Tutorial"  # Options: Tutorial, News, DeFi, Technical, Updates
tags: ["DeFi", "Stellar", "Tutorial"]  # 1-5 tags
coverImage: "https://images.unsplash.com/photo-example?w=1200&h=630"
coverImageAlt: "Description of cover image"
published: true  # Set to false for drafts
featured: false  # Set to true to feature on homepage
seoTitle: "Optional custom SEO title (max 60 chars)"
seoDescription: "Optional custom SEO description (max 160 chars)"
---

# Your Content Here

Write your post content using Markdown and MDX.

## Features Available

- **Bold** and *italic* text
- Lists (ordered and unordered)
- Code blocks with syntax highlighting
- Images with automatic optimization
- Links (internal and external)
- Tables
- Blockquotes
- And more!

### Code Example

\`\`\`typescript
import { DeFindexClient } from '@defindex/sdk';

const client = new DeFindexClient({
  apiKey: 'your-api-key'
});
\`\`\`

### Images

![Alt text](/blog/images/my-new-post/image.jpg)

Images are automatically optimized by Next.js Image component.
```

3. **Add images** (if needed):

```bash
mkdir -p public/blog/images/my-new-post
# Add your images to this folder
```

4. **Preview your post**:

```bash
yarn dev
# Visit http://localhost:3000/blog/my-new-post
```

5. **Publish**:

Set `published: true` in frontmatter and deploy.

### Blog Post Best Practices

#### Frontmatter Guidelines

- **Title**: 50-60 characters for best SEO
- **Slug**: lowercase, hyphen-separated, matches filename
- **Date**: Use ISO 8601 format (YYYY-MM-DDTHH:mm:ss.sssZ)
- **Excerpt**: 150-200 characters, compelling summary
- **Category**: Choose one that best fits your content
- **Tags**: 3-5 relevant tags for discoverability
- **Cover Image**: 1200x630px for optimal social sharing

#### Content Guidelines

- Use descriptive headings (H2, H3) for structure
- Include code examples for technical posts
- Add alt text to all images
- Link to related posts and resources
- Break up long paragraphs
- Use bullet points and lists
- Include a clear conclusion

#### SEO Tips

- Include target keywords naturally
- Use internal links to other posts
- Add structured data (automatically handled)
- Optimize images (use Next.js Image)
- Write compelling excerpts

### Managing Blog Content

#### Draft Posts

Set `published: false` in frontmatter to create drafts. They won't appear in production builds.

#### Featured Posts

Set `featured: true` to highlight important posts. Featured posts appear first in listings.

#### Updating Posts

1. Edit the MDX file
2. Update `lastModified` date in frontmatter
3. Rebuild and redeploy

#### Deleting Posts

1. Delete the MDX file from `/content/blog/`
2. Remove associated images from `/public/blog/images/`
3. Rebuild to update the RSS feed

### Categories and Tags

**Available Categories:**
- Tutorial - Step-by-step guides
- News - Product updates and announcements
- DeFi - DeFi insights and analysis
- Technical - Deep technical articles
- Updates - General updates

**Tag Best Practices:**
- Use existing tags when possible
- Keep tags consistent (check other posts)
- Use title case for multi-word tags
- Maximum 5 tags per post

### RSS Feeds

RSS feeds are automatically generated during build:

- RSS 2.0: `/rss.xml`
- Atom 1.0: `/atom.xml`
- JSON Feed: `/feed.json`

Add to your site header:
```html
<link rel="alternate" type="application/rss+xml" title="DeFindex Blog" href="/rss.xml" />
```

### Blog Routes

- `/blog` - Main blog listing with search
- `/blog/[slug]` - Individual blog post
- `/blog/category/[category]` - Posts by category
- `/blog/tag/[tag]` - Posts by tag

### Customization

#### Styling

Blog components use the DeFindex design system:
- Colors: Cyan, Lime, Purple gradients
- Typography: Familjen Grotesk (headings), Inter (body)
- Effects: Glassmorphism, backdrop blur

To customize, edit:
- `/src/components/blog/MDXComponents.tsx` - MDX component styles
- `/src/components/blog/BlogCard.tsx` - Post card design
- `/src/components/blog/BlogPost.tsx` - Post layout

#### Components

Custom MDX components available:
- Headings (h1-h6) - With gradient text
- Code blocks - With syntax highlighting
- Images - Optimized with Next.js Image
- Links - With external link handling
- Tables - Styled tables
- Blockquotes - Custom styling

### Troubleshooting

#### Images not loading

Check that:
- Image path is correct
- Image domain is in `next.config.mjs` under `images.remotePatterns`
- Image file exists in `/public/blog/images/`

#### Post not appearing

Verify:
- `published: true` in frontmatter
- Frontmatter validates (check build logs)
- MDX file is in `/content/blog/`
- File extension is `.mdx`

#### Build errors

Common issues:
- Invalid frontmatter (check Zod schema in `/src/lib/blog-types.ts`)
- Missing required fields
- Invalid date format
- Syntax errors in MDX content

Run build to see detailed errors:
```bash
yarn build
```

## Project Structure

```
/content/blog/              # Blog posts (MDX)
/docs/                      # Documentation
/public/                    # Static assets
  /blog/images/            # Blog images
  /images/                 # General images
/src/
  /app/                    # Next.js App Router pages
    /api/                  # API routes
    /blog/                 # Blog routes
  /components/             # React components
    /blog/                 # Blog-specific components
    /common/               # Shared components
    /globals/              # Global components (nav, footer)
  /constants/              # Constants and config
  /context/                # React context
  /lib/                    # Utilities
    blog.ts                # Blog utilities
    blog-types.ts          # Blog TypeScript types
    generate-rss.ts        # RSS feed generator
  /styles/                 # Global styles
```

## Environment Variables

Create a `.env.local` file:

```env
# Site URL (for RSS feeds and metadata)
NEXT_PUBLIC_SITE_URL=https://defindex.io

# Add other environment variables as needed
```

## Deployment

### Vercel (Recommended)

1. Push to GitHub
2. Import project in Vercel
3. Configure environment variables
4. Deploy

RSS feeds are automatically generated during build.

### Manual Deployment

```bash
yarn build
yarn start
```

Or deploy the `.next` folder to any Node.js hosting provider.

## Learn More

- [Next.js Documentation](https://nextjs.org/docs)
- [MDX Documentation](https://mdxjs.com/)
- [Tailwind CSS](https://tailwindcss.com/)
- [DeFindex Documentation](https://docs.defindex.io)

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## License

Copyright © 2025 DeFindex. All rights reserved.
