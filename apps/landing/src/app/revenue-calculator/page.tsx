'use client';

import Footer from '@/components/globals/Footer';
import Navbar from '@/components/globals/navbar/Navbar';
import RevenueCalculator from '@/components/globals/RevenueCalculator';
import NavigateTab from '@/context/NavigateTab';
import { useState } from 'react';

export default function RevenueCalculatorPage() {
  const [index, setIndex] = useState(0);

  return (
    <div
      className="min-h-screen bg-[#033036] relative overflow-x-hidden w-full"
      style={{ maxWidth: '100dvw' }}
    >
      <div className="w-full h-full mx-auto">
        <NavigateTab.Provider value={{ index, setIndex }}>
          <Navbar />
            <section className="pb-6 lg:pb-10">
              <RevenueCalculator />
            </section>

            {/* Testimonial Section */}
            <section className="max-w-full pb-24 px-4 md:px-12 lg:px-6 xl:px-16">
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
