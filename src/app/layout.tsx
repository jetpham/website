import "~/styles/globals.css";

import { type Metadata, type Viewport } from "next";

export const metadata: Metadata = {
  title: "Jet Pham",
  description: "Jet Pham's personal website",
  appleWebApp: {
    title: "Jet Pham",
  },
};

export const viewport: Viewport = {
  width: "device-width",
  initialScale: 1,
  themeColor: "#000000",
};

export default function RootLayout({
  children,
}: Readonly<{ children: React.ReactNode }>) {
  return (
    <html lang="en">
      <body>
        {children}
      </body>
    </html>
  );
}
