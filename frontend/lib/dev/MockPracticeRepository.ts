import "server-only";

import type { PracticeRepository } from "@/lib/practice/PracticeRepository";
import type { Practice } from "@/types/practice";
import { mockPractices } from "./mockPractices";

export class MockPracticeRepository implements PracticeRepository {
  async findAll(): Promise<Practice[]> {
    return mockPractices.map((practice) => ({ ...practice }));
  }

  async findById(id: string): Promise<Practice | null> {
    const practice = mockPractices.find((practice) => practice.id === id);

    return practice ? { ...practice } : null;
  }
}
