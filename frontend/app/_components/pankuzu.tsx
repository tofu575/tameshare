// app/practices/layout.tsx

import Link from "next/link";

export default function PankuzuLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <>
      <nav aria-label="パンくず">
        <ol>
          <li>
            <Link href="/">ホーム</Link>
          </li>
          <li>
            <Link href="/practices/practice-1">試すこと一覧</Link>
          </li>
        </ol>
      </nav>

      {children}
    </>
  );
}
