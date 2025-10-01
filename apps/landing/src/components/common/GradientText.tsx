import { ReactNode } from "react";

type GradientVariant = "primary" | "secondary" | "tertiary" | "purple" | "green";

interface GradientTextProps {
    variant?: GradientVariant;
    children: ReactNode;
    className?: string;
    as?: "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "p" | "span";
    style?: React.CSSProperties;
}

const gradients: Record<GradientVariant, string> = {
    primary: "linear-gradient(137deg, #FFFFFF 7%, #DEC9F4 50%, rgba(2, 72, 82, 1) 100%)",
    secondary: "linear-gradient(91deg, #FFF -5.52%, #DEC9F4 84.31%, #024852 101.37%)",
    tertiary: "linear-gradient(90deg, #FFF -5.52%, #D3FBFF 90%, #024852 104%)",
    purple: "linear-gradient(137deg, #DEC9F4 7%, #024852 100%)",
    green: "linear-gradient(90deg, rgba(211, 255, 180, 1) 5%,  rgba(2, 72, 82, 1) 85%)",
};

function GradientText({
    variant = "primary",
    children,
    className = "",
    as: Component = "span",
    style: style = {},
}: GradientTextProps) {
    return (
        <Component
            className={className}
            style={{
                background: gradients[variant],
                WebkitBackgroundClip: "text",
                WebkitTextFillColor: "transparent",
                backgroundClip: "text",
                ...style
            }}
        >
            {children}
        </Component>
    );
}

export default GradientText;
