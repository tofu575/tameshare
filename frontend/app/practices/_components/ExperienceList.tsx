import type { Experience } from "@/types/experience";

export function ExperienceList({ experiences }: { experiences: Experience[] }) {
  if (experiences.length === 0) {
    return <p>まだExperienceはありません。</p>;
  }

  return (
    <ul className="card-list">
      {experiences.map((experience) => (
        <li className="card" key={experience.id}>
          <p>{experience.note || "ひとことはありません。"}</p>
          <small className="muted">
            {experience.isMine ? "自分のExperience" : "みんなのExperience"}
          </small>
        </li>
      ))}
    </ul>
  );
}
