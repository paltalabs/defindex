"use client";
import React, { useState } from "react";
import SmoothCollapse from "react-smooth-collapse";

interface Props {
    title: string;
    description: string;
    isOpen?: boolean;
}

function Unit(props: Props) {
    const { title, description, isOpen = false } = props;

    const [open, setOpen] = useState(isOpen);
    const handleToggle = () => setOpen((v) => !v);

    return (
        <div className="rounded-2xl px-5 md:px-8 pt-6 md:pt-10 pb-8 md:pb-12 bg-white shadow-[0px_5px_16px_0px_rgba(8,_15,_52,_0.06)]">
            <div onClick={handleToggle} className="flex items-center gap-4 mb-1" role="button">
                <b className="flex-grow font-bold font-familjen-grotesk italic text-[20px] md:text-base text-cyan-950">
                    {title}
                </b>
                <img
                    className="md:w-auto w-7"
                    src={open ? "/images/Group 482262.svg" : "/images/Group 36813.svg"}
                    alt=""
                />
            </div>
            <SmoothCollapse expanded={open}>
                <div className="w-[calc(100%-80px)]">
                    <p className="font-inter-tight text-[16px] md:text-sm text-cyan-950">
                        {description}
                    </p>
                </div>
            </SmoothCollapse>
        </div>
    );
}

export default Unit;
