'use client';
import NavigateTab from "@/context/NavigateTab";
import { useRouter } from "next/navigation";
import { useContext } from "react";
import { Link } from "react-scroll";

const menuItems = [
    { id: 2, name: "How It Works", to: "how-it-works" },
    { id: 3, name: "For Wallets Builders", to: "wallets-builders" },
    { id: 4, name: "For Developers", to: "for-developers" },
    { id: 5, name: "Our Team", to: "our-team" },
    { id: 6, name: "Frequently Asked Questions", to: "FAQ" },
];

function Lists({ toggle }: { toggle?: () => void }) {
    const { setIndex } = useContext(NavigateTab);
    const router = useRouter();
    return (
        <div>
            <div className="flex lg:flex-row flex-col lg:items-center ">
                {menuItems.map(({ id, name, to }) => (
                    <Link
                        href={`#${to}`}
                        to={`${to}`}
                        key={id}
                        aria-label={to}
                        offset={-150}
                        className="px-3 py-2"
                        activeClass="nav-activated"
                        onClickCapture={(e)=>{
                            const label = e.currentTarget.ariaLabel;
                            switch (label) {
                                case 'for-developers':
                                    setIndex(1);
                                    router.push('/#for-developers');
                                    break;
                                case 'wallets-builders':
                                    setIndex(0);
                                    router.push('/#wallets-builders');
                                    break;
                                default:
                                    router.push(`/#${label}`);
                            }
                        }}
                    >
                        <span className="text-white leading-none text-[14px] lg:text-xs font-manrope hover:underline"  style={{lineHeight: '16px'}}>
                            {name}
                        </span>
                    </Link>
                ))}
            </div>
        </div>
    );
}

export default Lists;
