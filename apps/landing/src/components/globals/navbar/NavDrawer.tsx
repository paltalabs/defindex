"use client";
import dynamic from "next/dynamic";
import React from "react";
import { IoCloseOutline } from "react-icons/io5";
import "react-modern-drawer/dist/index.css";

interface DrawerProps {
  open: boolean;
  onClose?: () => void;
  direction: "left" | "right" | "top" | "bottom";
  children: React.ReactNode;
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
const Drawer = dynamic<DrawerProps>(() => import("react-modern-drawer") as any, {
  ssr: false,
});

import Image from "next/image";
import { default as Link, default as NextLink } from "next/link";
import { FaDiscord, FaGithub, FaLinkedin, FaXTwitter } from "react-icons/fa6";
import { Link as ScrollLink } from "react-scroll";
import SimpleBar from "simplebar-react";
import "simplebar-react/dist/simplebar.min.css";
import Lists from "./Lists";

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
                                <NextLink href="/" aria-label="Go to homepage">
                                    <Image className="h-7" src="/images/defindex.svg" alt="DeFindex logo" width={120} height={28} />
                                </NextLink>
                                <button
                                    onClick={toggleDrawer}
                                    role="button"
                                    className="text-[22px]"
                                    aria-label="Close navigation drawer"
                                >
                                    <IoCloseOutline />
                                </button>
                            </div>
                            <div className="mb-6">
                                <Lists toggle={toggleDrawer} />
                            </div>
                            <div className="sm:hidden">
                                <ScrollLink
                                    className="rounded-3xl outlined-button border border-lime-200 px-6 py-3 w-full flex justify-center text-center"
                                    to="cta-form"
                                    offset={-150}
                                >
                                    <span className="font-extrabold font-manrope text-[14px] lg:text-xs leading-tight text-lime-200">
                                        Contact Us
                                    </span>
                                </ScrollLink>
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
