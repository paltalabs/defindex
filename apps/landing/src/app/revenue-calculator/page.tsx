'use client';

import Footer from '@/components/globals/Footer';
import Navbar from '@/components/globals/navbar/Navbar';
import RevenueCalculator from '@/components/globals/RevenueCalculator';
import NavigateTab from '@/context/NavigateTab';
import Image from 'next/image';
import { useState } from 'react';

export default function RevenueCalculatorPage() {
  const [index, setIndex] = useState(0);

  return (
    <div
      className="min-h-screen bg-[#033036] relative overflow-x-hidden w-full"
      style={{ maxWidth: '100dvw' }}
    >
      {/* Background image */}
      <div className="absolute inset-0 -z-10">
        <Image
          src="/images/hero-background.png"
          alt="DeFindex background pattern"
          fill
          className="object-cover opacity-20"
        />
      </div>

      <div className="w-full px-4 sm:px-6 lg:px-8 xl:px-12 mx-auto">
        <NavigateTab.Provider value={{ index, setIndex }}>
          <Navbar />

          {/* Main Content */}
          <section className="pt-12 pb-6 lg:pt-24 lg:pb-10">
            <RevenueCalculator />
          </section>

          {/* Testimonial Section */}
          <section className="max-w-full pb-24 px-1">
            <div
              className="rounded-2xl p-6 md:p-8 border border-cyan-800/50"
              style={{
                background:
                  'linear-gradient(135deg, rgba(3, 48, 54, 0.6) 0%, rgba(1, 71, 81, 0.3) 100%)',
              }}
            >
              <blockquote className="text-center">
                <p className="text-base md:text-lg lg:text-xl text-white/90 italic mb-4 md:mb-6">
                  &ldquo;With DeFindex, we offer our users a way to earn yield
                  without them ever thinking about blockchain complexity. It
                  just works.&rdquo;
                </p>
                <footer>
                  <cite className="not-italic">
                    <span className="text-lime-200 font-semibold">
                      Beans Wallet
                    </span>
                    <span className="text-white/60 ml-2">
                      — Integration Partner
                    </span>
                  </cite>
                </footer>
              </blockquote>
            </div>
          </section>
        </NavigateTab.Provider>

        <Footer />
      </div>
    </div>
  );
}
