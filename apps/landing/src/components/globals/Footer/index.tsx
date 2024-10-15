import React from "react";
import Link from "next/link";
import { FaXTwitter, FaDiscord, FaLinkedin, FaGithub } from "react-icons/fa6";
import { Dock, DockIcon } from "@/components/ui/dock";

const links: [React.ElementType, string][] = [
    [FaXTwitter, "/"],
    [FaDiscord, "/"],
    [FaLinkedin, "/"],
    [FaGithub, "/"],
];

function Footer() {
    return (
        <footer className="bg-cyan-900">
            <div className="container">
                <div className="mx-auto flex max-w-[1440px] items-end">
                    <div className="hidden lg:block flex-grow">
                        <img
                            data-aos="fade-zoom-in"
                            className="w-[52.5vw] max-w-[756px]"
                            src="/images/stickers.svg"
                            alt=""
                        />
                    </div>
                    <div className="lg:max-w-[316px] w-full shrink-0 py-8 sm:py-10 xl:py-16">
                        <div className="mb-10 flex gap-6">
                            <div className="flex flex-col gap-4 flex-grow">
                                <b
                                    data-aos="fade-up"
                                    data-aos-offset="0"
                                    className="font-bold font-manrope leading-[1.125em] text-xs text-white"
                                >
                                    Links
                                </b>
                                <Link
                                    data-aos="fade-up"
                                    data-aos-offset="0"
                                    data-aos-delay="100"
                                    className="font-manrope text-xs leading-normal text-white hover:text-lime-200 duration-200"
                                    href="/"
                                >
                                    For Wallets Builders
                                </Link>
                                <Link
                                    data-aos="fade-up"
                                    data-aos-offset="0"
                                    data-aos-delay="100"
                                    className="font-manrope text-xs leading-normal text-white hover:text-lime-200 duration-200"
                                    href="/"
                                >
                                    For Developers
                                </Link>
                            </div>
                            <div className="flex flex-col gap-4">
                                <b
                                    data-aos="fade-up"
                                    data-aos-offset="0"
                                    className="font-bold font-manrope leading-[1.125em] text-xs text-white"
                                >
                                    Information
                                </b>
                                <Link
                                    data-aos="fade-up"
                                    data-aos-offset="0"
                                    data-aos-delay="100"
                                    className="font-manrope text-xs leading-normal text-white duration-200 hover:text-lime-200"
                                    href="/"
                                >
                                    FAQ
                                </Link>
                                <Link
                                    data-aos="fade-up"
                                    data-aos-offset="0"
                                    data-aos-delay="100"
                                    className="font-manrope text-xs leading-normal text-white duration-200 hover:text-lime-200"
                                    href="/"
                                >
                                    Contact Us
                                </Link>
                                <Link
                                    data-aos="fade-up"
                                    data-aos-offset="0"
                                    data-aos-delay="100"
                                    className="font-manrope text-xs leading-normal text-white duration-200 hover:text-lime-200"
                                    href="/"
                                >
                                    About Us
                                </Link>
                            </div>
                        </div>
                        <div className="flex lg:justify-end">
                            <Dock className="border-0 flex gap-4" direction="middle">
                                {links.map(([Icon, url], index) => (
                                    <DockIcon key={index}>
                                        <Link className="group" href={url}>
                                            <Icon className="text-lime-200 group-hover:text-white text-[20px] sm:text-[26px] md:text-[32px]" />
                                        </Link>
                                    </DockIcon>
                                ))}
                            </Dock>
                        </div>
                    </div>
                </div>
            </div>
        </footer>
    );
}

export default Footer;
