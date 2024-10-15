"use client";
import React from "react";
import Slider, { Settings } from "react-slick";
import "slick-carousel/slick/slick.css";
import "slick-carousel/slick/slick-theme.css";

function OurTeam() {
    const settings: Settings = {
        dots: false,
        infinite: true,
        speed: 500,
        slidesToScroll: 3,
        centerMode: true,
        variableWidth: true,
        arrows: false,
    };

    return (
        <section id="our-team">
            <div className="container">
                <div className="pt-24 md:pt-28 xl:pt-[150px]">
                    <div className="max-w-[850px] mx-auto flex gap-2 xl:gap-4 items-center flex-col">
                        <h2 className="font-familjen-grotesk italic pr-1 text-center text-[48px] md:text-[64px] xl:text-3xl tracking-[-0.03em] text-linear bg-linear">
                            Our <b>Team</b>
                        </h2>
                        <p className="font-inter-tight text-center sm:text-[18px] md:text-[22px] xl:text-lg leading-[1.333em] text-white">
                            A diverse team of engineers, developers, and communicators from Latin
                            America. We are also the team behind Soroswap (the First DEX of Soroban,
                            and now AMM aggregator). We also created multiple developer tools such
                            as Soroban-react, create-soroban-dapp, mercury-sdk
                        </p>
                    </div>
                    <div className="py-16 md:py-24 xl:py-[120px]">
                        <div className="h-[300px] md:h-[400px] xl:h-[522px]">
                            <div>
                                <Slider {...settings}>
                                    {[...Array(12)].map((_, index) => (
                                        <div
                                            className="px-2 md:px-4 grid place-content-center"
                                            key={index}
                                        >
                                            <img
                                                className="rounded-3xl xl:rounded-[40px] w-ful item-wrapper h-[220px] md:h-[340px] xl:h-[442px] aspect-[407/522]"
                                                src="https://placehold.co/407x522"
                                                alt=""
                                            />
                                        </div>
                                    ))}
                                </Slider>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    );
}

export default OurTeam;
