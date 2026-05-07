'use client';

import { useEffect, useRef, useState } from 'react';
import type { SortKey } from '@/types/vault.types';

interface FilterBarProps {
  onSearch: (q: string) => void;
  sort: SortKey;
  onSort: (s: SortKey) => void;
}

const SORT_OPTIONS: SortKey[] = ['TVL', 'APY', 'Name'];

export default function FilterBar({ onSearch, sort, onSort }: FilterBarProps) {
  const [inputValue, setInputValue] = useState('');
  const [focusedOpt, setFocusedOpt] = useState<string | null>(null);
  const debounceRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  useEffect(() => {
    if (debounceRef.current) clearTimeout(debounceRef.current);
    debounceRef.current = setTimeout(() => onSearch(inputValue), 150);
    return () => {
      if (debounceRef.current) clearTimeout(debounceRef.current);
    };
  }, [inputValue, onSearch]);

  return (
    <div style={{ display: 'flex', alignItems: 'center', gap: 12, marginBottom: 16 }}>
      {/* Search */}
      <div
        style={{
          flex: 1,
          display: 'flex',
          alignItems: 'center',
          gap: 10,
          padding: '10px 14px',
          borderRadius: 12,
          background: 'rgba(29,57,62,.4)',
          border: '1px solid rgba(193,200,201,.12)',
        }}
      >
        <svg
          width="14"
          height="14"
          viewBox="0 0 24 24"
          fill="none"
          stroke="rgba(255,255,255,.4)"
          strokeWidth="2"
          strokeLinecap="round"
          strokeLinejoin="round"
          aria-hidden="true"
        >
          <circle cx="11" cy="11" r="8" />
          <path d="m21 21-4.3-4.3" />
        </svg>
        <input
          type="search"
          placeholder="Search partners or strategies…"
          value={inputValue}
          onChange={e => setInputValue(e.target.value)}
          style={{
            all: 'unset',
            flex: 1,
            fontSize: 13,
            color: '#fff',
            fontFamily: 'Inter Tight, sans-serif',
          }}
          aria-label="Search"
        />
      </div>

      {/* Sort chips */}
      {SORT_OPTIONS.map(opt => {
        const active = sort === opt;
        return (
          <button
            key={opt}
            onClick={() => onSort(opt)}
            style={{
              all: 'unset',
              cursor: 'pointer',
              padding: '8px 14px',
              borderRadius: 999,
              border: '1px solid rgba(193,200,201,.12)',
              background: active ? 'rgba(217,249,157,.06)' : 'rgba(29,57,62,.4)',
              color: active ? '#D9F99D' : 'rgba(255,255,255,.6)',
              fontSize: 12,
              fontWeight: 600,
              transition: 'all 200ms cubic-bezier(0.4,0,0.2,1)',
              outline: focusedOpt === opt ? '4px solid rgba(217,249,157,.40)' : 'none',
            }}
            onFocus={() => setFocusedOpt(opt)}
            onBlur={() => setFocusedOpt(null)}
          >
            ↓ {opt}
          </button>
        );
      })}
    </div>
  );
}
