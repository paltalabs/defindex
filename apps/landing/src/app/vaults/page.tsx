'use client';

import Footer from '@/components/globals/Footer';
import Navbar from '@/components/globals/navbar/Navbar';
import VaultsTable from '@/components/vaults/VaultsTable';
import Image from 'next/image';

export default function VaultsPage() {
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

        <VaultsTable />
      </main>
      <Footer />
    </div>
  );
}
