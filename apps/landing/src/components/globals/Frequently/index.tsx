import React from "react";
import Unit from "@/components/globals/Frequently/Unit";

function Frequently() {
    return (
        <section className="mb-20 md:mb-[120px]">
            <div className="container">
                <div className="mx-auto max-w-[1216px] grid lg:grid-cols-2">
                    <div className="lg:max-w-[435px] lg:mb-0 mb-12">
                        <h2 className="text-linear leading-[1.03em] mb-3 xl:mb-6 bg-linear font-bold font-familjen-grotesk italic text-[48px] sm:text-[56px] lg:text-xl">
                            Frequently Asked Questions
                        </h2>
                        <p className="font-inter-tight text-[20px] xl:text-lg leading-[1.25em] text-white">
                            DeFindex makes it easy for wallet providers to offer yield-generating accounts to their users through diverse DeFi strategies. It provides seamless integration, security, and transparency, ensuring both developers and users can benefit from passive income and innovative financial tools.
                        </p>
                    </div>
                    <div className="flex flex-col gap-6">
                        <Unit
                            isOpen
                            title="What is DeFindex, and how does it work? "
                            description="DeFindex allows wallet providers to integrate automated, secure, and diversified DeFi strategies into their applications, enabling users to earn passive income from cryptocurrencies. It operates through smart contracts, which handle everything from reinvestment to securing funds."
                        />
                        <Unit
                            title="How can DeFindex benefit wallet builders?"
                            description="Wallet builders can customize portfolios using a variety of DeFi strategies, enhancing user engagement with yield-generating accounts. DeFindex provides easy integration tools that allow users to start earning with just one click."
                        />{" "}
                        <Unit
                            title="Is DeFindex secure and decentralized?"
                            description="Yes, DeFindex operates through secure and transparent smart contracts, ensuring that all transactions are decentralized and under the full control of the user, providing peace of mind to partners and users alike."
                        />
                    </div>
                </div>
            </div>
        </section>
    );
}

export default Frequently;
