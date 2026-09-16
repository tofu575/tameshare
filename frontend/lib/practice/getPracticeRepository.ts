import "server-only";

import type { PracticeRepository } from "./PracticeRepository";

// Generated Clientを利用するPractice Repositoryを返す。
export async function getPracticeRepository(): Promise<PracticeRepository> {
  const { ApiPracticeRepository } = await import("./ApiPracticeRepository");
  return new ApiPracticeRepository();
}
