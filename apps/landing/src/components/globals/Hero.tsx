import React from "react";
import Link from "next/link";
import Image from "next/image";

function Hero() {
    return (
        <section
            id="hero"
            className="pt-24 pb-12 sm:pb-24 md:pb-32 lg:pb-[200px] md:pt-28 lg:pt-[180px]"
        >
            <div className="container">
                <div className="max-w-[1216px] py-10 max-h-[600px] md:max-h-[750px] z-0 relative h-screen mx-auto">
                    <h1 className="font-bold max-w-[300px] md:max-w-[370px] font-familjen-grotesk italic text-[80px] md:text-[96px] lg:text-[120px] leading-[0.86em] xl:text-4xl bg-[linear-gradient(121deg,_#FFF_7.14%,_#DEC9F4_82.55%)] text-linear">
                        DeFi Made Easy
                    </h1>
                    <Image
                        quality={100}
                        width={886}
                        height={748}
                        className="absolute -z-10 left-1/2 top-1/2 -translate-y-1/2 -translate-x-1/2"
                        src="/images/hero-background.png"
                        alt=""
                    />
                    <div className="max-w-[266px] flex flex-col items-start absolute bottom-0 py-[70px] right-0">
                        <p className="font-inter font-italic text-[16px] sm:text-[20px] xl:text-lg leading-[1.12em] mb-4 sm:mb-6 text-blue-100/80">
                            Offer yield-generating accounts to your users with seamless DeFi
                            strategies.
                        </p>
                        <Link
                            className="font-extrabold contained-button font-manrope text-[14px] md:leading-none md:text-xs text-cyan-950 leading-none rounded-3xl bg-lime-200 px-6 py-3 lg:py-5 xl:min-h-[60px] flex items-center justify-center"
                            href=""
                        >
                            Explore DeFindex
                        </Link>
                    </div>
                </div>
            </div>
        </section>
    );
}

export default Hero;
