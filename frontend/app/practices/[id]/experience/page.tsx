import { notFound } from "next/navigation";
import { getExperienceRepository } from "@/lib/experience/getExperienceRepository";
import { getPracticeRepository } from "@/lib/practice/getPracticeRepository";
import { ExperienceForm } from "./_components/ExperienceForm";

type Props = { params: Promise<{ id: string }> };

export default async function Page({ params }: Props) {
  const { id } = await params;
  const practiceRepository = await getPracticeRepository();
  const practice = await practiceRepository.findById(id);
  if (!practice) notFound();

  const experienceRepository = await getExperienceRepository();
  const experience = await experienceRepository.findMineByPracticeId(id);

  return (
    <main>
      <h1>「{practice.title}」を試した結果</h1>
      <p>現在のAPI仕様では、任意の短いひとことを記録します。</p>
      <ExperienceForm
        practiceId={practice.id}
        practiceTitle={practice.title}
        experience={experience}
      />
    </main>
  );
}
