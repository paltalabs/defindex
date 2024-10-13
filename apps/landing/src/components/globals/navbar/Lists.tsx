import React, { useContext } from "react";
import { Link, scroller } from "react-scroll";
import NavigateTab from "@/context/NavigateTab";

const menuItems = [
    { id: 1, name: "Home", to: "hero" },
    { id: 2, name: "How It Works", to: "how-it-works" },
    { id: 3, name: "For Wallets Builders", to: "wallets-builders" },
    { id: 4, name: "For Developers", to: "" },
    { id: 5, name: "Our Team", to: "our-team" },
];

function Lists({ toggle }: { toggle?: () => void }) {
    const { index, setIndex } = useContext(NavigateTab);

    return (
        <div>
            <div className="flex lg:flex-row flex-col lg:items-center">
                {menuItems.map(({ id, name, to }) => (
                    <Link
                        href={`#${to}`}
                        to={to}
                        key={id}
                        spy
                        offset={-120}
                        className="px-3 py-2"
                        activeClass="nav-activated"
                        onClick={() => {
                            toggle?.();
                            setIndex(name === "For Developers" ? 1 : 0);
                            scroller.scrollTo("wallets-builders", { offset: -120 });
                        }}
                    >
                        <span className="text-white leading-none text-[14px] lg:text-xs font-manrope hover:underline">
                            {name}
                        </span>
                    </Link>
                ))}
            </div>
        </div>
    );
}

export default Lists;
