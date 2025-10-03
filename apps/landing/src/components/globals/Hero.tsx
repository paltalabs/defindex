import GradientText from "@/components/common/GradientText";
import ScheduleDemoButton from "@/components/common/ScheduleDemoButton";
import Image from "next/image";
import Link from "next/link";

const heroBackgroundCss = {
    position: 'absolute' as const,
    left: 'clamp(-100px, -5vw, 0px)',
    top: '200px',
    zIndex: 1,
}
const heroBackgroundCssSm = {
    position: 'absolute' as const,
    zIndex: 1,
    justify: 'center'
}

function Hero() {
    return (
        <section
            id="hero"
            className="pt-16 pb-12 mb-[400px] lg:mb-[400px] xl:mb-[200px] md:pb-6 md:pt-20 lg:pt-24 bg-[#033036] overflow-hidden w-full"
        >
            <div className="container w-full max-w-full px-4">
                <div className="grid lg:grid-cols-3 items-center gap-8 w-full">
                    {/* Hero Image */}
                    <div style={heroBackgroundCss} className="hidden lg:block  ">
                    <Image
                        src="/images/demo_hand.webp"
                        alt="Phone with 15% APY showing hand holding device with glass gradient background"
                        className="mx-auto object-contain"
                        width={800}
                        height={700}
                        priority
                    />
                    </div>
                    <div style={heroBackgroundCssSm} className="xs:block top-[900px] md:top-[650px] justify-self-center lg:hidden max-w-fit">
                    <Image
                        src="/images/demo_hand.webp"
                        alt="Phone with 15% APY showing hand holding device with glass gradient background"
                        className="mx-auto object-contain"
                        width={500}
                        height={500}
                        priority
                    />
                    </div>
                    {/*Text and buttons */}
                    <div className="text-left z-10 col-span-2 lg:col-span-2 lg:col-start-2 px-4 sm:px-6 lg:mr-6 sm:h-100">
                        <GradientText
                            as="h1"
                            variant="primary"
                            className="
                                font-familjen-grotesk 
                                text-[32px]
                                sm:text-[48px] 
                                md:text-[60px] 
                                lg:text-[80px] 
                                leading-[1.1em] 
                                sm:leading-[1.04em] 
                                tracking-[0.05em] 
                                sm:tracking-[0.1em] 
                                mb-4 
                                sm:mb-6
                            "
                            style={{
                                fontWeight: 700,
                                fontStyle: 'Bold',
                                fontSize: '80px',
                                lineHeight: '108%',
                                letterSpacing: '10%',
                                textAlign: 'center',
                            }}
                        >
                            YIELD-AS-A-SERVICE <span style={{fontWeight: '400 !important', fontStyle: 'Regular', fontSize: '80px',lineHeight: '108%', letterSpacing: '0.2em', textAlign: 'center'}}>FOR WALLETS & APPS</span>
                        </GradientText>
                        <p className="font-inter text-[16px] text-center sm:text-[18px] md:text-[20px] mx-[17%] lg:text-[24px] text-[#D3FBFF] opacity-80 mb-6 sm:mb-8">
                            Plug-and-play SDKs that let users grow and protect stablecoin savings — while you earn TVL and revenue.
                        </p>
                        <div className="flex flex-col sm:flex-row gap-4 justify-self-center">
                            <ScheduleDemoButton />
                            <Link
                                href="https://github.com/paltalabs/defindex"
                                target="blank"
                                className="flex items-center justify-center bg-transparent border border-[#D3FFB4] text-[#D3FFB4] font-manrope font-[800] text-sm rounded-3xl px-6 py-4 sm:py-6 transition-all duration-200 hover:scale-105 hover:bg-[#D3FFB4]/10 hover:shadow-lg active:scale-95"
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
