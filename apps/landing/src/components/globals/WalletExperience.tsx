import React from "react";
import Link from "next/link";
import Image from "next/image";

function WalletExperience() {
    return (
        <section className="py-12 md:py-20 lg:py-[120px] relative z-0">
            <Image
                width={1442}
                height={647}
                src="/images/CTA section.png"
                className="absolute inset-0 w-full h-full -z-10"
                alt=""
            />

            <div className="container">
                <div className="max-w-[1056px] mx-auto py-10 md:py-16 px-6 sm:px-10 md:px-[85px] rounded-3xl bg-[linear-gradient(115deg,_#033036_16.31%,_#06383D_81.67%)] backdrop-blur-[29px]">
                    <h2 className="text-linear italic mb-6 bg-linear font-bold font-familjen-grotesk text-center text-[36px] sm:text-[40px] leading-none lg:text-xl tracking-[-0.03em]">
                        Explore How DeFindex Can Elevate Your Wallet Experience
                    </h2>
                    <p className="mb-8 font-inter-tight text-center text-[18px] sm:text-[20px] lg:text-lg leading-normal text-white">
                        We&apos;re not live yet, but you can be the first to experience how DeFindex
                        simplifies DeFi
                    </p>
                    <div className="flex sm:flex-row flex-col justify-center gap-2 sm:gap-4">
                        <Link
                            className="rounded-3xl bg-lime-200 px-6 contained-button py-4 lg:py-5 lg:min-h-[60px] leading-none flex gap-2.5 items-center justify-center font-extrabold font-manrope text-[14px] md:leading-none md:text-xs text-cyan-950"
                            href="mailto:dev@paltalabs.io"
                        >
                            Schedule a Demo
                        </Link>
                        <Link
                            href="mailto:dev@paltalabs.io"
                            className="rounded-3xl border border-white outlined-button px-6 leading-none py-4 lg:py-5 lg:min-h-[60px] flex gap-2.5 items-center justify-center font-extrabold font-manrope text-[14px] md:leading-none md:text-xs text-white"
                        >
                            Join the Waitlist
                        </Link>
                    </div>
                </div>
            </div>
        </section>
    );
}

export default WalletExperience;
