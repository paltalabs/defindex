'use client';
import NavigateTab from "@/context/NavigateTab";
import { useRouter } from "next/navigation";
import { useContext } from "react";
import { FiExternalLink } from "react-icons/fi";
import { Link } from "react-scroll";

const menuItems = [
    { id: 1, name: "Home", to: "hero" },
    { id: 2, name: "Why Integrate", to: "why-integrate-yield" },
    { id: 3, name: "Case Studies", to: "what-builders-are-doing" },
    { id: 4, name: "Documentation", to: "https://docs.defindex.io", external: true },
];
/* eslint-disable @typescript-eslint/no-unused-vars */
function Lists({ toggle }: { toggle?: () => void }) {
    const { setIndex } = useContext(NavigateTab);
    const router = useRouter();
    return (
        <div>
            <div className="flex lg:flex-row flex-col lg:items-center ">
                {menuItems.map(({ id, name, to, external }) => (
                    external ? (
                        <a
                            href={to}
                            key={id}
                            aria-label={name}
                            className="px-3 py-2 flex items-center gap-1 transition-all duration-200 hover:scale-105 active:scale-95"
                            target="_blank"
                            rel="noopener noreferrer"
                        >
                            <span className="text-white leading-none text-[14px] lg:text-xs font-manrope hover:underline transition-colors duration-200 hover:text-[#D3FFB4]" style={{lineHeight: '16px', textShadow: '0 2px 4px rgba(0,0,0,0.6), 0 1px 2px rgba(0,0,0,0.4)'}}>
                                {name}
                            </span>
                            <FiExternalLink className="text-white text-[12px] lg:text-[10px] transition-colors duration-200 group-hover:text-[#D3FFB4]" style={{filter: 'drop-shadow(0 2px 4px rgba(0,0,0,0.6))'}} />
                        </a>
                    ) : (
                        <Link
                            href={`#${to}`}
                            to={`${to}`}
                            key={id}
                            aria-label={to}
                            offset={-150}
                            className="px-3 py-2 cursor-pointer transition-all ease-in hover:scale-105 active:scale-95"
                            activeClass="nav-activated"
                            spy={true}
                            smooth={true}
                        >
                            <span className="text-white leading-none text-[14px] lg:text-xs font-manrope hover:underline transition-colors duration-200 hover:text-[#D3FFB4]"  style={{lineHeight: '16px', textShadow: '0 2px 4px rgba(0,0,0,0.6), 0 1px 2px rgba(0,0,0,0.4)'}}>
                                {name}
                            </span>
                        </Link>
                    )
                ))}
            </div>
        </div>
    );
}
/* eslint-disable @typescript-eslint/no-unused-vars */

export default Lists;
