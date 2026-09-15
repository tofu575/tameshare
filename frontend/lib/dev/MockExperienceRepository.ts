import "server-only";

import type {
  ExperienceRepository,
  SaveExperienceInput,
} from "@/lib/experience/ExperienceRepository";
import type { Experience } from "@/types/experience";
import { mockExperiences } from "./mockExperiences";

export class MockExperienceRepository implements ExperienceRepository {
  async findByPracticeId(practiceId: string): Promise<Experience[]> {
    return mockExperiences
      .filter((experience) => experience.practiceId === practiceId)
      .map((experience) => ({ ...experience }));
  }

  async findMineByPracticeId(practiceId: string): Promise<Experience | null> {
    const experience = mockExperiences.find(
      (item) => item.practiceId === practiceId && item.isMine,
    );
    return experience ? { ...experience } : null;
  }

  async findMine(): Promise<Experience[]> {
    return mockExperiences
      .filter((experience) => experience.isMine)
      .map((experience) => ({ ...experience }));
  }

  async saveMine(input: SaveExperienceInput): Promise<Experience> {
    const existingIndex = mockExperiences.findIndex(
      (item) => item.practiceId === input.practiceId && item.isMine,
    );
    const experience: Experience = {
      id:
        existingIndex >= 0
          ? mockExperiences[existingIndex].id
          : `experience-${mockExperiences.length + 1}`,
      ...input,
      userId: "current-user",
      isMine: true,
      createdAt:
        existingIndex >= 0
          ? mockExperiences[existingIndex].createdAt
          : new Date().toISOString(),
      updatedAt: new Date().toISOString(),
    };

    if (existingIndex >= 0) {
      mockExperiences[existingIndex] = experience;
    } else {
      mockExperiences.push(experience);
    }
    return { ...experience };
  }
}
