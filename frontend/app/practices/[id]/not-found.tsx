import Link from "next/link";

export default function NotFound() {
  return (
    <main>
      <h1>Practiceが見つかりません</h1>
      <p>指定されたPracticeは存在しないか、公開されていません。</p>
      <Link href="/practices">Practice一覧へ戻る</Link>
    </main>
  );
}
