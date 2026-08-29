import "server-only";

import type { ExperienceRepository } from "./ExperienceRepository";

export async function getExperienceRepository(): Promise<ExperienceRepository> {
  const { MockExperienceRepository } = await import(
    "@/lib/dev/MockExperienceRepository"
  );
  return new MockExperienceRepository();
}
