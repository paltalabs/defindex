import React, { ReactNode } from "react";
import Navbar from "./navbar/Navbar";
import Index from "./Footer";

interface LayoutProps {
    children?: ReactNode;
}

function Layout({ children }: LayoutProps) {
    return (
        <div>
            <Navbar />
            {children}
            <Index />
        </div>
    );
}

export default Layout;
