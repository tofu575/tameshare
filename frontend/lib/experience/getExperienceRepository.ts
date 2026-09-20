import "server-only";

import type { ExperienceRepository } from "./ExperienceRepository";

// Generated Clientを利用するExperience Repositoryを返す。
export async function getExperienceRepository(): Promise<ExperienceRepository> {
  const { ApiExperienceRepository } = await import("./ApiExperienceRepository");
  return new ApiExperienceRepository();
}
