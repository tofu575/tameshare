import "server-only";

import type { PracticeRepository } from "@/lib/practice/PracticeRepository";
import type { Practice } from "@/types/practice";
import { mockPractices } from "./mockPractices";

export class MockPracticeRepository implements PracticeRepository {
  async findAll(): Promise<Practice[]> {
    await new Promise((resolve) => setTimeout(resolve, 300));

    return mockPractices.map((practice) => ({
      ...practice,
      sources: practice.sources?.map((source) => ({ ...source })),
    }));
  }

  async findById(id: string): Promise<Practice | null> {
    const practice = mockPractices.find((practice) => practice.id === id);

    return practice
      ? {
          ...practice,
          sources: practice.sources?.map((source) => ({ ...source })),
        }
      : null;
  }
}
