import Link from "next/link";

export function Breadcrumbs() {
  return (
    <nav className="breadcrumbs" aria-label="パンくず">
      <ol>
        <li>
          <Link href="/">ホーム</Link>
        </li>
        <li>
          <Link href="/practices">Practice</Link>
        </li>
      </ol>
    </nav>
  );
}
