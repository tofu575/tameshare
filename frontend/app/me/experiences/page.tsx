import Link from "next/link";
import { getExperienceRepository } from "@/lib/experience/getExperienceRepository";

export const dynamic = "force-dynamic";

export default async function Page() {
  const repository = await getExperienceRepository();
  const experiences = await repository.findMine();

  return (
    <main>
      <h1>自分が試したもの</h1>
      {experiences.length > 0 ? (
        <ul className="card-list">
          {experiences.map((experience) => (
            <li key={experience.id}>
              <article className="card">
                <h2>{experience.practiceTitle}</h2>
                <p>{experience.note || "ひとことはありません。"}</p>
                <div className="actions">
                  <Link href={`/practices/${experience.practiceId}`}>
                    Practiceを見る
                  </Link>
                  <Link href={`/practices/${experience.practiceId}/experience`}>
                    編集する
                  </Link>
                </div>
              </article>
            </li>
          ))}
        </ul>
      ) : (
        <p>記録したExperienceはまだありません。</p>
      )}
    </main>
  );
}
