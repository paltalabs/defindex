import React from "react";
import Link from "next/link";
import { FaXTwitter, FaDiscord, FaLinkedin, FaGithub } from "react-icons/fa6";

const links: [React.ElementType, string][] = [
    [FaXTwitter, "https://x.com/PaltaLabs"],
    [FaDiscord, "https://discord.gg/CUC26qUTw7"],
    [FaLinkedin, "https://www.linkedin.com/company/paltalabs"],
    [FaGithub, "https://github.com/paltalabs/defindex"],
];

function Footer() {
    return (
        <footer className="bg-cyan-900">
            <div className="container">
                <div className="mx-auto flex max-w-[1440px] items-end">
                    <div className="hidden lg:block flex-grow">
                        <img
                            className="w-[52.5vw] max-w-[756px]"
                            src="/images/stickers.svg"
                            alt=""
                        />
                    </div>
                    <div className="lg:max-w-[316px] w-full shrink-0 py-8 sm:py-10 xl:py-16">
                        <div className="mb-10 flex gap-6">
                            <div className="flex flex-col gap-4 flex-grow">
                                <b className="font-bold font-manrope leading-[1.125em] text-xs text-white">
                                    Links
                                </b>
                                <Link
                                    className="font-manrope text-xs leading-normal text-white hover:text-lime-200 duration-200"
                                    href="/"
                                >
                                    For Wallets Builders
                                </Link>
                                <Link
                                    className="font-manrope text-xs leading-normal text-white hover:text-lime-200 duration-200"
                                    href="/"
                                >
                                    For Developers
                                </Link>
                            </div>
                            <div className="flex flex-col gap-4">
                                <b className="font-bold font-manrope leading-[1.125em] text-xs text-white">
                                    Information
                                </b>
                                <Link
                                    className="font-manrope text-xs leading-normal text-white duration-200 hover:text-lime-200"
                                    href="#FAQ"
                                >
                                    FAQ
                                </Link>
                                <Link
                                    className="font-manrope text-xs leading-normal text-white duration-200 hover:text-lime-200"
                                    href="https://paltalabs.io/#contact-us"
                                >
                                    Contact Us
                                </Link>
                                <Link
                                    className="font-manrope text-xs leading-normal text-white duration-200 hover:text-lime-200"
                                    href="https://paltalabs.io/"
                                >
                                    About Us
                                </Link>
                            </div>
                        </div>
                        <div className="flex lg:justify-end">
                            <div className="flex gap-4">
                                {links.map(([Icon, url], index) => (
                                    <Link
                                        className="group hover:scale-110 duration-75"
                                        key={index}
                                        href={url}
                                    >
                                        <Icon className="text-lime-200 group-hover:text-white duration-75 text-[20px] sm:text-[26px] md:text-[32px]" />
                                    </Link>
                                ))}
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </footer>
    );
}

export default Footer;
