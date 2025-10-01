"use client";

import CTAButton from "@/components/common/CTAButton";
import GradientText from "@/components/common/GradientText";
import { CONTAINER_MAX_WIDTH } from "@/constants/design";

export default function OurTeam() {
    return (
        <section id="built-by-palta-labs" className="py-20 px-4 md:px-8 lg:px-16 overflow-hidden w-full">
            <div className="max-w-full mx-auto w-full" style={{maxWidth: CONTAINER_MAX_WIDTH}}>
                <div className="rounded-3xl p-6 sm:p-8 md:p-16 text-center bg-[#DEC9F4]">
                    <GradientText
                        as="h2"
                        variant="purple"
                        className="font-familjen-grotesk text-[28px] sm:text-[38px] md:text-[56px] lg:text-[72px] font-normal leading-tight mb-6 sm:mb-8 px-4"
                        style={{fontWeight: 700}}
                    >
                        Built by Palta Labs
                    </GradientText>

                    <p className="font-inter text-sm sm:text-base md:text-lg text-white/80 max-w-4xl mx-auto mb-6 sm:mb-8 leading-relaxed px-4">
                        A team of DeFi builders focused on making stablecoin yield accessible, automated, and safe.
                        We're committed to transparency, security, and helping developers build the future of decentralized finance.
                    </p>

                    <CTAButton
                        variant="secondary"
                        href="https://paltalabs.io"
                        target="_blank"
                        rel="noopener noreferrer"
                        className="mx-[auto] min-w-[200px] max-w-[300px]"
                    >
                        Learn more about Palta Labs
                    </CTAButton>
                </div>
            </div>
        </section>
    );
}