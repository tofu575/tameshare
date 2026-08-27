import type { Practice } from '@/types/practice';

type Props = {
  practices: Practice[];
};

export const PracticeList = ({ practices }: Props) => {
  return (
    <ul>
      {practices.map((practice) => (
        <li key={practice.id}>{practice.title}</li>
      ))}
    </ul>
  );
};
