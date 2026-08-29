import type { Practice } from '@/types/practice';
import Link from 'next/link';

type Props = {
  practices: Practice[];
};

export const PracticeList = ({ practices }: Props) => {
  return (
    <ul>
      {practices.map((practice) => (
        <Link href={`/practices/${practice.id}`}>
          <li key={practice.id}>{practice.title}</li>
        </Link>
      ))}
    </ul>
  );
};
