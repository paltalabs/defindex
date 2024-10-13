"use client";
import React from "react";
import useNavbarEffect from "@/hooks/useNavbarEffect";
import NextLink from "next/link";
import Lists from "./Lists";
import { FiMenu } from "react-icons/fi";
import NavDrawer from "@/components/globals/navbar/NavDrawer";

function Navbar() {
    const nav = useNavbarEffect("py-7 lg:py-10", "py-4 lg:py-6 bg-[#033036]");

    const [isOpen, setIsOpen] = React.useState(false);
    const toggleDrawer = () => {
        setIsOpen((prevState) => !prevState);
    };

    return (
        <div>
            <nav className={`fixed top-0 left-0 z-[1020] right-0 duration-200 ${nav}`}>
                <div className="container">
                    <div className="grid grid-cols-2 lg:grid-cols-[200px_1fr_200px] items-center max-w-[1210px] mx-auto">
                        <div>
                            <NextLink href="/">
                                <img
                                    className="h-7 sm:h-9 lg:h-[42px]"
                                    src="/images/defindex.svg"
                                    alt=""
                                />
                            </NextLink>
                        </div>
                        <div className="hidden lg:flex flex-grow justify-center">
                            <Lists />
                        </div>
                        <div className="flex justify-end items-center gap-6">
                            <NextLink
                                className="hidden outlined-button rounded-3xl border border-lime-200 px-6 py-3 lg:min-h-[60px] sm:flex gap-2.5 items-center justify-center"
                                href="/"
                            >
                                <span className="font-extrabold font-manrope text-[14px] lg:text-xs lg:leading-tight text-lime-200">
                                    Schedule a Demo
                                </span>
                            </NextLink>
                            <button
                                onClick={toggleDrawer}
                                role="button"
                                className="block lg:hidden text-white text-[22px] sm:text-[26px]"
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
