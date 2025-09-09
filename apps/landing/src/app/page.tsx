"use client";
import CTAForm from "@/components/common/CTAFrom";
import Footer from "@/components/globals/Footer";
import Frequently from "@/components/globals/Frequently";
import Hero from "@/components/globals/Hero";
import HowWorks from "@/components/globals/HowWorks";
import InvestmentGrowth from "@/components/globals/InvestmentGrowth";
import Navbar from "@/components/globals/navbar/Navbar";
import OurTeam from "@/components/globals/OurTeam";
import Security from "@/components/globals/Security";
import WalletBuilders from "@/components/globals/WalletBuilders";
import NavigateTab from "@/context/NavigateTab";
import Image from "next/image";
import { useState } from "react";

function Home() {
    const [index, setIndex] = useState(0);

    return (
        <div className="min-h-screen bg-black relative z-0 ">
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
                <InvestmentGrowth />
                <WalletBuilders />
                <Security />
                <OurTeam />
                <Frequently />
                {/* <WalletExperience /> */}
                <CTAForm className="mt-20" />
            </NavigateTab.Provider>
            <Footer />
        </div>
    );
}

export default Home;
