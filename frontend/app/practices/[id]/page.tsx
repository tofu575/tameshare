import Link from "next/link";
import { notFound } from "next/navigation";
import { ExperienceList } from "@/app/practices/_components/ExperienceList";
import { getExperienceRepository } from "@/lib/experience/getExperienceRepository";
import { getPracticeRepository } from "@/lib/practice/getPracticeRepository";

type Props = {
  params: Promise<{ id: string }>;
};

export default async function Page({ params }: Props) {
  const { id } = await params;
  const practiceRepository = await getPracticeRepository();
  const practice = await practiceRepository.findById(id);

  if (!practice) {
    notFound();
  }

  const experienceRepository = await getExperienceRepository();
  const experiences = await experienceRepository.findByPracticeId(id);
  const myExperience = experiences.find((experience) => experience.isMine);

  return (
    <main>
      <h1>{practice.title}</h1>

      <section>
        <h2>出典</h2>
        {practice.sources && practice.sources.length > 0 ? (
          <ul>
            {practice.sources.map((source) => (
              <li key={source.url}>
                <a href={source.url} target="_blank" rel="noreferrer">
                  {source.url}
                </a>
              </li>
            ))}
          </ul>
        ) : (
          <p>出典はまだ登録されていません。</p>
        )}
      </section>

      <section>
        <h2>みんなのExperience</h2>
        <p>これまでに {experiences.length} 件のExperienceがあります。</p>
        <ExperienceList experiences={experiences} />
      </section>

      <section>
        <h2>自分のExperience</h2>
        {myExperience ? (
          <>
            <p>{myExperience.note || "ひとことはありません。"}</p>
            <Link className="button-link" href={`/practices/${id}/experience`}>
              編集する
            </Link>
          </>
        ) : (
          <Link className="button-link" href={`/practices/${id}/experience`}>
            試した結果を残す
          </Link>
        )}
      </section>
    </main>
  );
}
