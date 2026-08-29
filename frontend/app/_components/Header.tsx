import Link from "next/link";

export function Header() {
  return (
    <header className="site-header">
      <Link className="site-title" href="/">
        試Share
      </Link>
      <nav aria-label="メインナビゲーション">
        <ul>
          <li>
            <Link href="/practices">Practice</Link>
          </li>
          <li>
            <Link href="/me/experiences">自分の体験</Link>
          </li>
          <li>
            <Link href="/requests/new">掲載リクエスト</Link>
          </li>
        </ul>
      </nav>
    </header>
  );
}
