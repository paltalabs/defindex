import Link from "next/link";

interface ScheduleDemoButtonProps {
    className?: string;
}

function ScheduleDemoButton({ className = "" }: ScheduleDemoButtonProps) {
    return (
        <Link
            href="mailto:dev@paltalabs.io"
            className={`flex items-center justify-center bg-[#D3FFB4] text-[#033036] font-manrope font-[800] text-sm rounded-3xl px-6 py-4 sm:py-6 border border-[#D3FFB4] whitespace-nowrap transition-all duration-200 hover:scale-105 hover:shadow-lg hover:bg-[#E5FFCF] active:scale-95 ${className}`}
        >
            Schedule a Demo
        </Link>
    );
}

export default ScheduleDemoButton;
