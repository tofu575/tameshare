import type { Practice } from "@/types/practice";

export interface PracticeRepository {
  findAll(): Promise<Practice[]>;
  findById(id: string): Promise<Practice | null>;
}
