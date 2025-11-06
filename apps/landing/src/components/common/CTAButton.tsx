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
    primary: "bg-[#D3FFB4] text-[#033036] border-[#D3FFB4] hover:bg-[#E5FFCF] hover:shadow-lg",
    secondary: "bg-[#014751] text-white border-[#014751] hover:bg-[#025A66] hover:shadow-lg",
    outlined: "bg-transparent border-[#D3FFB4] text-[#D3FFB4] hover:bg-[#D3FFB4]/10 hover:shadow-lg",
    white: "bg-white border-white text-[#033036] hover:bg-gray-50 hover:shadow-lg",
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
            className={`flex items-center justify-center font-manrope font-extrabold text-sm rounded-3xl px-6 py-4 border whitespace-nowrap transition-all duration-200 hover:scale-105 active:scale-95 ${variantStyles[variant]} ${className}`}
            target={target}
            rel={rel}
        >
            {children}
        </Link>
    );
}

export default CTAButton;
