'use client';

import { useState } from 'react';
import { FaXTwitter, FaLinkedin } from 'react-icons/fa6';
import { FiCopy, FiCheck } from 'react-icons/fi';

interface ShareButtonsProps {
  title: string;
  url: string;
}

/**
 * ShareButtons component - Client component for social sharing
 * Handles copying link to clipboard and social media sharing
 */
export default function ShareButtons({ title, url }: ShareButtonsProps) {
  const [copied, setCopied] = useState(false);

  const handleCopyLink = async () => {
    try {
      await navigator.clipboard.writeText(url);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch (error) {
      console.error('Failed to copy:', error);
    }
  };

  return (
    <div className="flex gap-3">
      {/* Twitter/X Share */}
      <a
        href={`https://twitter.com/intent/tweet?text=${encodeURIComponent(title)}&url=${encodeURIComponent(url)}`}
        target="_blank"
        rel="noopener noreferrer"
        className="p-2 bg-cyan-900/50 border border-cyan-800/30 rounded-lg hover:border-lime-200/50 transition-colors"
        aria-label="Share on X/Twitter"
      >
        <FaXTwitter className="w-5 h-5 text-cyan-200" />
      </a>

      {/* LinkedIn Share */}
      <a
        href={`https://www.linkedin.com/sharing/share-offsite/?url=${encodeURIComponent(url)}`}
        target="_blank"
        rel="noopener noreferrer"
        className="p-2 bg-cyan-900/50 border border-cyan-800/30 rounded-lg hover:border-lime-200/50 transition-colors"
        aria-label="Share on LinkedIn"
      >
        <FaLinkedin className="w-5 h-5 text-cyan-200" />
      </a>

      {/* Copy Link */}
      <button
        onClick={handleCopyLink}
        className="p-2 bg-cyan-900/50 border border-cyan-800/30 rounded-lg hover:border-lime-200/50 transition-colors relative"
        aria-label={copied ? 'Link copied!' : 'Copy link'}
      >
        {copied ? (
          <FiCheck className="w-5 h-5 text-lime-200" />
        ) : (
          <FiCopy className="w-5 h-5 text-cyan-200" />
        )}
      </button>
    </div>
  );
}
