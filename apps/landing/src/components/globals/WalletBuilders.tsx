"use client";
import React, { useContext } from "react";
import { Tab, Tabs, TabList, TabPanel } from "react-tabs";
import Solutions from "@/components/globals/Solutions";
import NavigateTab from "@/context/NavigateTab";

function WalletBuilders() {
    const { index, setIndex } = useContext(NavigateTab);

    return (
        <section className="-mb-7 relative z-10 bg-white">
            <div className="rounded-3xl bg-white pt-16 pb-12 md:pb-20">
                <div className="container">
                    <Tabs
                        selectedIndex={index}
                        onSelect={setIndex}
                        className="mx-auto max-w-[1217px] wallets-tabs"
                    >
                        <TabList className="grid grid-cols-2 max-w-[808px] mx-auto items-center mb-8 sm:mb-16 xl:mb-[90px]">
                            <Tab
                                role="button"
                                className="py-4 xl:py-6 cursor-pointer focus:outline-0 rounded-l-3xl px-6 sm:px-12 xl:px-[104px] flex gap-2.5 items-center justify-center border-2 border-cyan-950 font-familjen-grotesk text-[16px] whitespace-nowrap sm:text-[20px] md:text-lg leading-none md:leading-none tracking-[-0.03em] hover:bg-cyan-950/10 duration-75 text-cyan-950"
                            >
                                For Wallets Builders
                            </Tab>
                            <Tab
                                role="button"
                                className="py-4 xl:py-6 cursor-pointer focus:outline-0 rounded-r-3xl px-6 sm:px-12 xl:px-[104px] flex gap-2.5 items-center justify-center border-2 border-cyan-950 font-familjen-grotesk text-[16px] whitespace-nowrap sm:text-[20px] md:text-lg leading-none md:leading-none tracking-[-0.03em] hover:bg-cyan-950/10 duration-75 text-cyan-950"
                            >
                                For Developers
                            </Tab>
                        </TabList>

                        <div id="wallets-builders">
                            <TabPanel>
                                <Solutions
                                    color="red"
                                    label="Solutions for Wallet builders"
                                    title="Tailored Solutions **for Wallet builders**"
                                    learn_more="/"
                                    thumb="/images/CTA.png"
                                    strategies={[
                                        {
                                            id: 1,
                                            icon: "/images/bytesize_portfolio.svg",
                                            description:
                                                "Choose from a variety of DeFi strategies to offer customized portfolios",
                                        },
                                        {
                                            id: 2,
                                            icon: "/images/iconoir_laptop-dev-mode.svg",
                                            description:
                                                "Developer-friendly tools to seamlessly integrate into your app",
                                        },
                                        {
                                            id: 3,
                                            icon: "/images/ci_users.svg",
                                            description:
                                                "Enhance user engagement with yield-generating accounts",
                                        },
                                    ]}
                                    advices={[
                                        {
                                            title: "Choose Your Strategies:",
                                            describe: `Select from a variety of yield-generating strategies to
                                            match your users'' needs. Offer a curated portfolio
                                            that maximizes returns while managing risk.`,
                                        },
                                        {
                                            title: "Effortless Integration:",
                                            describe: `Whether you're building or managing a wallet,
                                            DeFindex provides the developer tools you need. Easily
                                            integrate customizable buttons directly into your app,
                                            enabling your users to start earning yield with just one
                                            click.`,
                                        },
                                    ]}
                                />
                            </TabPanel>
                        </div>
                        <div>
                            <TabPanel>
                                <Solutions
                                    color="green"
                                    label="Benefits for Developers"
                                    title="For **Developers**"
                                    learn_more="/"
                                    thumb="/images/CTA (1).png"
                                    strategies={[
                                        {
                                            id: 1,
                                            icon: "/images/hugeicons_developer.svg",
                                            description:
                                                "Develop your own strategies on DeFindex.",
                                        },
                                        {
                                            id: 2,
                                            icon: "/images/eos-icons_blockchain.svg",
                                            description:
                                                "Incentivize usage of your protocol through yield opportunities.",
                                        },
                                        {
                                            id: 3,
                                            icon: "/images/iconoir_community.svg",
                                            description:
                                                "Engage with a growing community of DeFi users.",
                                        },
                                    ]}
                                    advices={[
                                        {
                                            title: "Build & Innovate:",
                                            describe: `Protocol developers can create and integrate their own strategies within DeFindex, incentivizing users to engage with their protocol through tailored yield options.`,
                                        },
                                    ]}
                                />
                            </TabPanel>
                        </div>
                    </Tabs>
                </div>
            </div>
        </section>
    );
}

export default WalletBuilders;
