import React from "react";
import { IoCloseOutline } from "react-icons/io5";
import Drawer from "react-modern-drawer";
import "react-modern-drawer/dist/index.css";

import SimpleBar from "simplebar-react";
import "simplebar-react/dist/simplebar.min.css";
import Image from "next/image";
import NextLink from "next/link";
import Lists from "./Lists";
import Link from "next/link";
import { FaDiscord, FaGithub, FaLinkedin, FaXTwitter } from "react-icons/fa6";

interface Props {
    isOpen: boolean;
    toggleDrawer?: () => void;
}

const links: [React.ElementType, string][] = [
    [FaXTwitter, "/"],
    [FaDiscord, "/"],
    [FaLinkedin, "/"],
    [FaGithub, "/"],
];

function NavDrawer({ toggleDrawer, isOpen }: Props) {
    return (
        <div className="relative z-[1080]">
            <Drawer open={isOpen} onClose={toggleDrawer} direction="left">
                <div className="h-screen relative z-0 flex flex-col text-white justify-between">
                    <Image
                        width={1440}
                        height={6797}
                        className="w-full h-full inset-0 absolute -z-10"
                        src="/images/web-background.png"
                        alt=""
                    />
                    <SimpleBar style={{ height: "calc(100vh - 150px)" }}>
                        <div className="p-4">
                            <div className="flex items-center justify-between gap-2 mb-12">
                                <NextLink href="/">
                                    <img className="h-7" src="/images/defindex.svg" alt="" />
                                </NextLink>
                                <button
                                    onClick={toggleDrawer}
                                    role="butotn"
                                    className="text-[22px]"
                                    aria-label="drawer close"
                                >
                                    <IoCloseOutline />
                                </button>
                            </div>
                            <div className="mb-6">
                                <Lists toggle={toggleDrawer} />
                            </div>
                            <div className="sm:hidden">
                                <NextLink
                                    className="rounded-3xl outlined-button border border-lime-200 px-6 py-3 w-full flex justify-center text-center"
                                    href="/"
                                >
                                    <span className="font-extrabold font-manrope text-[14px] lg:text-xs leading-tight text-lime-200">
                                        Schedule a Demo
                                    </span>
                                </NextLink>
                            </div>
                        </div>
                    </SimpleBar>
                    <div className="p-4">
                        <div className="flex gap-2 mb-1">
                            {links.map(([Icon, url], index) => (
                                <Link className="group" key={index} href={url}>
                                    <Icon className="text-lime-200 group-hover:text-white duration-100 text-[16px]" />
                                </Link>
                            ))}
                        </div>
                        <p className="text-[12px] text-white/75">
                            &copy; {new Date().getFullYear()} Alright reserved.
                        </p>
                    </div>
                </div>
            </Drawer>
        </div>
    );
}

export default NavDrawer;
