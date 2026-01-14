'use client';
import NavigateTab from "@/context/NavigateTab";
import { usePathname, useRouter } from "next/navigation";
import { useContext } from "react";
import { FiExternalLink } from "react-icons/fi";
import { Link } from "react-scroll";

const menuItems = [
    { id: 1, name: "Home", to: "hero" },
    { id: 2, name: "Why Integrate", to: "why-integrate-yield" },
    { id: 3, name: "Case Studies", to: "what-builders-are-doing" },
    { id: 4, name: "Simulate Revenue", to: "/revenue-calculator", external: true, isInternal: true },
    { id: 5, name: "Blog", to: "/blog", external: true, isInternal: true },
    { id: 6, name: "Documentation", to: "https://docs.defindex.io", external: true },
    { id: 7, name: "Dashboards", to: "https://dune.com/paltalabs/defindex", external: true },
];
/* eslint-disable @typescript-eslint/no-unused-vars */
function Lists({ toggle }: { toggle?: () => void }) {
    const { setIndex } = useContext(NavigateTab);
    const router = useRouter();
    const pathname = usePathname();
    const isHomePage = pathname === '/';

    return (
        <div>
            <div className="flex lg:flex-row flex-col lg:items-center ">
                {menuItems.map(({ id, name, to, external, isInternal }) => {
                    // If it's an external link, render as anchor
                    if (external) {
                        return (
                            <a
                                href={to}
                                key={id}
                                aria-label={name}
                                className="px-3 py-2 flex items-center gap-1 transition-all duration-normal hover:scale-105 active:scale-95"
                                target={isInternal ? "_self" : "_blank"}
                                rel={isInternal ? undefined : "noopener noreferrer"}
                            >
                                <span className="text-white leading-none text-sm lg:text-xs font-manrope hover:underline transition-colors duration-normal hover:text-lime-200" style={{lineHeight: '16px', textShadow: '0 2px 4px rgba(0,0,0,0.6), 0 1px 2px rgba(0,0,0,0.4)'}}>
                                    {name}
                                </span>
                                {!isInternal && <FiExternalLink className="text-white text-xs lg:text-xs transition-colors duration-normal group-hover:text-lime-200" style={{filter: 'drop-shadow(0 2px 4px rgba(0,0,0,0.6))'}} />}
                            </a>
                        );
                    }

                    // If we're on homepage, use react-scroll
                    if (isHomePage) {
                        return (
                            <Link
                                to={to}
                                key={id}
                                aria-label={to}
                                offset={-150}
                                className="px-3 py-2 cursor-pointer transition-all ease-in hover:scale-105 active:scale-95"
                                activeClass="nav-activated"
                                spy={true}
                                smooth={true}
                            >
                                <span className="text-white leading-none text-sm lg:text-xs font-manrope hover:underline transition-colors duration-normal hover:text-lime-200"  style={{lineHeight: '16px', textShadow: '0 2px 4px rgba(0,0,0,0.6), 0 1px 2px rgba(0,0,0,0.4)'}}>
                                    {name}
                                </span>
                            </Link>
                        );
                    }

                    // If we're NOT on homepage, use regular anchor to navigate back
                    return (
                        <a
                            href={`/#${to}`}
                            key={id}
                            aria-label={name}
                            className="px-3 py-2 cursor-pointer transition-all ease-in hover:scale-105 active:scale-95"
                        >
                            <span className="text-white leading-none text-sm lg:text-xs font-manrope hover:underline transition-colors duration-normal hover:text-lime-200"  style={{lineHeight: '16px', textShadow: '0 2px 4px rgba(0,0,0,0.6), 0 1px 2px rgba(0,0,0,0.4)'}}>
                                {name}
                            </span>
                        </a>
                    );
                })}
            </div>
        </div>
    );
}
/* eslint-disable @typescript-eslint/no-unused-vars */

export default Lists;
