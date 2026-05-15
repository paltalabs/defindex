"use client";
import { useState } from "react";

type SubscribeStatus = "idle" | "loading" | "success" | "error";

function validateEmail(email: string): boolean {
    return /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email.trim());
}

export interface NewsletterSubscribePayload {
    email: string;
}

interface NewsletterFormProps {
    onSubscribe?: (payload: NewsletterSubscribePayload) => Promise<void>;
}

export default function NewsletterForm({ onSubscribe }: NewsletterFormProps) {
    const [email, setEmail] = useState("");
    const [status, setStatus] = useState<SubscribeStatus>("idle");
    const [validationError, setValidationError] = useState("");

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        setValidationError("");

        if (!validateEmail(email)) {
            setValidationError("Please enter a valid email address.");
            return;
        }

        setStatus("loading");
        try {
            if (onSubscribe) {
                await onSubscribe({ email: email.trim() });
            }
            setStatus("success");
        } catch {
            setStatus("error");
        }
    };

    return (
        <section className="py-16 px-4 sm:px-6 lg:px-8">
            <div
                className="relative rounded-[28px] px-6 md:px-12 lg:px-16 py-12 sm:py-16 w-full overflow-hidden"
                style={{
                    background: "rgba(255,255,255,.04)",
                    border: "1px solid rgba(255,255,255,.08)",
                    backdropFilter: "blur(12px)",
                    WebkitBackdropFilter: "blur(12px)",
                }}
            >
                {/* Top border accent */}
                <div
                    style={{
                        height: "2px",
                        background: "linear-gradient(90deg, transparent 0%, #D3FFB4 50%, transparent 100%)",
                        position: "absolute",
                        top: 0,
                        left: "10%",
                        right: "10%",
                    }}
                />

                {status === "success" ? (
                    <div className="text-center py-4">
                        <div
                            className="w-16 h-16 mx-auto mb-6 rounded-full flex items-center justify-center"
                            style={{ background: "rgba(211, 255, 180, 0.15)", border: "1px solid rgba(211, 255, 180, 0.3)" }}
                        >
                            <svg className="w-8 h-8" viewBox="0 0 24 24" fill="none" stroke="#D3FFB4" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round">
                                <polyline points="20 6 9 17 4 12" />
                            </svg>
                        </div>
                        <h3 className="font-familjen-grotesk text-2xl sm:text-3xl font-semibold text-white mb-3">
                            You&apos;re subscribed!
                        </h3>
                        <p className="font-inter text-white/70 text-sm sm:text-base max-w-md mx-auto">
                            Thanks for joining. We&apos;ll keep you updated with the latest DeFindex news and updates.
                        </p>
                    </div>
                ) : (
                    <div className="flex flex-col items-center text-center gap-8">
                        <div className="max-w-xl">
                            <h2 className="font-familjen-grotesk text-[28px] sm:text-[38px] md:text-[48px] font-normal text-[#D3FFB4] mb-4">
                                Stay in the loop
                            </h2>
                            <p className="font-inter text-white/70 text-sm sm:text-base leading-relaxed">
                                Subscribe to our newsletter and be the first to hear about DeFindex updates, new strategies, and ecosystem news.
                            </p>
                        </div>

                        <form
                            onSubmit={handleSubmit}
                            className="w-full max-w-lg flex flex-col sm:flex-row gap-3"
                            noValidate
                        >
                            <div className="flex-1 flex flex-col gap-1">
                                <input
                                    type="email"
                                    value={email}
                                    onChange={(e) => {
                                        setEmail(e.target.value);
                                        if (validationError) setValidationError("");
                                        if (status === "error") setStatus("idle");
                                    }}
                                    placeholder="your@email.com"
                                    aria-label="Email address"
                                    className="w-full px-4 py-3 rounded-xl text-white text-sm font-inter placeholder-white/40 focus:outline-none transition-all duration-200"
                                    style={{
                                        background: "rgba(255,255,255,0.07)",
                                        border: validationError
                                            ? "1px solid rgba(252, 91, 49, 0.6)"
                                            : "1px solid rgba(255,255,255,0.12)",
                                    }}
                                    disabled={status === "loading"}
                                />
                                {validationError && (
                                    <p className="text-left text-xs font-inter" style={{ color: "#FC5B31" }}>
                                        {validationError}
                                    </p>
                                )}
                                {status === "error" && !validationError && (
                                    <p className="text-left text-xs font-inter" style={{ color: "#FC5B31" }}>
                                        Something went wrong. Please try again.
                                    </p>
                                )}
                            </div>

                            <button
                                type="submit"
                                disabled={status === "loading"}
                                className="shrink-0 px-6 py-3 rounded-xl font-manrope font-extrabold text-sm transition-all duration-200"
                                style={{
                                    background: status === "loading" ? "rgba(211, 255, 180, 0.6)" : "#D3FFB4",
                                    color: "#033036",
                                    cursor: status === "loading" ? "not-allowed" : "pointer",
                                    transform: "translateY(0)",
                                }}
                                onMouseEnter={(e) => {
                                    if (status !== "loading") {
                                        (e.currentTarget as HTMLButtonElement).style.background = "#E5FFCF";
                                        (e.currentTarget as HTMLButtonElement).style.transform = "translateY(-1px)";
                                    }
                                }}
                                onMouseLeave={(e) => {
                                    (e.currentTarget as HTMLButtonElement).style.background = status === "loading" ? "rgba(211, 255, 180, 0.6)" : "#D3FFB4";
                                    (e.currentTarget as HTMLButtonElement).style.transform = "translateY(0)";
                                }}
                            >
                                {status === "loading" ? "Subscribing…" : "Subscribe"}
                            </button>
                        </form>

                        <p className="font-inter text-xs text-white/40 max-w-sm">
                            No spam, ever. Unsubscribe at any time.
                        </p>
                    </div>
                )}
            </div>
        </section>
    );
}
