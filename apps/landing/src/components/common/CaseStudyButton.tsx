import Link from "next/link";

type CaseStudyVariant = "light" | "dark";

interface CaseStudyButtonProps {
    variant?: CaseStudyVariant;
    href: string;
    className?: string;
}

const variantStyles: Record<CaseStudyVariant, string> = {
    light: "bg-lime-200 text-cyan-950 hover:bg-brand-primary-hover hover:shadow-md",
    dark: "bg-cyan-900 text-lime-200 hover:bg-cyan-950 hover:shadow-md",
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
            className={`inline-block px-3 py-1 rounded-full text-xs sm:text-sm font-semibold transition-all duration-normal hover:scale-105 active:scale-95 ${variantStyles[variant]} ${className}`}
        >
            Case Study
        </Link>
    );
}

export default CaseStudyButton;
