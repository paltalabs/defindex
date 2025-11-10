"use client";

import GradientText from "@/components/common/GradientText";
import ScheduleDemoButton from "@/components/common/ScheduleDemoButton";
import { CONTAINER_MAX_WIDTH, ICON_FILTER } from "@/constants/design";
import Image from "next/image";
import Link from "next/link";
import { FiExternalLink } from "react-icons/fi";

export default function Solutions() {
    return (
        <section id="why-integrate-yield" className="py-10 px-4 md:px-8 overflow-hidden w-full">
            {/* Main container with white background and rounded corners */}
            <div className="max-w-full mx-auto bg-white rounded-3xl p-4 sm:p-8 xl:px-32 md:p-24 w-full" style={{zIndex: 1, position: 'relative', maxWidth: CONTAINER_MAX_WIDTH}}>
                {/* Title and Schedule Demo button */}
                <div className="text-center mb-12 sm:mb-16">
                    <GradientText
                        as="h2"
                        variant="green"
                        className="font-familjen-grotesk text-xl sm:text-2xl md:text-3xl lg:text-4xl mb-6 sm:mb-8 mx-4 font-extrabold"
                        style={{lineHeight: '1.0em'}}
                    >
                        Why Integrate Yield with DeFindex
                    </GradientText>
                    <div className="flex flex-col sm:flex-row gap-4 justify-center items-center w-full">
                        <ScheduleDemoButton />
                        <Link
                            href="https://docs.defindex.io/defindex-protocol/what-is-defindex#solutions-for-wallet-builders"
                            target="_blank"
                            rel="noopener noreferrer"
                            aria-label="View DeFindex solutions documentation"
                            className="flex items-center justify-center bg-cyan-900 text-lime-200 font-manrope font-extrabold text-sm rounded-3xl px-6 py-4 sm:py-6 transition-all duration-normal hover:scale-105 hover:bg-cyan-900/90 hover:shadow-lg active:scale-95"
                        >
                            Open documentation
                             <FiExternalLink className="text-lime-200 text-xs lg:text-xs transition-colors duration-normal group-hover:text-lime-200 ml-2" style={{filter: 'drop-shadow(0 2px 4px rgba(0,0,0,0.6))'}} />
                        </Link>
                    </div>
                </div>

                {/* FOR WALLETS Section */}
                <div className="mb-12 sm:mb-20">
                    <div className="text-center mb-8 sm:mb-12 px-4">
                        <h3 className="font-familjen-grotesk text-lg text-cyan-950 mb-3 sm:mb-4" style={{letterSpacing: '0.2em', lineHeight: '2.5em', fontWeight: 400}}>
                            FOR WALLETS
                        </h3>
                        <h4 className="font-familjen-grotesk text-2xl sm:text-2xl md:text-2xl font-semibold text-cyan-950 mb-4 sm:mb-6">
                            Make users stick around
                        </h4>
                        <p className="text-base sm:text-xl text-cyan-950/80 font-normal mx-auto">
                            Give your users reasons to stay engaged with automated yield strategies.
                        </p>
                    </div>

                    {/* Cards Grid */}
                    <div className="grid rounded-2xl grid-cols-1 md:grid-cols-3 border border-orange-500 overflow-hidden">
                        <div className="p-6 md:border-r border-b md:border-b-0 border-orange-500 text-center bg-gradient-card-orange">
                            <div className="mb-6 flex justify-center">
                                <Image src="/images/icon-shield-check.svg" alt="Shield Check" width={48} height={48} className="w-12 h-12" />
                            </div>
                            <p className="font-inter text-cyan-950">
                                Safe, automated ways to grow balances - without users leaving your app.
                            </p>
                        </div>

                        <div className="p-6 md:border-r border-b md:border-b-0 border-orange-500 text-center bg-gradient-card-orange">
                            <div className="mb-6 flex justify-center">
                                <Image src="/images/icon-sack-money.svg" alt="Sack Money" width={48} height={48} className="w-12 h-12" />
                            </div>
                            <p className="font-inter text-cyan-950">
                                Reduce churn in uncertain economies with capital-preserving vaults.
                            </p>
                        </div>

                        <div className="p-6 text-center bg-gradient-card-orange">
                            <div className="mb-6 flex justify-center">
                                <Image src="/images/icon-profit.svg" alt="Revenue Up" width={48} height={48} className="w-12 h-12" />
                            </div>
                            <p className="font-inter text-cyan-950">
                                Capture revenue.
                            </p>
                        </div>
                    </div>
                </div>

                {/* FOR DEFI APPS Section */}
                <div>
                    <div className="text-center mb-8 sm:mb-12 px-4">
                        <h3 className="font-familjen-grotesk text-lg text-cyan-950 mb-3 sm:mb-4" style={{letterSpacing: '0.2em', lineHeight: '2.5em', fontWeight: 400}}>
                            FOR DEFI APPS
                        </h3>
                        <h4 className="font-familjen-grotesk text-2xl sm:text-2xl md:text-2xl font-semibold text-cyan-950 mb-4 sm:mb-6">
                            Unlock new TVL with safer yield
                        </h4>
                        <p className="text-base sm:text-xl text-cyan-950/80 font-normal mx-auto">
                            Attract new users and convert them into active users.
                        </p>
                    </div>

                    {/* Cards Grid */}
                    <div className="grid grid-cols-1 md:grid-cols-3 rounded-2xl border border-brand-primary-dark overflow-hidden">
                        <div className="p-6 text-center md:border-r border-b md:border-b-0 border-brand-primary-dark bg-gradient-card-purple">
                            <div className="mb-6 flex justify-center">
                                <Image src="/images/icon-coin.svg" alt="Stablecoin" width={48} height={48} className="w-12 h-12" style={{filter: ICON_FILTER.green}} />
                            </div>
                            <p className="font-inter text-cyan-950">
                                Stablecoins are the #1 held asset in emerging markets - we help you attract them.
                            </p>
                        </div>

                        <div className="p-6 text-center md:border-r border-b md:border-b-0 border-brand-primary-dark bg-gradient-card-purple">
                            <div className="mb-6 flex justify-center">
                                <Image src="/images/icon-scalability.svg" alt="Scalability" width={48} height={48} className="w-12 h-12" style={{filter: ICON_FILTER.green}} />
                            </div>
                            <p className="font-inter text-cyan-950">
                                Unlock new TVL.
                            </p>
                        </div>

                        <div className="p-6 text-center bg-gradient-card-purple">
                            <div className="mb-6 flex justify-center">
                                <Image src="/images/icon-profit.svg" alt="Revenue Up" width={48} height={48} className="w-12 h-12" style={{filter: ICON_FILTER.green}} />
                            </div>
                            <p className="font-inter text-cyan-950">
                                Increase distribution of your DeFi protocol.
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    );
}