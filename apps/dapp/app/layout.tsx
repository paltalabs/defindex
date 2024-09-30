import { ReactNode } from "react";
import { Providers } from "../src/providers/main-provider";
export default function RootLayout({
  children,
}: {
    children: ReactNode,
}) {
  return (
    <html lang='en'>
      <body>
        <Providers>{children}</Providers>
      </body>
    </html>
  )
}