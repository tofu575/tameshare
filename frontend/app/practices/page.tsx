import { PracticeList } from "@/app/practices/_components/PracticeList";
import { getPracticeRepository } from "@/lib/practice/getPracticeRepository";

export const dynamic = "force-dynamic";

export default async function Page() {
  const repository = await getPracticeRepository();
  const practices = await repository.findAll();

  return (
    <main>
      <h1>Practiceを探す</h1>
      <p>登録されている、実際に試せる方法の一覧です。</p>
      <PracticeList practices={practices} />
    </main>
  );
}
