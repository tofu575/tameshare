import PankuzuLayout from "./_components/pankuzu";

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <PankuzuLayout>
      <html lang="ja">
        <body>{children}</body>
      </html>
    </PankuzuLayout>
  );
}
