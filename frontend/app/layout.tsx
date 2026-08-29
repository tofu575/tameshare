import type { Metadata } from "next";
import { Footer } from "./_components/Footer";
import { Header } from "./_components/Header";
import "./globals.css";

export const metadata: Metadata = {
  title: "試Share",
  description: "実際に試せる方法と、試した体験を共有するサービス",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="ja">
      <body>
        <Header />
        {children}
        <Footer />
      </body>
    </html>
  );
}
