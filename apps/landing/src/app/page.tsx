"use client";
import Footer from "@/components/globals/Footer";
import Frequently from "@/components/globals/Frequently";
import Hero from "@/components/globals/Hero";
import Navbar from "@/components/globals/navbar/Navbar";
import OurTeam from "@/components/globals/OurTeam";
import Security from "@/components/globals/Security";
import Solutions from "@/components/globals/Solutions";
import Testimonials from "@/components/globals/Testimonials";
import CodeExample from "@/components/globals/CodeExample";
import NavigateTab from "@/context/NavigateTab";
import Image from "next/image";
import { useState } from "react";

function Home() {
    const [index, setIndex] = useState(0);

    return (
        <div className="min-h-screen bg-[#033036] relative overflow-x-hidden w-full" style={{maxWidth:'100dvw'}}>
            {/* Background image */}
            <div className="absolute inset-0 -z-10">
                <Image
                    src="/images/hero-background.png"
                    alt=""
                    fill
                    className="object-cover opacity-20"
                />
            </div>

            <NavigateTab.Provider value={{ index, setIndex }}>
                <Navbar />
                <Hero />
                <Solutions />
                <Testimonials />
                <CodeExample />
                <Security />
                <OurTeam />
                <Frequently />
            </NavigateTab.Provider>

            <Footer />
        </div>
    );
}

export default Home;