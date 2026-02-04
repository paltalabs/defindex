"use client";
import Image from "next/image";
import Link from "next/link";

export default function Footer() {
    return (
        <footer className="bg-cyan-900 relative overflow-hidden w-full">
            {/* Background assets */}
            <div className="absolute inset-0">
                <Image
                    src="/images/footer-background-asset.png"
                    alt="DeFindex Stellar ecosystem footer background"
                    fill
                    className="object-cover opacity-30"
                />
            </div>

            <div className="relative max-w-7xl mx-auto px-4 md:px-8 lg:px-16 py-12 sm:py-16 w-full">
                <div className="flex flex-col lg:flex-row items-start justify-between">
                    {/* Left side - Stickers */}
                    <div className="flex-shrink-0 mb-8 lg:mb-0">
                        <Image
                            src="/images/footer-stickers.svg"
                            alt=""
                            width={600}
                            height={300}
                            className="w-full sm:w-auto h-auto max-w-[280px] sm:max-w-md lg:max-w-lg"
                        />
                    </div>

                    {/* Right side - Links and social icons */}
                    <div className="flex flex-col lg:flex-row gap-8 sm:gap-12 lg:gap-16">
                        {/* Links columns */}
                        <div className="flex gap-8 sm:gap-12">
                            {/* Links column */}
                            <div className="flex flex-col gap-3 sm:gap-4">
                                <h3 className="font-manrope font-bold text-sm sm:text-sm text-white mb-1 sm:mb-2">
                                    Links
                                </h3>
                                <Link
                                    href="/"
                                    className="font-manrope text-xs sm:text-sm text-white hover:text-lime-200 transition-colors duration-normal"
                                >
                                    Home
                                </Link>
                                <Link
                                    href="/#why-integrate-yield"
                                    className="font-manrope text-xs sm:text-sm text-white hover:text-lime-200 transition-colors duration-normal"
                                >
                                    Why Integrate
                                </Link>
                                <Link
                                    href="/#what-builders-are-doing"
                                    className="font-manrope text-xs sm:text-sm text-white hover:text-lime-200 transition-colors duration-normal"
                                >
                                    Case Studies
                                </Link>
                                <Link
                                    href="/revenue-calculator"
                                    className="font-manrope text-xs sm:text-sm text-white hover:text-lime-200 transition-colors duration-normal"
                                >
                                    Partner Revenue
                                </Link>
                                <Link
                                    href="/blog"
                                    className="font-manrope text-xs sm:text-sm text-white hover:text-lime-200 transition-colors duration-normal"
                                >
                                    Blog
                                </Link>
                                <Link
                                    href="https://docs.defindex.io"
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    className="font-manrope text-xs sm:text-sm text-white hover:text-lime-200 transition-colors duration-normal"
                                >
                                    Documentation
                                </Link>
                                <Link
                                    href="/privacy-policy"
                                    className="font-manrope text-xs sm:text-sm text-white hover:text-lime-200 transition-colors duration-normal"
                                >
                                    Privacy Policy
                                </Link>
                                <Link
                                    href="/tos"
                                    className="font-manrope text-xs sm:text-sm text-white hover:text-lime-200 transition-colors duration-normal"
                                >
                                    Terms of Service
                                </Link>
                                <Link
                                    href="/tos-raffle"
                                    className="font-manrope text-xs sm:text-sm text-white hover:text-lime-200 transition-colors duration-normal"
                                >
                                    Raffle Terms
                                </Link>
                            </div>

                            {/* Information column */}
                            <div className="flex flex-col gap-3 sm:gap-4">
                                <h3 className="font-manrope font-bold text-sm sm:text-sm text-white mb-1 sm:mb-2">
                                    Information
                                </h3>
                                <Link
                                    href="#faq"
                                    className="font-manrope text-xs sm:text-sm text-white hover:text-lime-200 transition-colors duration-normal"
                                >
                                    FAQ
                                </Link>
                                <Link
                                    href="mailto:dev@paltalabs.io"
                                    className="font-manrope text-xs sm:text-sm text-white hover:text-lime-200 transition-colors duration-normal"
                                >
                                    Contact Us
                                </Link>
                                <Link
                                    href="https://paltalabs.io/"
                                    className="font-manrope text-xs sm:text-sm text-white hover:text-lime-200 transition-colors duration-normal"
                                >
                                    About Us
                                </Link>
                            </div>
                        </div>

                        {/* Social icons */}
                        <div className="flex gap-3 sm:gap-4 lg:flex-col lg:justify-end">
                            <Link
                                href="https://x.com/defindex_"
                                target="_blank"
                                rel="noopener noreferrer"
                                aria-label="Follow DeFindex on X/Twitter for Stellar ecosystem updates"
                                className="w-8 h-8 sm:w-10 sm:h-10 lg:w-8 lg:h-8 bg-white rounded-full flex items-center justify-center hover:scale-110 transition-transform duration-normal"
                            >
                                <img src="/images/icon-x-black.svg" alt="X/Twitter" className="w-3 h-3 sm:w-4 sm:h-4" />
                            </Link>
                            <Link
                                href="https://discord.gg/CUC26qUTw7"
                                target="_blank"
                                rel="noopener noreferrer"
                                aria-label="Join DeFindex Discord community for Stellar DeFi discussions"
                                className="w-8 h-8 sm:w-10 sm:h-10 lg:w-8 lg:h-8 bg-white rounded-full flex items-center justify-center hover:scale-110 transition-transform duration-normal"
                            >
                                <img src="/images/icon-discord-black.svg" alt="Discord" className="w-3 h-3 sm:w-4 sm:h-4" />
                            </Link>
                            <Link
                                href="https://www.linkedin.com/company/defindex"
                                target="_blank"
                                rel="noopener noreferrer"
                                aria-label="Connect with DeFindex on LinkedIn"
                                className="w-8 h-8 sm:w-10 sm:h-10 lg:w-8 lg:h-8 bg-white rounded-full flex items-center justify-center hover:scale-110 transition-transform duration-normal"
                            >
                                <img src="/images/icon-linkedin-black.svg" alt="LinkedIn" className="w-3 h-3 sm:w-4 sm:h-4" />
                            </Link>
                            <Link
                                href="https://github.com/paltalabs/defindex"
                                target="_blank"
                                rel="noopener noreferrer"
                                aria-label="View DeFindex Stellar SDK repository on GitHub"
                                className="w-8 h-8 sm:w-10 sm:h-10 lg:w-8 lg:h-8 bg-white rounded-full flex items-center justify-center hover:scale-110 transition-transform duration-normal"
                            >
                                <img src="/images/icon-github-black.svg" alt="GitHub" className="w-3 h-3 sm:w-4 sm:h-4" />
                            </Link>
                            <Link
                                href="https://dune.com/paltalabs/defindex"
                                target="_blank"
                                rel="noopener noreferrer"
                                aria-label="View DeFindex analytics dashboards on Dune"
                                className="w-8 h-8 sm:w-10 sm:h-10 lg:w-8 lg:h-8 bg-white rounded-full flex items-center justify-center hover:scale-110 transition-transform duration-normal"
                            >
                                <img src="/images/dune-logo.svg" alt="Dune Analytics" className="w-4 h-4 sm:w-5 sm:h-5" />
                            </Link>
                        </div>
                    </div>
                </div>

                {/* Stellar ecosystem mention */}
                <div className="mt-8 pt-6 border-t border-white/10 text-center">
                    <p className="font-manrope text-xs sm:text-sm text-white/70">
                        Built on <span className="text-lime-200 font-semibold">Stellar Network</span> | Powered by <span className="text-lime-200 font-semibold">Soroban</span>
                    </p>
                </div>
            </div>
        </footer>
    );
}