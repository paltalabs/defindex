"use client";
import NavDrawer from "@/components/globals/navbar/NavDrawer";
import useNavbarEffect from "@/hooks/useNavbarEffect";
import useScrollOpacity from "@/hooks/useScrollOpacity";
import NextLink from "next/link";
import React from "react";
import { FiMenu } from "react-icons/fi";
import Lists from "./Lists";

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
                <div className="container w-full max-w-full px-4 py-2 md:py-6 md:px-6" style={{backgroundColor: "rgb(3, 48, 54)"}}>
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
            </nav>
            <div className="lg:hidden">
                <NavDrawer isOpen={isOpen} toggleDrawer={toggleDrawer} />
            </div>
        </div>
    );
}

export default Navbar;
