import { getCategoryUrl } from '@/lib/blog-client';
import type { CategoryWithCount } from '@/lib/blog-types';
import Link from 'next/link';

interface CategoryFilterProps {
  categories: CategoryWithCount[];
  currentCategory?: string;
}

/**
 * CategoryFilter component - Displays categories with post counts
 * Allows users to filter posts by category
 *
 * @param categories - Array of categories with post counts
 * @param currentCategory - Currently selected category (if any)
 */
export default function CategoryFilter({ categories, currentCategory }: CategoryFilterProps) {
  return (
    <div className="bg-cyan-950/30 backdrop-blur-sm border border-cyan-800/30 rounded-2xl p-6">
      <h3 className="text-lg font-familjen-grotesk font-bold text-lime-200 mb-4">
        Categories
      </h3>

      <div className="space-y-2">
        {/* All Categories Link */}
        <Link
          href="/blog"
          className={`
            flex items-center justify-between px-4 py-2 rounded-lg
            transition-all
            ${
              !currentCategory
                ? 'bg-lime-200/10 border border-lime-200/50 text-lime-200'
                : 'border border-cyan-800/30 text-cyan-300 hover:border-cyan-700 hover:text-lime-200'
            }
          `}
        >
          <span className="font-inter font-medium">All Articles</span>
          <span className="text-sm font-manrope font-bold">
            {categories.reduce((sum, cat) => sum + cat.count, 0)}
          </span>
        </Link>

        {/* Category Links */}
        {categories.map((category) => {
          const isActive =
            currentCategory?.toLowerCase() === category.name.toLowerCase();

          return (
            <Link
              key={category.slug}
              href={getCategoryUrl(category.name)}
              className={`
                flex items-center justify-between px-4 py-2 rounded-lg
                transition-all
                ${
                  isActive
                    ? 'bg-lime-200/10 border border-lime-200/50 text-lime-200'
                    : 'border border-cyan-800/30 text-cyan-300 hover:border-cyan-700 hover:text-lime-200'
                }
              `}
            >
              <span className="font-inter font-medium">{category.name}</span>
              <span className="text-sm font-manrope font-bold">
                {category.count}
              </span>
            </Link>
          );
        })}
      </div>

      {/* Decorative Element */}
      <div className="mt-6 pt-6 border-t border-cyan-800/30">
        <p className="text-cyan-400 text-sm font-inter text-center">
          Explore articles by category
        </p>
      </div>
    </div>
  );
}
