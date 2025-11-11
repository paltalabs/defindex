import Link from "next/link";

type ButtonVariant = "primary" | "secondary" | "outlined" | "white";

interface CTAButtonProps {
    variant?: ButtonVariant;
    href: string;
    children: React.ReactNode;
    className?: string;
    target?: string;
    rel?: string;
    style?: React.CSSProperties;
}

const variantStyles: Record<ButtonVariant, string> = {
    primary: "bg-lime-200 text-cyan-900 border-lime-200 hover:bg-brand-primary-hover hover:shadow-lg",
    secondary: "bg-cyan-950 text-white border-cyan-950 hover:bg-brand-dark-cyan-2 hover:shadow-lg",
    outlined: "bg-transparent border-lime-200 text-lime-200 hover:bg-lime-200/10 hover:shadow-lg",
    white: "bg-white border-white text-cyan-900 hover:bg-gray-50 hover:shadow-lg",
};

function CTAButton({
    variant = "primary",
    href,
    children,
    className = "",
    target,
    rel,
    style,
}: CTAButtonProps) {
    return (
        <Link
            href={href}
            className={`flex items-center justify-center font-manrope font-extrabold text-sm rounded-3xl px-6 py-4 border whitespace-nowrap transition-all duration-normal hover:scale-105 active:scale-95 ${variantStyles[variant]} ${className}`}
            target={target}
            rel={rel}
        >
            {children}
        </Link>
    );
}

export default CTAButton;
