import type { Experience } from "@/types/experience";

export type SaveExperienceInput = {
  practiceId: string;
  practiceTitle: string;
  note: string | null;
};

export interface ExperienceRepository {
  findByPracticeId(practiceId: string): Promise<Experience[]>;
  findMineByPracticeId(practiceId: string): Promise<Experience | null>;
  findMine(): Promise<Experience[]>;
  saveMine(input: SaveExperienceInput): Promise<Experience>;
}
