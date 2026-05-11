"use client";

import ScheduleDemoButton from "@/components/common/ScheduleDemoButton";
import Link from "next/link";
import Image from "next/image";
import { useState, useEffect } from "react";

const ROTATING_WORDS = ["wallet", "bank", "app", "neobank"];
const TYPING_SPEED_MS = 80;
const DELETING_SPEED_MS = 50;
const PAUSE_AFTER_TYPED_MS = 1800;
const CURSOR_BLINK_INTERVAL_MS = 530;

function Hero() {
    const [wordIndex, setWordIndex] = useState(0);
    const [displayedWord, setDisplayedWord] = useState("wallet");
    const [isDeleting, setIsDeleting] = useState(false);
    const [cursorVisible, setCursorVisible] = useState(true);

    useEffect(() => {
        const id = setInterval(() => setCursorVisible((v) => !v), CURSOR_BLINK_INTERVAL_MS);
        return () => clearInterval(id);
    }, []);

    useEffect(() => {
        const currentWord = ROTATING_WORDS[wordIndex];
        if (!isDeleting && displayedWord === currentWord) {
            const id = setTimeout(() => setIsDeleting(true), PAUSE_AFTER_TYPED_MS);
            return () => clearTimeout(id);
        }
        if (isDeleting && displayedWord === "") {
            setIsDeleting(false);
            setWordIndex((i) => (i + 1) % ROTATING_WORDS.length);
            return;
        }
        const delay = isDeleting ? DELETING_SPEED_MS : TYPING_SPEED_MS;
        const id = setTimeout(() => {
            setDisplayedWord(
                isDeleting
                    ? currentWord.slice(0, displayedWord.length - 1)
                    : currentWord.slice(0, displayedWord.length + 1)
            );
        }, delay);
        return () => clearTimeout(id);
    }, [displayedWord, isDeleting, wordIndex]);

    return (
        <section
            id="hero"
            className="relative overflow-hidden pt-14 pb-0 md:pt-20 lg:pt-24"
        >
            <div className="max-w-[1180px] mx-auto px-5">
                {/* Two-column grid: text left, art right */}
                <div className="grid grid-cols-1 md:grid-cols-[1.1fr_0.9fr] gap-8 md:gap-12 items-center">
                    {/* Left: text */}
                    <div className="z-10 py-10 md:py-16">
                        {/* h1 with animated coral italic word */}
                        <h1
                            className="font-familjen-grotesk font-bold text-white mb-5"
                            style={{
                                fontSize: "clamp(40px, 8.5vw, 84px)",
                                lineHeight: "1.02",
                                letterSpacing: "-0.02em",
                                textWrap: "balance",
                            }}
                        >
                            Yield infrastructure for every{" "}
                            <br />
                            <em
                                style={{
                                    fontStyle: "italic",
                                    color: "#FC5B31",
                                    fontWeight: "inherit",
                                }}
                            >
                                {displayedWord}
                                <span style={{ opacity: cursorVisible ? 1 : 0 }}>|</span>
                            </em>
                        </h1>

                        <p
                            className="font-inter-tight text-white/70 max-w-[60ch] mb-7"
                            style={{ fontSize: "clamp(17px, 1.6vw, 20px)", lineHeight: "1.55" }}
                        >
                            Plug-and-play SDKs built on Stellar that let users grow and protect
                            stablecoin savings — while you earn TVL and revenue.
                        </p>

                        <div className="flex flex-wrap gap-3">
                            <ScheduleDemoButton />
                            <Link
                                href="/strategies"
                                aria-label="Explore DeFindex Strategies"
                                className="inline-flex items-center gap-2 rounded-full font-inter-tight font-bold text-sm text-white px-6 py-3.5 transition-all duration-200 hover:scale-[1.04] active:scale-95"
                                style={{
                                    border: "1.5px solid rgba(255,255,255,.30)",
                                    background: "transparent",
                                }}
                                onMouseEnter={(e) => {
                                    (e.currentTarget as HTMLAnchorElement).style.background =
                                        "rgba(255,255,255,.06)";
                                    (e.currentTarget as HTMLAnchorElement).style.borderColor =
                                        "rgba(255,255,255,.55)";
                                }}
                                onMouseLeave={(e) => {
                                    (e.currentTarget as HTMLAnchorElement).style.background =
                                        "transparent";
                                    (e.currentTarget as HTMLAnchorElement).style.borderColor =
                                        "rgba(255,255,255,.30)";
                                }}
                            >
                                Explore Vaults
                            </Link>
                        </div>
                    </div>

                    {/* Right: glass art */}
                    <div
                        className="relative flex items-center justify-center min-h-[280px] md:min-h-[400px]"
                        aria-hidden="true"
                    >
                        {/* Conic gradient ring */}
                        <div
                            className="absolute inset-0 m-auto rounded-full"
                            style={{
                                width: "min(92%, 440px)",
                                aspectRatio: "1",
                                background:
                                    "conic-gradient(from 200deg, rgba(222,201,244,.0) 0%, rgba(222,201,244,.35) 25%, rgba(211,255,180,.25) 55%, rgba(252,91,49,.18) 75%, rgba(222,201,244,.0) 100%)",
                                filter: "blur(28px)",
                                opacity: 0.85,
                            }}
                        />
                        <Image
                            src="/images/glass-02.png"
                            alt=""
                            width={420}
                            height={420}
                            className="relative w-[min(86%,420px)] h-auto"
                            style={{
                                filter:
                                    "drop-shadow(-12px 18px 50px rgba(218,242,236,.18)) drop-shadow(0 30px 60px rgba(0,0,0,.45))",
                            }}
                            priority
                        />
                    </div>
                </div>
            </div>
        </section>
    );
}

export default Hero;
