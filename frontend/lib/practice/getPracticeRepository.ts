import "server-only";

import type { PracticeRepository } from "./PracticeRepository";

export async function getPracticeRepository(): Promise<PracticeRepository> {
  const useMockApi = process.env.USE_MOCK_API === "true";

  if (process.env.NODE_ENV === "production" && useMockApi) {
    throw new Error("USE_MOCK_API=true is not allowed in production");
  }

  if (useMockApi) {
    const { MockPracticeRepository } = await import(
      "@/lib/dev/MockPracticeRepository"
    );

    return new MockPracticeRepository();
  }

  const backendApiUrl = process.env.BACKEND_API_URL;

  if (!backendApiUrl) {
    throw new Error("BACKEND_API_URL is required when the HTTP API is used");
  }

  const { HttpPracticeRepository } = await import("./HttpPracticeRepository");

  return new HttpPracticeRepository(backendApiUrl);
}
