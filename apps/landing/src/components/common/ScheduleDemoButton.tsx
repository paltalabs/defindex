import Link from "next/link";

interface ScheduleDemoButtonProps {
    className?: string;
}

function ScheduleDemoButton({ className = "" }: ScheduleDemoButtonProps) {
    return (
        <Link
            href="https://cal.com/devmonsterblock"
            target="_blank"
            aria-label="Schedule a demo with DeFindex team to learn about Stellar yield integration"
            rel="noopener noreferrer"
            className={`flex items-center justify-center bg-lime-200 text-cyan-900 font-manrope font-extrabold text-sm rounded-3xl px-6 py-4 sm:py-6 border border-lime-200 whitespace-nowrap transition-all duration-normal hover:scale-105 hover:shadow-lg hover:bg-brand-primary-hover active:scale-95 ${className}`}
        >
            Schedule a Demo
        </Link>
    );
}

export default ScheduleDemoButton;
