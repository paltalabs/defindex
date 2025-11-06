"use client";
import GradientText from "@/components/common/GradientText";
import Unit from "@/components/globals/Frequently/Unit";
import Link from "next/link";
import { FiExternalLink } from "react-icons/fi";

export default function Frequently() {
    return (
        <section id="faq" className="py-20 px-4 md:px-8 lg:px-16 overflow-hidden w-full">
            <div className="max-w-full mx-auto w-full" style={{maxWidth: 'calc(100vw - 2rem)'}}>
                <div className="grid grid-cols-1 lg:grid-cols-2 gap-8 sm:gap-12">
                    {/* Left side - Titles */}
                    <div className="px-4">
                        <h2 className="font-familjen-grotesk text-[28px] sm:text-[38px] md:text-[48px] lg:text-[48px] font-normal text-[#D3FFB4] mb-4 sm:mb-6">
                            FAQ
                        </h2>
                        <GradientText
                            as="h3"
                            variant="secondary"
                            className="font-familjen-grotesk text-xl sm:text-2xl md:text-3xl font-semibold text-white mb-3 sm:mb-4 mr-32">
                            Still have questions?
                        </GradientText>
                        <p className="font-inter sm:text-lg text-white/80 leading-relaxed" style={{fontWeight: 400,}}>
                            Find answers to common questions about integrating DeFindex
                        </p>
                    </div>

                    {/* Right side - Accordion */}
                    <div className="flex flex-col gap-3 sm:gap-4">
                        <Unit
                            isOpen={true}
                            title="How quickly can I integrate DeFindex into my app?"
                            description="You can integrate DeFindex in under a week using our comprehensive SDK. Our documentation includes step-by-step guides and code examples to get you started quickly."
                        />
                        <Unit
                            title="How do revenue shares work?"
                            description="Partners choose any percentage of the revenue that they want to charge their users. Then, DeFindex takes an agreed percentage of what the partners charge. For example, if a partner charges 50% of the revenue, and DeFindex takes 30% of that, the partner will receive 35% of the revenue, and DeFindex will receive 15%."
                        />
                        <Unit
                            title="Is DeFindex secure and audited?"
                            description={
                                <>
                                    Yes, all our smart contracts have been audited by leading security firms. We provide full transparency with open-source code and detailed audit reports. you can view our latest audit report{" "}
                                    <Link
                                        href="https://github.com/paltalabs/defindex/blob/main/audits/2025_03_18_ottersec_defindex_audit.pdf"
                                        target="_blank"
                                        rel="noopener noreferrer"
                                        className="inline-flex items-center gap-1 underline hover:opacity-80 transition-opacity"
                                    >
                                        here
                                        <FiExternalLink className="w-4 h-4" />
                                    </Link>
                                </>
                            }
                        />
                        <Unit
                            title="What kind of yields can users expect?"
                            description="Yields vary based on market conditions and strategy performance, typically ranging from 5-30% APY. All returns are clearly displayed with historical performance data."
                        />
                        <Unit
                            title="Do you provide developer support?"
                            description="Absolutely! We are more than just a protocol, we are your partners in growth. We provide technical support to help you integrate DeFindex into your platform, and we also offer co-marketing support to help you promote your services to your users. Join our Discord community to get faster support and connect with other developers and partners."
                        />
                    </div>
                </div>
            </div>
        </section>
    );
}