import Link from "next/link";
import type { Practice } from "@/types/practice";

type Props = {
  practices: Practice[];
};

export const PracticeList = ({ practices }: Props) => {
  if (practices.length === 0) {
    return <p>Practiceはまだ登録されていません。</p>;
  }

  return (
    <ul className="card-list">
      {practices.map((practice) => (
        <li key={practice.id}>
          <article className="card">
            <h2>{practice.title}</h2>
            <Link href={`/practices/${practice.id}`}>詳細を見る</Link>
          </article>
        </li>
      ))}
    </ul>
  );
};
