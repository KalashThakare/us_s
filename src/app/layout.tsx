import "../app/globals.css"

import { Inter } from "next/font/google";
import { ThemeProvider } from "@/components/theme-provider";
import { Manrope } from "next/font/google";
import { cn } from "@/lib/utils";
import Header from "@/components/Header";
import { Footer } from "@/components/footer";


const inter = Inter({
  subsets: ["latin"],
  variable: "--font-sans",
});

const fontHeading = Manrope({
  subsets: ['latin'],
  display: 'swap',
  variable: '--font-heading',
})

const fontBody = Manrope({
  subsets: ['latin'],
  display: 'swap',
  variable: '--font-body',
})

export const metadata = {
  title: {
    default: "Uss | Local Community Engagement Platform",
  },
  icons: [{ rel: "icon", url: "./logo.png" }],
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body
        className={cn("antialiased", fontHeading.variable, fontBody.variable)}
      >
        <ThemeProvider
          attribute="class"
          defaultTheme="system"
          enableSystem
          disableTransitionOnChange
        >
          <Header />
          {children}
          <Footer/>
        </ThemeProvider>
      </body>
    </html>
  );
}
