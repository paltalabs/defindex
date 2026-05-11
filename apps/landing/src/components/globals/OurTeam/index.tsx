import Image from "next/image";
import Link from "next/link";

export default function OurTeam() {
    return (
        <section id="built-by-palta-labs" className="px-4 sm:px-6 pt-0 pb-20 md:pb-28">
            <div className="max-w-[1180px] mx-auto">
                <div
                    className="relative overflow-hidden rounded-[32px] grid grid-cols-1 gap-7 p-12 md:p-[72px] md:grid-cols-[1.2fr_1fr] md:items-center"
                    style={{
                        background: "linear-gradient(150deg, #EFE3FB 0%, #DEC9F4 50%, #C9B5E8 100%)",
                        color: "#014751",
                    }}
                >
                    {/* Text column */}
                    <div>
                        <h3
                            className="font-familjen-grotesk font-bold leading-[1.18] mb-4"
                            style={{
                                fontSize: "clamp(28px, 3.6vw, 44px)",
                                color: "#014751",
                                letterSpacing: "-0.02em",
                            }}
                        >
                            Built by{" "}
                            <em style={{ fontStyle: "italic", color: "#FC5B31" }}>
                                PaltaLabs
                            </em>
                        </h3>

                        <p
                            className="font-inter-tight leading-[1.6] mb-6"
                            style={{ fontSize: 16, color: "#014751", opacity: 0.8 }}
                        >
                            A team of DeFi builders focused on making stablecoin yield accessible,
                            automated, and safe. We&apos;re committed to transparency, security, and
                            helping developers build the future of decentralized finance.
                        </p>

                        <Link
                            href="https://paltalabs.io"
                            target="_blank"
                            rel="noopener noreferrer"
                            className="inline-flex items-center gap-2 font-inter-tight font-bold text-[15px] text-white rounded-full px-[22px] py-[14px] transition-all duration-200 hover:scale-[1.04] active:scale-95 whitespace-nowrap"
                            style={{ background: "#014751", border: "1.5px solid #014751" }}
                        >
                            Learn more about PaltaLabs
                        </Link>
                    </div>

                    {/* Glass image column */}
                    <div className="flex justify-center">
                        <Image
                            src="/images/glass-08.png"
                            alt="PaltaLabs visual"
                            width={260}
                            height={260}
                            className="h-auto"
                            style={{
                                width: "min(80%, 260px)",
                                filter: "drop-shadow(-12px 18px 30px rgba(1,71,81,.18))",
                            }}
                        />
                    </div>
                </div>
            </div>
        </section>
    );
}
