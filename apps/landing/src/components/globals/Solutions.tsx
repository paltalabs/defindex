import Link from "next/link";

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

import { useEffect, useState } from "react";

function Solutions(props: Props) {
    const { advices, learn_more, thumb, label, title, strategies, color } = props;
    const [activeStrategy, setActiveStrategy] = useState<string | number>(strategies[0]?.id);

    const [visible, setVisible] = useState(false);

    useEffect(() => {
        setVisible(true);
    }, [activeStrategy]);

    return (
        <div className={`
                transition-opacity duration-1000
                ${visible ? "opacity-100" : "opacity-0"}
            `}>
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
                <img
                    loading="lazy"
                    width={'auto'}
                    height={'full'}
                    src={thumb}
                    className="rounded-t-xl sm:rounded-t-none sm:rounded-tl-3xl sm:rounded-l-3xl sm:w-auto w-full"
                    alt=""
                />
                {strategies.map(({ icon, description, id }) => (
                    <div
                        key={id}
                        className={`
                            group grid place-content-center border p-4 xl:p-10 sm:last:rounded-r-3xl last:rounded-b-xl sm:last:rounded-b-none sm:last:rounded-br-3xl sm:border-r-0 border-b-0 sm:border-b last:border-b last:border-r
                            ${
                                activeStrategy === id
                                    ? (color === "red"
                                        ? "bg-orange-500/20 border-orange-500"
                                        : "bg-green-700/20 border-green-700")
                                    : (color === "red"
                                        ? "border-orange-500/40"
                                        : "border-green-700/40 opacity-50")
                            }
                        `}
                        onClick={() => setActiveStrategy(id)}
                    >
                        <div className={`mb-6 flex justify-center ${activeStrategy !== id ? "opacity-30 contrast-0" : ""}`}>
                            <img src={icon} alt="" className="md:w-auto w-6" />
                        </div>
                        <p className={`font-inter-tight text-center text-[16px] xl:text-md text-cyan-950 line-clamp-3 ${activeStrategy !== id ? "text-opacity-30" : ""}`}>
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
