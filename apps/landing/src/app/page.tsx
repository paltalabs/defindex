"use client";
import React, { useEffect, useState } from "react";
import Navbar from "@/components/globals/navbar/Navbar";
import HowWorks from "@/components/globals/HowWorks";
import WalletBuilders from "@/components/globals/WalletBuilders";
import Security from "@/components/globals/Security";
import OurTeam from "@/components/globals/OurTeam";
import Frequently from "@/components/globals/Frequently";
import WalletExperience from "@/components/globals/WalletExperience";
import Footer from "@/components/globals/Footer";
import Hero from "@/components/globals/Hero";
import Image from "next/image";
import NavigateTab from "@/context/NavigateTab";

import AOS from "aos";
import "aos/dist/aos.css";

function Home() {
    const [index, setIndex] = useState(0);

    useEffect(() => {
        AOS.init({
            easing: "ease-out-quad",
            duration: 500,
        });
    }, []);

    return (
        <div className="min-h-screen bg-black overflow-hidden relative z-0 ">
            <Image
                width={1440}
                height={6797}
                className="w-full h-full inset-0 absolute -z-10"
                src="/images/web-background.png"
                alt=""
            />
            <NavigateTab.Provider value={{ index, setIndex }}>
                <Navbar />
                <Hero />
                <HowWorks />
                <WalletBuilders />
            </NavigateTab.Provider>
            <Security />
            <OurTeam />
            <Frequently />
            <WalletExperience />
            <Footer />
        </div>
    );
}

export default Home;
