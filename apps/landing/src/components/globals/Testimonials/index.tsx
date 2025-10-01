"use client";

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
        description: "Get started with our SDK in under an hour.",
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
        title: "Multi-Token Support",
        description: "Support for USDC, EURC, and other stablecoins.",
    },
];

export default function Testimonials() {
    const [activeCard, setActiveCard] = useState(0);

    return (
        <section id="what-builders-are-doing" className="py-20 px-4 md:px-8 lg:px-16 overflow-hidden w-full">
            <div className="max-w-full mx-auto w-full" style={{maxWidth: 'calc(100vw - 2rem)'}}>
                {/* Section Header */}
                <div className="text-center mb-12 sm:mb-16 mx-[25%]" style={{lineHeight: '2.5em'}}>
                    <GradientText
                        variant="tertiary" 
                        className="
                            font-familjen-grotesk 
                            text-[28px] 
                            sm:text-[38px] 
                            md:text-[48px] 
                            lg:text-[72px] 
                            font-bold 
                            text-white 
                            mb-4
                            "
                        style={{}}
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
                        className="rounded-2xl relative overflow-hidden min-h-[400px] sm:min-h-[500px]"
                        style={{
                            background: 'linear-gradient(100deg, #FFFFFF 0%, #DEC9F4 70%, #024852 100%)',
                        }}
                    >
                        <div className="relative z-10 p-6 sm:p-8 md:p-12 max-w-[60%] md:max-w-[55%]">
                            <div className="inline-block bg-[#D3FFB4] text-[#014751] px-3 py-1 rounded-full text-xs sm:text-sm font-semibold mb-4 sm:mb-6">
                                Case Study
                            </div>
                            <Image src="/images/beans-logo.svg" alt="Beans" width={150} height={48} className="h-8 sm:h-12 mb-4 sm:mb-6" />
                            <h3 className="font-familjen-grotesk text-xl sm:text-2xl md:text-3xl font-semibold text-cyan-950 mb-3 sm:mb-4">
                                $610K in stablecoin deposits within 3 months
                            </h3>
                            <blockquote className="font-inter text-sm sm:text-base md:text-lg text-cyan-950/80 italic">
                                "Our users don't care about blockchain — they care about saving and sending money safely. DeFindex vaults fit that promise: tested, reliable components that help them grow stablecoin savings inside the Beans wallet"
                            </blockquote>
                            <cite className="font-inter text-sm sm:text-base text-cyan-950 not-italic block mt-3 sm:mt-4">
                                Wouter, CMO Beans
                            </cite>
                        </div>
                        <div className="absolute bottom-0 right-0 w-[45%] sm:w-[40%] md:w-[35%] h-full flex items-end justify-end">
                            <Image src="/images/testimonial-mockup.png" alt="Beans App Mockup" width={500} height={600} className="w-full h-auto max-h-full object-contain object-bottom" />
                        </div>
                    </div>
                </div>

                {/* Testimonial 2 - Soroswap */}
                <div className="mb-12 sm:mb-16">
                    <div className="bg-[#D3FFB4] rounded-2xl p-6 sm:p-8 md:p-12 grid grid-cols-1 md:grid-cols-2 gap-6 sm:gap-8 items-center">
                        <div className="md:order-2">
                            <div className="inline-block bg-[#033036] text-[#D3FFB4] px-3 py-1 rounded-full text-xs sm:text-sm font-semibold mb-4 sm:mb-6">
                                Case Study
                            </div>
                            <Image src="/images/soroswap-logo.svg" alt="Soroswap" width={150} height={48} className="h-8 sm:h-12 mb-4 sm:mb-6" />
                            <h3 className="font-familjen-grotesk text-xl sm:text-2xl md:text-3xl font-semibold text-cyan-950 mb-3 sm:mb-4">
                                Integrated in under 1 week
                            </h3>
                            <blockquote className="font-inter text-sm sm:text-base md:text-lg text-cyan-950/80 italic">
                                "With DeFindex, we unlock new TVL by giving our users simple, automated ways to grow their stablecoins. It keeps them engaged longer — without adding complexity for our team."
                            </blockquote>
                            <cite className="font-inter text-sm sm:text-base text-cyan-950 not-italic block mt-3 sm:mt-4">
                                Esteban, CEO Soroswap
                            </cite>
                        </div>
                        <div className="md:order-1 flex justify-center">
                            <Image src="/images/soroswap-preview.png" alt="Soroswap App Mockup" width={600} height={400} className="max-w-full h-auto rounded-lg" style={{filter: 'drop-shadow(5px 5px 10px rgba(0, 0, 0, 0.3))'}} />
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