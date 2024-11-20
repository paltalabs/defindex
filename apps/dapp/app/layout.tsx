import { ReactNode } from "react";
import { Providers } from "../src/providers/main-provider";
export default function RootLayout({
  children,
}: {
    children: ReactNode,
}) {
  return (
    <html lang='en' className="dark" style={{ colorScheme: "dark" }} suppressHydrationWarning>
      <body>
        <Providers>{children}</Providers>
      </body>
    </html>
  )
}