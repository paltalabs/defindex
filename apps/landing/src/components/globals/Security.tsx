"use client";

import CTAButton from "@/components/common/CTAButton";
import InteractiveCard from "@/components/common/InteractiveCard";
import { GRADIENTS } from "@/constants/design";
import { SecurityFeature } from "@/types";
import { useState } from "react";
import GradientText from "../common/GradientText";

const securityFeatures: SecurityFeature[] = [
    {
        id: 0,
        icon: "/images/icon-audit.svg",
        title: "Audited Contracts",
        description: "Audited smart contracts and transparent performance history",
    },
    {
        id: 1,
        icon: "/images/icon-non-custodial.svg",
        title: "Non-Custodial",
        description: "Avoid custody risk and compliance burden",
    },
    {
        id: 2,
        icon: "/images/icon-diversified.svg",
        title: "Diversified Yields",
        description: "Diversified yield sources and automated rebalancing",
    },
    {
        id: 3,
        icon: "/images/icon-stablecoin-first.svg",
        title: "Support for Any Asset",
        description: "USDC, EURC, and more stellar assets",
    },
];

export default function Security() {
    const [activeCard, setActiveCard] = useState(1);

    return (
        <section id="secure-by-design" className="py-20 px-4 md:px-8 lg:px-16 overflow-hidden w-full">
            <div className="max-w-full mx-auto w-full" style={{maxWidth: 'calc(100vw - 2rem)'}}>
                {/* Title */}
                <div className="text-center mb-16">
                    <GradientText 
                        variant="secondary"
                        className="font-familjen-grotesk text-[28px] sm:text-[38px] md:text-[48px] lg:text-[60px] font-bold px-4">
                        Secure by design
                    </GradientText>

                    {/* Cards Grid - HORIZONTAL LAYOUT */}
                    <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 sm:gap-6 mb-8 sm:mb-12 pt-12">
                        {securityFeatures.map((feature) => (
                            <InteractiveCard
                                key={feature.id}
                                id={feature.id}
                                isActive={activeCard === feature.id}
                                onClick={() => setActiveCard(feature.id)}
                                icon={feature.icon}
                                iconAlt={feature.title}
                                title={feature.title}
                                description={feature.description}
                                activeGradient={GRADIENTS.cardGreen}
                                iconClassName="w-16 h-16 sm:w-20 sm:h-20 md:w-24 md:h-24"
                            />
                        ))}
                    </div>

                    {/* View Audit Report Button */}
                    <CTAButton
                        variant="primary"
                        href="https://github.com/paltalabs/defindex/blob/main/audits/2025_03_18_ottersec_defindex_audit.pdf"
                        target="_blank"
                        rel="noopener noreferrer"
                        className="mx-[auto] min-w-[200px] max-w-[300px]"
                    >
                        View Audit Report
                    </CTAButton>
                </div>
            </div>
        </section>
    );
}