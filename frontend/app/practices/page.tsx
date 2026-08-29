import { PracticeList } from "@/app/practices/_components/PracticeList";
import { getPracticeRepository } from "@/lib/practice/getPracticeRepository";

export const dynamic = "force-dynamic";

export default async function Page() {
  const repository = await getPracticeRepository();
  const practices = await repository.findAll();

  return (
    <main>
      <h1>Practice一覧</h1>
      <PracticeList practices={practices} />
    </main>
  );
}
