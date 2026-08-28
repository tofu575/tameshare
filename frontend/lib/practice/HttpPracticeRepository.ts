import "server-only";

import type { Practice } from "@/types/practice";
import type { PracticeRepository } from "./PracticeRepository";

export class HttpPracticeRepository implements PracticeRepository {
  private readonly baseUrl: string;

  constructor(baseUrl: string) {
    this.baseUrl = baseUrl.replace(/\/$/, "");
  }

  async findAll(): Promise<Practice[]> {
    const response = await fetch(`${this.baseUrl}/practices`);

    if (!response.ok) {
      throw new Error(`Failed to fetch practices: ${response.status}`);
    }

    return response.json();
  }

  async findById(id: string): Promise<Practice | null> {
    const response = await fetch(
      `${this.baseUrl}/practices/${encodeURIComponent(id)}`,
    );

    if (response.status === 404) {
      return null;
    }

    if (!response.ok) {
      throw new Error(`Failed to fetch practice: ${response.status}`);
    }

    return response.json();
  }
}
