'use client';

import { useState } from 'react';
import Footer from '@/components/globals/Footer';
import Navbar from '@/components/globals/navbar/Navbar';
import VaultsTable from '@/components/vaults/VaultsTable';
import StrategiesTable from '@/components/vaults/StrategiesTable';
import Image from 'next/image';

type ActiveTab = 'partners' | 'strategies';

export default function VaultsPage() {
  const [activeTab, setActiveTab] = useState<ActiveTab>('partners');

  return (
    <div className="min-h-screen w-full bg-black relative z-0 overflow-x-hidden">
      <Image
        width={1440}
        height={6797}
        className="w-full h-full inset-0 absolute -z-10 object-cover"
        src="/images/web-background.png"
        alt=""
      />
      <Navbar />
      <main className="container mx-auto max-w-6xl px-4 sm:px-6 py-12 sm:py-16 lg:py-20 mt-20">
        <header className="mb-8 lg:mb-12">
          <h1 className="font-manrope font-bold text-3xl lg:text-4xl text-white mb-4">
            DeFindex Vaults
          </h1>
          <p className="text-white/70 text-lg max-w-2xl">
            Explore our curated selection of yield-generating vaults built on
            Stellar. Each vault is designed to optimize returns while managing
            risk.
          </p>
        </header>

        {/* Tab bar */}
        <div className="flex gap-2 mb-6">
          <button
            onClick={() => setActiveTab('partners')}
            className={`px-5 py-2 rounded-full text-sm font-manrope font-semibold border transition-colors ${
              activeTab === 'partners'
                ? 'bg-lime-200/10 text-lime-200 border-lime-200/30'
                : 'text-white/50 border-transparent hover:text-white/80'
            }`}
          >
            Partners
          </button>
          <button
            onClick={() => setActiveTab('strategies')}
            className={`px-5 py-2 rounded-full text-sm font-manrope font-semibold border transition-colors ${
              activeTab === 'strategies'
                ? 'bg-lime-200/10 text-lime-200 border-lime-200/30'
                : 'text-white/50 border-transparent hover:text-white/80'
            }`}
          >
            Strategies
          </button>
        </div>

        {activeTab === 'partners' ? <VaultsTable /> : <StrategiesTable />}
      </main>
      <Footer />
    </div>
  );
}
