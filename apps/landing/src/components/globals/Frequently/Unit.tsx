"use client";
import { useState } from "react";
import SmoothCollapse from "react-smooth-collapse";

interface Props {
    title: string;
    description: React.ReactNode;
    isOpen?: boolean;
}

function Unit(props: Props) {
    const { title, description, isOpen = false } = props;

    const [open, setOpen] = useState(isOpen);
    const handleToggle = () => setOpen((v) => !v);

    return (
        <div
            className={`rounded-2xl px-6 py-6 transition-all duration-300 ${
                open
                    ? "bg-white"
                    : "bg-[#014751] text-white"
            }`}
        >
            <div
                onClick={handleToggle}
                className="flex items-center justify-between cursor-pointer"
                role="button"
            >
                <h3 className={`font-familjen-grotesk text-lg font-semibold ${
                    open ? "text-cyan-950" : "text-white"
                }`}>
                    {title}
                </h3>
                <button
                    className={`w-8 h-8 rounded-full flex items-center justify-center transition-all duration-300 ${
                        open
                            ? "bg-[#DEC9F4]"
                            : "bg-[#D3FFB4]"
                    }`}
                >
                    <img
                        src={open ? "/images/icon-minus.svg" : "/images/icon-plus.svg"}
                        alt={open ? "Collapse" : "Expand"}
                        className="w-4 h-4"
                        style={{ color: '#033036' }}
                    />
                </button>
            </div>
            <SmoothCollapse expanded={open}>
                <div className="mt-4 pr-10">
                    <p 
                        className={`font-inter text-[18px] text-[#014751] ${
                            open ? "text-cyan-950" : "text-white"
                        }`}
                        style={{fontWeight: 400,}}
                    >
                        {description}
                    </p>
                </div>
            </SmoothCollapse>
        </div>
    );
}

export default Unit;