"use client";

function OurTeam() {
    return (
        <section id="our-team" className="mb-16">
            <div className="container">
                <div className="pt-24 md:pt-28 xl:pt-[150px]">
                    <div className="max-w-[850px] mx-auto flex gap-2 xl:gap-4 items-center flex-col">
                        <h2 className="font-familjen-grotesk italic pr-1 text-center text-[48px] md:text-[64px] xl:text-3xl tracking-[-0.03em] text-linear bg-linear">
                            Our <b>Team</b>
                        </h2>
                        <p className="font-inter-tight text-center sm:text-[18px] md:text-[22px] xl:text-lg leading-[1.333em] text-white">
                            PaltaLabs🥑 is a diverse team of engineers, developers, and communicators from Latin
                            America. We are also the team behind Soroswap (the First DEX of Soroban,
                            and now AMM aggregator). We also created multiple developer tools such
                            as Soroban-react, create-soroban-dapp, mercury-sdk
                        </p>
                    </div>
                </div>
            </div>
        </section>
    );
}

export default OurTeam;
