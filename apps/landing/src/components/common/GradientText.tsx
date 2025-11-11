import { ReactNode } from "react";
import { GRADIENT_OBJECTS } from "@/constants/design";

type GradientVariant = "primary" | "secondary" | "tertiary" | "purple" | "green";

interface GradientTextProps {
    variant?: GradientVariant;
    children: ReactNode;
    className?: string;
    as?: "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "p" | "span";
    style?: React.CSSProperties;
    textStroke?: string; // Color for text stroke (e.g., COLORS.dark)
}

function GradientText({
    variant = "primary",
    children,
    className = "",
    as: Component = "span",
    style: style = {},
    textStroke,
}: GradientTextProps) {

    return (
        <Component
            className={className}
            style={{
                position: 'relative',
                display: 'inline-block',
            }}
        >
            {textStroke && (
                <div
                    aria-hidden="true"
                    style={{
                        position: 'absolute',
                        pointerEvents: 'none',
                        color: textStroke,
                        top: 0,
                        left: -1,
                        zIndex: -1,
                        textShadow: `1px 1px 16px ${textStroke}, -1px 1px 16px ${textStroke}, 1px -1px 16px ${textStroke}, -1px -1px 16px ${textStroke}`,
                        opacity: 0.6,
                        ...style
                    }}
                >
                    {children}
                </div>
            )}
            <div
                style={{
                    ...GRADIENT_OBJECTS[variant],
                    ...style
                }}
            >
                {children}
            </div>
        </Component>
    );
}

export default GradientText;
