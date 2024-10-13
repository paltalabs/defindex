import React from "react";
import Link from "next/link";
import Image from "next/image";

interface Props {
    label: string;
    title: string;
    learn_more: string;
    thumb: string;
    strategies: {
        id: string | number;
        description: string;
        icon: string;
    }[];
    advices: { title: string; describe: string }[];
    color: "red" | "green";
}

function Solutions(props: Props) {
    const { advices, learn_more, thumb, label, title, strategies, color } = props;

    return (
        <div>
            <div className="flex gap-4 justify-center flex-col max-w-w-[629px] mb-16 lg:mb-20">
                <p className="font-familjen-grotesk text-[20px] lg:text-lg uppercase leading-[1.425em] tracking-[-0.03em] text-uppercase text-cyan-950">
                    {label}
                </p>
                <h4
                    className="font-familjen-grotesk max-w-[600px] text-[56px] xl:text-3xl leading-[1.04em] tracking-[-0.03em] text-cyan-950"
                    dangerouslySetInnerHTML={{
                        __html: title.replace(/\*\*(.*?)\*\*/g, "<b>$1</b>"),
                    }}
                />
                <div className="flex">
                    <Link
                        href={learn_more}
                        className="font-extrabold font-manrope text-[14px] xl:text-xs leading-none xl:leading-none text-cyan-950 rounded-3xl bg-lime-200 px-6 py-4 xl:p-6 flex gap-2.5 items-center justify-center"
                    >
                        Learn More About Integration
                    </Link>
                </div>
            </div>
            <div className="grid sm:grid-cols-4 mb-12 md:mb-16 lg:mb-20">
                <Image
                    width={304}
                    height={329}
                    src={thumb}
                    className="rounded-t-xl sm:rounded-t-none sm:rounded-tl-3xl sm:rounded-l-3xl sm:w-auto w-full"
                    alt=""
                />
                {strategies.map(({ icon, description, id }) => (
                    <div
                        key={id}
                        className={`${color === "red" ? "[&:nth-child(2)]:bg-item-linear" : "[&:nth-child(2)]:bg-item-linear-green"} group grid place-content-center border ${color === "red" ? "border-orange-500/40" : "border-[#014751]/40"} p-4 xl:p-10 sm:last:rounded-r-3xl last:rounded-b-xl sm:last:rounded-b-none sm:last:rounded-br-3xl sm:border-r-0 border-b-0 sm:border-b last:border-b last:border-r`}
                    >
                        <div className="mb-6 flex justify-center group-[&:nth-child(n+3)]:opacity-30 group-[&:nth-child(n+3)]:contrast-0">
                            <img src={icon} alt="" className="md:w-auto w-6" />
                        </div>
                        <p className="font-inter-tight text-center text-[16px] xl:text-md group-[&:nth-child(n+3)]:text-opacity-30 text-cyan-950 line-clamp-3">
                            {description}
                        </p>
                    </div>
                ))}
            </div>
            <div className="flex gap-20">
                {advices.map(({ describe, title }, index) => (
                    <div key={index} className="">
                        <b className="font-extrabold font-inter-tight text-[16px] md:text-[20px] xl:text-lg leading-[1.6075em] text-cyan-900/80">
                            {title}
                        </b>{" "}
                        <p className="inline font-inter-tight text-[16px] md:text-[20px] xl:text-lg leading-[1.6075em] text-cyan-900/80">
                            {describe}
                        </p>
                    </div>
                ))}
            </div>
        </div>
    );
}

export default Solutions;
