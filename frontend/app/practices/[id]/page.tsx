import { notFound } from "next/navigation";
import { getPracticeRepository } from "@/lib/practice/getPracticeRepository";

type Props = {
  params: Promise<{ id: string }>;
};

export default async function Page({ params }: Props) {
  const { id } = await params;
  const repository = await getPracticeRepository();
  const practice = await repository.findById(id);

  if (!practice) {
    notFound();
  }

  return (
    <main>
      <h1>{practice.title}</h1>
      <p>{practice.description}</p>
    </main>
  );
}
