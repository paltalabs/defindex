"use client";
import { useEffect, useState } from "react";

function useScrollOpacity(maxScroll: number = 100) {
    const [opacity, setOpacity] = useState(0);

    useEffect(() => {
        const handleScroll = () => {
            const scrollY = window.scrollY;
            const newOpacity = Math.min(scrollY / maxScroll, 1);
            setOpacity(newOpacity);
        };

        handleScroll(); // Set initial opacity
        window.addEventListener("scroll", handleScroll);

        return () => window.removeEventListener("scroll", handleScroll);
    }, [maxScroll]);

    return opacity;
}

export default useScrollOpacity;
