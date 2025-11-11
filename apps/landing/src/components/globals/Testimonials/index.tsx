"use client";

import CaseStudyButton from "@/components/common/CaseStudyButton";
import GradientText from "@/components/common/GradientText";
import InteractiveCard from "@/components/common/InteractiveCard";
import { ICON_FILTER } from "@/constants/design";
import Image from "next/image";
import { useState } from "react";

const features = [
    {
        id: 0,
        icon: "/images/network-wired.svg",
        title: "Simple Integration",
        description: "Get started with our API in under an hour.",
    },
    {
        id: 1,
        icon: "/images/shield-lock.svg",
        title: "Built-in Safety",
        description: "Audited smart contracts and risk management.",
    },
    {
        id: 2,
        icon: "/images/icon-exchange.svg",
        title: "Multi-Token Vault",
        description: "Support for USDC, EURC, and any asset. Support major Stellar protocols.",
    },
];

export default function Testimonials() {
    const [activeCard, setActiveCard] = useState(0);

    return (
        <section id="what-builders-are-doing" className="py-10 px-4 md:px-8 lg:px-16 overflow-hidden w-full">
            <div className="max-w-full mx-auto w-full" style={{maxWidth: 'calc(100vw - 2rem)'}}>
                {/* Section Header */}
                <div className="text-center mb-12 sm:mb-16 mx-4">
                    <GradientText
                        variant="tertiary" 
                        className="
                            font-familjen-grotesk 
                            text-[32px] 
                            sm:text-[46px] 
                            md:text-[52px] 
                            lg:text-[72px] 
                            font-bold 
                            text-white 
                            mb-4
                            "
                        >
                        What builders are doing with DeFindex
                    </GradientText>
                    <p className="font-inter sm:text-[24px] text-white/80 max-w-2xl mx-auto" style={{fontWeight: 400, lineHeight: '1.5em'}}>
                        Real-world examples of yield integration in action.
                    </p>
                </div>

                {/* Testimonial 1 - Beans */}
                <div className="mb-8 sm:mb-12">
                    <div
                        className="rounded-2xl p-6 sm:p-8 md:p-12 pb-0 sm:pb-0 md:pb-0 lg:pb-0 grid grid-cols-1 lg:grid-cols-2 gap-6 sm:gap-8 items-stretch"
                        style={{
                            background: 'linear-gradient(100deg, #FFFFFF 0%, #DEC9F4 70%, #024852 100%)',
                        }}
                    >
                        <div className="md:order-0 pb-12">
                            <CaseStudyButton
                                variant="light"
                                href="/blog/beans-case-study"
                                className="mb-4 sm:mb-6"
                            />
                            <Image src="/images/beans-logo.svg" alt="Beans logo" width={150} height={48} className="h-auto mb-4 sm:mb-6 aspect-auto" />
                            <h3 className="font-familjen-grotesk text-lg sm:text-xl md:text-3xl font-semibold text-cyan-950 mb-3 sm:mb-4">
                                $610K in stablecoin deposits within 3 months
                            </h3>
                            <blockquote className="font-inter text-sm sm:text-base md:text-lg text-cyan-950/80 italic">
                                "Our users don't care about blockchain — they care about saving and sending money safely. DeFindex vaults fit that promise: tested, reliable components that help them grow stablecoin savings inside the Beans wallet"
                            </blockquote>
                            <cite className="font-inter text-sm sm:text-base text-cyan-950 not-italic block mt-3 sm:mt-4">
                                Wouter, CEO & Founder, Beans.
                            </cite>
                        </div>
                        <div className="md:order-1 flex items-end justify-center w-full">
                            <Image src="/images/testimonial-mockup.webp" alt="Beans App Mockup" width={500} height={600} className="w-full h-auto max-h-full object-contain object-bottom" />

                        </div>
                    </div>
                </div>

                {/* Testimonial 2 - Soroswap */}
                <div className="mb-12 sm:mb-16">
                    <div className="bg-[#D3FFB4] rounded-2xl p-6 sm:p-8 md:p-12 grid grid-cols-1 md:grid-cols-2 gap-6 sm:gap-8 items-center">
                        <div className="md:order-2">
                            <CaseStudyButton
                                variant="dark"
                                href="https://docs.defindex.io/use-cases/seevcash"
                                className="mb-4 sm:mb-6"
                            />
                            <Image src="/images/seevcash-logo.svg" alt="Seevcash" width={185} height={0} className="h-auto mb-4 sm:mb-6 aspect-auto" />
                            <h3 className="font-familjen-grotesk text-[46px] sm:text-[48px] md:text-[56px] lg:text-3xl font-semibold text-cyan-950 mb-3 sm:mb-4">
                                "Users love check their growing balances."
                            </h3>
                            <blockquote className="font-inter text-sm sm:text-base md:text-lg text-cyan-950/80 italic">
                            </blockquote>
                            <cite className="font-inter text-sm sm:text-base text-cyan-950 not-italic block mt-3 sm:mt-4">
                                Dawuda Iddrisu, CEO Seevcash.
                            </cite>
                        </div>
                        <div className="md:order-1 flex justify-center ">
                            <Image src="/images/seevcash-mockup.webp" alt="Seevcash App Mockup" width={200} height={600} className="max-h-full w-auto max-w-[80%] rounded-lg" style={{ filter: 'drop-shadow(5px 5px 10px rgba(0, 0, 0, 0.1))', transform: 'rotate(-10deg)' }} />
                        </div>
                    </div>
                </div>

                {/* 3 Key Points Section */}
                <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 rounded-2xl overflow-hidden border border-[#D3FFB4]">
                    {features.map((feature, index) => (
                        <InteractiveCard
                            key={feature.id}
                            id={feature.id}
                            isActive={activeCard === feature.id}
                            onClick={() => setActiveCard(feature.id)}
                            icon={feature.icon}
                            iconAlt={feature.title}
                            title={feature.title}
                            description={feature.description}
                            activeGradient="transparent"
                            inactiveBackground="bg-[#033036]"
                            iconClassName="w-12 h-12 sm:w-16 sm:h-16"
                            iconFilter={ICON_FILTER.dark}
                            textAlign="center"
                            cardClassName={`${index === 2 ? 'sm:col-span-2 md:col-span-1' : ''} ${activeCard === feature.id ? 'bg-[#D3FFB4]' : ''}`}
                        />
                    ))}
                </div>
            </div>
        </section>
    );
}