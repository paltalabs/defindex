"use client";
import GlassIcons from "@/components/GlassIcons";
import NavDrawer from "@/components/globals/navbar/NavDrawer";
import useNavbarEffect from "@/hooks/useNavbarEffect";
import useScrollOpacity from "@/hooks/useScrollOpacity";
import dynamic from "next/dynamic";
import NextLink from "next/link";
import React from "react";
import { FiMenu } from "react-icons/fi";
import Lists from "./Lists";

const GlassSurface = dynamic(() => import("@/components/GlassSurface"), {
    ssr: false,
});

function Navbar() {
    const nav = useNavbarEffect("pb-7 lg:pb-10", "pb-4 lg:pb-6");
    const opacity = useScrollOpacity(200);

    const [isOpen, setIsOpen] = React.useState(false);
    const toggleDrawer = () => {
        setIsOpen((prevState) => !prevState);
    };

    return (
        <div className="w-full">
            <nav
                className={`fixed top-0 left-0 z-[1020] right-0 duration-200 w-full ${nav}`}
                style={{
                    opacity,
                    transition: 'opacity 0.3s ease-out'
                }}
            >
                <GlassSurface
                    width="100%"
                    height="auto"
                    borderRadius={24}
                    backgroundOpacity={0.5}
                    saturation={2}
                    borderWidth={0.2}
                    brightness={50}
                    opacity={0.2}
                    blur={4}
                    displace={1.9}
                    distortionScale={-50}
                    mixBlendMode="luminosity"
                    className="py-4"

                >
                    <div className="container w-full max-w-full px-4">
                        <div className="grid grid-cols-2 lg:grid-cols-[200px_1fr_200px] items-center w-full">
                        <div>
                            <NextLink href="/">
                                <img
                                    className="h-7 sm:h-9 lg:h-[36px]"
                                    src="/images/defindex.svg"
                                    alt=""
                                    style={{filter: 'drop-shadow(0 2px 4px rgba(0,0,0,0.6))'}}
                                />
                            </NextLink>
                        </div>
                        <div className="hidden lg:flex flex-grow justify-center">
                            <Lists />
                        </div>
                        <div className="flex justify-end items-center gap-6">
                            <div className="hidden sm:flex items-center">
                                <a href="mailto:dev@paltalabs.io" className="flex items-center">
                                    <GlassIcons
                                        items={[
                                            {
                                                text: "Schedule a Demo",
                                                color: "green",
                                                label: "Schedule Demo"
                                            }
                                        ]}
                                        className="!gap-0 !py-0 !m-0"
                                    />
                                </a>
                            </div>
                            <button
                                onClick={toggleDrawer}
                                role="button"
                                className="block lg:hidden text-white text-[22px] sm:text-[26px]"
                                style={{filter: 'drop-shadow(0 2px 4px rgba(0,0,0,0.6))'}}
                            >
                                <FiMenu />
                            </button>
                        </div>
                    </div>
                    </div>
                </GlassSurface>
            </nav>
            <div className="lg:hidden">
                <NavDrawer isOpen={isOpen} toggleDrawer={toggleDrawer} />
            </div>
        </div>
    );
}

export default Navbar;
