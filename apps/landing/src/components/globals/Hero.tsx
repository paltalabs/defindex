import GradientText from "@/components/common/GradientText";
import ScheduleDemoButton from "@/components/common/ScheduleDemoButton";
import { COLORS } from "@/constants/design";
import Image from "next/image";
import Link from "next/link";

const heroBackgroundBase = {
    position: 'absolute' as const,
    zIndex: 1,
}
const heroBackgroundCss = {
    ...heroBackgroundBase,
    left: 'clamp(-100px, -5vw, 0px)',
    top: '200px',
}
const heroBackgroundCssSm = {
    ...heroBackgroundBase,
    justify: 'center'
}

function Hero() {
    return (
        <section
            id="hero"
            className="pt-16 pb-12 mb-[400px] lg:mb-[400px] xl:mb-[200px] md:pb-6 md:pt-20 lg:pt-24 bg-cyan-900 overflow-hidden w-full max-w-[100vw]"
        >
            <div className="container w-full px-2 sm:px-4 max-w-full">
                <div className="grid lg:grid-cols-3 items-center gap-4 sm:gap-8 w-full max-w-full">
                    {/* Hero Image */}
                    <div style={heroBackgroundCss} className="hidden lg:block">
                    <Image
                        src="/images/demo_hand.webp"
                        alt="DeFindex Stellar wallet interface showing stablecoin yield dashboard with 15% APY"
                        className="mx-auto object-contain"
                        width={800}
                        height={700}
                        priority
                    />
                    </div>
                    <div style={heroBackgroundCssSm} className="xs:block top-[650px] sm:top-[600px] md:top-[500px] justify-self-center lg:hidden max-w-fit">
                    <Image
                        src="/images/demo_hand.webp"
                        alt="DeFindex Stellar wallet interface showing stablecoin yield dashboard with 15% APY"
                        className="mx-auto object-contain"
                        width={500}
                        height={500}
                        priority
                    />
                    </div>
                    {/*Text and buttons */}
                    <div className="text-center lg:text-left z-10 col-span-2 lg:col-span-2 lg:col-start-2 px-2 sm:px-4 lg:px-6 lg:mr-6 sm:h-100 w-full">
                        <GradientText
                            as="h1"
                            variant="primary"
                            textStroke={COLORS.dark}
                            className="
                                font-familjen-grotesk
                                text-xl
                                sm:text-2xl
                                md:text-3xl
                                lg:text-5xl
                                leading-[1.1em]
                                sm:leading-[1.04em]
                                tracking-tight
                                sm:tracking-[0.05em]
                                md:tracking-[0.1em]
                                mb-4
                                sm:mb-6
                                uppercase
                                w-[100%]
                            "
                            style={{
                                fontWeight: 700,
                                fontStyle: 'Bold',
                                textAlign: 'center',
                            }}
                        >
                            DeFindex
                        </GradientText>
                        <GradientText
                            as="h2"
                            variant="secondary"
                            textStroke={COLORS.dark}
                            className="
                                font-familjen-grotesk
                                text-lg
                                sm:text-2xl
                                md:text-3xl
                                lg:text-3xl
                                leading-[1.1em]
                                sm:leading-[1.04em]
                                tracking-normal
                                sm:tracking-[0.05em]
                                md:tracking-[0.1em]
                                mb-4
                                sm:mb-6
                                uppercase
                                max-w-full
                                break-words
                            "
                            style={{
                                fontWeight: '400 !important',
                                fontStyle: 'Regular',
                                textAlign: 'center'
                            }}
                            >
                                Yield Infrastructure for Stellar Wallets & DeFi Apps
                        </GradientText>
                        <p
                            className="
                                font-inter
                                text-center
                                text-xs
                                sm:text-sm
                                md:text-md
                                lg:text-lg
                                text-brand-light-cyan-text
                                mx-0
                                sm:mx-[10%]
                                lg:mx-[17%]
                                mb-6
                                sm:mb-8
                                [text-shadow:_-1px_0px_8px_rgba(0,0,0,0.7)]
                                max-w-full
                                "
                        >
                            Plug-and-play SDKs built on Stellar that let users grow and protect stablecoin savings — while you earn TVL and revenue.
                        </p>
                        <div className="flex flex-col sm:flex-row gap-4 justify-center items-center w-full">
                            <ScheduleDemoButton />
                            <Link
                                href="https://github.com/paltalabs/defindex"
                                target="_blank"
                                rel="noopener noreferrer"
                                aria-label="View DeFindex Stellar SDK source code on GitHub"
                                className="flex items-center justify-center bg-transparent text-lime-200 font-manrope font-extrabold text-sm rounded-3xl px-6 py-4 sm:py-6 transition-all duration-normal hover:scale-105 hover:bg-lime-200/10 hover:shadow-lg active:scale-95"
                            >
                                View on GitHub
                            </Link>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    );
}

export default Hero;
