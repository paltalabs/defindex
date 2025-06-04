import Image from "next/image";

const steps = [
    {
        id: 1,
        title: "DeFindex operates on secure, transparent smart contracts",
        link: "https://drive.proton.me/urls/GYBBP5TS00#kCE6EHDN6sth"
    },
    {
        id: 2,
        title: "Every transaction is decentralized, giving you full control and peace of mind",
    },
];

function Security() {
    return (
        <section className="relative z-0">
            <Image
                width={260}
                height={255}
                quality={100}
                className="absolute xl:w-auto w-40 md:w-52 left-1/2 bottom-0 translate-y-[calc(50%+20px)] -translate-x-1/2"
                src="/images/stickers01 2.png"
                alt=""
            />
            <div className="container">
                <div className="max-w-[1440px] mx-autopt-[222px] relative z-0">
                    <Image
                        width={1440}
                        height={658}
                        className="absolute w-full h-full -z-10"
                        src="/images/3d-shapes-glowing-with-bright-holographic-colors 1.png"
                        alt=""
                    />
                    <div className="max-w-[1254px] pt-20 sm:pt-32 md:pt-44 xl:pt-[222px] pb-20 sm:pb-28 md:pb-40 xl:pb-[200px] mx-auto flex lg:flex-row flex-col items-center justify-between">
                        <div className="lg:max-w-[600px] lg:mb-0 mb-8">
                            <p className="uppercase font-familjen-grotesk text-[18px] md:text-[20px] md:text-lg leading-[1.2em] tracking-[-0.03em] text-uppercase text-blue-100">
                                Security & Transparency
                            </p>
                            <h2 className="font-familjen-grotesk text-[48px] md:text-[64px] leading-[1.11em] xl:text-3xl tracking-[-0.03em] text-linear bg-linear italic">
                                Secure, Transparent, <b>and Decentralized</b>
                            </h2>
                        </div>
                        <div className="lg:max-w-[422px]">
                            <ul className="flex gap-4 lg:gap-8 flex-col ">
                                {steps.map(({ id, title, link }) => (
                                    <li key={id}>
                                        <div className="flex gap-4 items-center">
                                            <span className="font-familjen-grotesk text-[#DEC9F4] -translate-y-[0.095em] font-bold text-[48px] md:text-[56px] xl:text-[64px]">
                                                {id}.
                                            </span>
                                            <div>
                                                <p className="font-inter-tight text-[18px] md:text-[22px] xl:text-lg text-blue-100">
                                                    {title}
                                                </p>
                                                {link && (
                                                    <a 
                                                        href={link}
                                                        target="_blank"
                                                        rel="noopener noreferrer"
                                                        className="text-blue-400 hover:text-blue-300 text-sm mt-1 inline-block"
                                                    >
                                                        View Audit Report →
                                                    </a>
                                                )}
                                            </div>
                                        </div>
                                    </li>
                                ))}
                            </ul>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    );
}

export default Security;
