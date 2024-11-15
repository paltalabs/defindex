import React from "react";

const strategies = [
    { id: 1, title: "Diversified DeFi strategies for your users" },
    { id: 2, title: "Secure and automated smart contracts handle everything" },
    { id: 3, title: "Reinvest earnings automatically to maximize returns" },
];

function HowWorks() {
    return (
        <section id="how-it-works" className="mb-16 sm:mb-20 md:mb-24 lg:mb-28 xl:mb-32">
            <div className="container">
                <div className="max-w-[1216px] mx-auto">
                    <div className="flex gap-2 md:gap-4 sm:items-center flex-col mb-7 sm:mb-10 md:mb-16 lg:mb-20">
                        <p className="font-familjen-grotesk uppercase text-left sm:text-center text-[18px] sm:text-[20px] md:text-[22px] lg:text-lg leading-[1.42em] tracking-[-0.03em] text-uppercase text-blue-100">
                            How It Works
                        </p>
                        <h2 className="font-bold font-familjen-grotesk leading-[1.11em] text-[48px] sm:text-[56px] md:text-[64px] lg:text-3xl tracking-[-0.03em] text-linear bg-linear">
                            How DeFindex Works
                        </h2>
                    </div>
                    <div className="flex lg:flex-row lg:gap-0 gap-16 flex-col">
                        <div className="flex-grow">
                            <div className="lg:max-w-80 mb-10 lg:mb-16">
                                <ul className="flex gap-2 lg:gap-8 flex-col">
                                    {strategies.map(({ id, title }) => (
                                        <li key={id}>
                                            <div className="flex gap-4 items-center">
                                                <span className="font-familjen-grotesk text-[#DEC9F4] -translate-y-[0.095em] font-bold text-[40px] sm:text-[48px] lg:text-[64px]">
                                                    {id}.
                                                </span>
                                                <p className="font-inter-tight text-[16pxs] sm:text-[20px] lg:text-lg text-blue-100">
                                                    {title}
                                                </p>
                                            </div>
                                        </li>
                                    ))}
                                </ul>
                            </div>
                            <div className="lg:max-w-[505px]">
                                <p className="font-semibold font-inter-tight leading-[1.315em] text-[16px] sm:text-[20px] md:text-lg text-blue-100 mb-6">
                                    DeFindex allows wallet builders to seamlessly offer
                                    yield-generating accounts to their users.
                                </p>
                                <p className="font-inter-tight text-[16px] sm:text-[20px] md:text-lg text-blue-100">
                                    By tapping into a variety of decentralized finance (DeFi)
                                    strategies, DeFindex simplifies the process of earning passive
                                    income from cryptocurrencies.
                                </p>
                            </div>
                        </div>
                        <div>
                            <img className="lg:w-auto w-full" src="/images/Infograph.svg" alt="" />
                        </div>
                    </div>
                </div>
            </div>
        </section>
    );
}

export default HowWorks;
