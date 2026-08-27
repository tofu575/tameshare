import { fetchPractices } from '@/lib/api/practice';
import { PracticeList } from '../_components/PracticeList';

export default async function Page() {
  const practices = await fetchPractices();

  return (
    <main>
      <h1>Practice一覧</h1>
      <PracticeList practices={practices} />
    </main>
  );
}
