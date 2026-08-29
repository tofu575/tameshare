import Link from "next/link";
import { getPracticeRepository } from "@/lib/practice/getPracticeRepository";

export const dynamic = "force-dynamic";

export default async function Page() {
  const repository = await getPracticeRepository();
  const recentPractices = (await repository.findAll()).slice(0, 3);

  return (
    <main>
      <section className="hero">
        <h1>試Share</h1>
        <p>試してみたことを、みんなで共有する。</p>
        <Link className="button-link" href="/practices">
          Practiceを見てみる
        </Link>
      </section>

      <section>
        <h2>最近追加されたPractice</h2>
        <ul>
          {recentPractices.map((practice) => (
            <li key={practice.id}>
              <Link href={`/practices/${practice.id}`}>{practice.title}</Link>
            </li>
          ))}
        </ul>
      </section>

      <section className="actions">
        <Link href="/requests/new">掲載してほしい情報を送る</Link>
      </section>
    </main>
  );
}
