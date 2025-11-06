import Link from "next/link";

type CaseStudyVariant = "light" | "dark";

interface CaseStudyButtonProps {
    variant?: CaseStudyVariant;
    href: string;
    className?: string;
}

const variantStyles: Record<CaseStudyVariant, string> = {
    light: "bg-[#D3FFB4] text-[#014751] hover:bg-[#E5FFCF] hover:shadow-md",
    dark: "bg-[#033036] text-[#D3FFB4] hover:bg-[#014751] hover:shadow-md",
};

function CaseStudyButton({
    variant = "light",
    href,
    className = "",
}: CaseStudyButtonProps) {
    return (
        <Link
            href={href}
            target="_blank"
            rel="noopener noreferrer"
            className={`inline-block px-3 py-1 rounded-full text-xs sm:text-sm font-semibold transition-all duration-200 hover:scale-105 active:scale-95 ${variantStyles[variant]} ${className}`}
        >
            Case Study
        </Link>
    );
}

export default CaseStudyButton;
