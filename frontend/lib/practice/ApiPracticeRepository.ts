import "server-only";

import { ApiResponseError } from "@/lib/api/ApiResponseError";
import { getPractice } from "@/lib/generated/api/practices/practices";
import type { Practice } from "@/types/practice";
import { listPracticeDtos } from "./listPracticeDtos";
import type { PracticeRepository } from "./PracticeRepository";

// Generated ClientからUI用Practiceを取得するRepository。
export class ApiPracticeRepository implements PracticeRepository {
  /** Practiceを全件取得する。 */
  async findAll(): Promise<Practice[]> {
    return (await listPracticeDtos()).map((practice) => ({
      id: practice.id,
      title: practice.title,
      createdAt: practice.created_at,
    }));
  }

  /** Sourceを含むPractice詳細を取得する。 */
  async findById(id: string): Promise<Practice | null> {
    const response = await getPractice(id);
    if (response.status === 404) {
      return null;
    }
    if (response.status !== 200) {
      throw new ApiResponseError(
        response.status,
        response.data.code,
        response.data.message,
      );
    }
    return {
      id: response.data.id,
      title: response.data.title,
      createdAt: response.data.created_at,
      sources: response.data.sources,
    };
  }
}
