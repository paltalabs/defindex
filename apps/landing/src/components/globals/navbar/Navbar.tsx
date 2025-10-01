"use client";
import ScheduleDemoButton from "@/components/common/ScheduleDemoButton";
import NavDrawer from "@/components/globals/navbar/NavDrawer";
import useNavbarEffect from "@/hooks/useNavbarEffect";
import NextLink from "next/link";
import React from "react";
import { FiMenu } from "react-icons/fi";
import Lists from "./Lists";

function Navbar() {
    const nav = useNavbarEffect("py-7 lg:py-10", "py-4 lg:py-6 bg-[#033036]");

    const [isOpen, setIsOpen] = React.useState(false);
    const toggleDrawer = () => {
        setIsOpen((prevState) => !prevState);
    };

    return (
        <div className="w-full">
            <nav className={`fixed top-0 left-0 z-[1020] right-0 duration-200 w-full ${nav}`}>
                <div className="container w-full max-w-full">
                    <div className="grid grid-cols-2 lg:grid-cols-[200px_1fr_200px] items-center w-full px-4">
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
                            <div className="hidden sm:block">
                                <ScheduleDemoButton />
                            </div>
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
