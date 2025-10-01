"use client";
import Link from "next/link";
import Image from "next/image";

export default function Footer() {
    return (
        <footer className="bg-[#033036] relative overflow-hidden w-full">
            {/* Background assets */}
            <div className="absolute inset-0">
                <Image
                    src="/images/footer-background-asset.png"
                    alt=""
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
                                    href="https://docs.defindex.io/wallet-developer/introduction"
                                    className="font-manrope text-xs sm:text-sm text-white hover:text-[#D3FFB4] transition-colors duration-200"
                                >
                                    For Wallets Builders
                                </Link>
                                <Link
                                    href="https://docs.defindex.io/defi-developers/developer-introduction"
                                    className="font-manrope text-xs sm:text-sm text-white hover:text-[#D3FFB4] transition-colors duration-200"
                                >
                                    For Developers
                                </Link>
                                <Link
                                    href="/privacy-policy"
                                    className="font-manrope text-xs sm:text-sm text-white hover:text-[#D3FFB4] transition-colors duration-200"
                                >
                                    Privacy Policy
                                </Link>
                            </div>

                            {/* Information column */}
                            <div className="flex flex-col gap-3 sm:gap-4">
                                <h3 className="font-manrope font-bold text-sm sm:text-sm text-white mb-1 sm:mb-2">
                                    Information
                                </h3>
                                <Link
                                    href="#faq"
                                    className="font-manrope text-xs sm:text-sm text-white hover:text-[#D3FFB4] transition-colors duration-200"
                                >
                                    FAQ
                                </Link>
                                <Link
                                    href="mailto:dev@paltalabs.io"
                                    className="font-manrope text-xs sm:text-sm text-white hover:text-[#D3FFB4] transition-colors duration-200"
                                >
                                    Contact Us
                                </Link>
                                <Link
                                    href="https://paltalabs.io/"
                                    className="font-manrope text-xs sm:text-sm text-white hover:text-[#D3FFB4] transition-colors duration-200"
                                >
                                    About Us
                                </Link>
                            </div>
                        </div>

                        {/* Social icons */}
                        <div className="flex gap-3 sm:gap-4 lg:flex-col lg:justify-end">
                            <Link
                                href="https://x.com/defindex_"
                                className="w-8 h-8 sm:w-10 sm:h-10 lg:w-8 lg:h-8 bg-white rounded-full flex items-center justify-center hover:scale-110 transition-transform duration-200"
                            >
                                <img src="/images/icon-x-black.svg" alt="X/Twitter" className="w-3 h-3 sm:w-4 sm:h-4" />
                            </Link>
                            <Link
                                href="https://discord.gg/CUC26qUTw7"
                                className="w-8 h-8 sm:w-10 sm:h-10 lg:w-8 lg:h-8 bg-white rounded-full flex items-center justify-center hover:scale-110 transition-transform duration-200"
                            >
                                <img src="/images/icon-discord-black.svg" alt="Discord" className="w-3 h-3 sm:w-4 sm:h-4" />
                            </Link>
                            <Link
                                href="https://www.linkedin.com/company/paltalabs"
                                className="w-8 h-8 sm:w-10 sm:h-10 lg:w-8 lg:h-8 bg-white rounded-full flex items-center justify-center hover:scale-110 transition-transform duration-200"
                            >
                                <img src="/images/icon-linkedin-black.svg" alt="LinkedIn" className="w-3 h-3 sm:w-4 sm:h-4" />
                            </Link>
                            <Link
                                href="https://github.com/paltalabs/defindex"
                                className="w-8 h-8 sm:w-10 sm:h-10 lg:w-8 lg:h-8 bg-white rounded-full flex items-center justify-center hover:scale-110 transition-transform duration-200"
                            >
                                <img src="/images/icon-github-black.svg" alt="GitHub" className="w-3 h-3 sm:w-4 sm:h-4" />
                            </Link>
                        </div>
                    </div>
                </div>
            </div>
        </footer>
    );
}