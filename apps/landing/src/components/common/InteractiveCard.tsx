import Image from "next/image";

interface InteractiveCardProps {
    id: number;
    isActive: boolean;
    onClick: () => void;
    icon: string;
    iconAlt: string;
    title: string;
    description: string;
    activeGradient?: string;
    inactiveBackground?: string;
    iconClassName?: string;
    iconFilter?: string;
    cardClassName?: string;
    textAlign?: "left" | "center";
}

function InteractiveCard({
    isActive,
    onClick,
    icon,
    iconAlt,
    title,
    description,
    activeGradient = "linear-gradient(135deg, #D3FFB4 0%, #024852 100%)",
    inactiveBackground = "bg-[#033036]",
    iconClassName = "w-12 h-12 sm:w-16 sm:h-16",
    iconFilter,
    cardClassName = "",
    textAlign = "left",
}: InteractiveCardProps) {
    return (
        <button
            onClick={onClick}
            className={`border border-[#D3FFB4] p-4 sm:p-6 flex flex-col items-center gap-3 sm:gap-4 transition-all duration-300 cursor-pointer active:scale-95 ${
                isActive
                    ? "scale-105 shadow-lg"
                    : `${inactiveBackground} hover:bg-[#044852] hover:scale-105 hover:shadow-md`
            } ${cardClassName}`}
            style={
                isActive && activeGradient !== "transparent"
                    ? {
                          background: activeGradient,
                      }
                    : {
                        border: '0.1px solid rgba(211, 255, 180, 0.3)'
                    }
            }
        >
            <div className="flex-shrink-0">
                <Image
                    src={icon}
                    alt={iconAlt}
                    width={96}
                    height={96}
                    className={iconClassName}
                    style={
                        iconFilter && isActive
                            ? { filter: iconFilter }
                            : iconFilter && !isActive
                            ? {
                                  filter: "brightness(0) saturate(100%) invert(100%) sepia(0%) saturate(0%) hue-rotate(0deg) brightness(100%) contrast(100%)",
                              }
                            : undefined
                    }
                />
            </div>
            <div className={`text-${textAlign}`}>
                <h3
                    className={`font-familjen-grotesk text-base sm:text-lg font-semibold mb-1 sm:mb-2 ${
                        isActive ? "text-[#033036]" : "text-white"
                    }`}
                >
                    {title}
                </h3>
                <p
                    className={`font-inter text-xs sm:text-sm ${
                        isActive ? "text-[#033036]/80" : "text-white/80"
                    }`}
                >
                    {description}
                </p>
            </div>
        </button>
    );
}

export default InteractiveCard;
