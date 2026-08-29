import { Breadcrumbs } from "@/app/_components/Breadcrumbs";

export default function PracticesLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <div className="practice-area">
      <p className="muted">Practice — 実際に試せる方法</p>
      <Breadcrumbs />
      {children}
    </div>
  );
}
