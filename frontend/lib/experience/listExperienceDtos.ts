import "server-only";

import { ApiResponseError } from "@/lib/api/ApiResponseError";
import { listExperiences } from "@/lib/generated/api/experiences/experiences";
import type { Experience } from "@/lib/generated/model";

// 指定Practiceの全Experience DTOをGenerated Clientで取得する。
export async function listExperienceDtos(
  practiceId: string,
): Promise<Experience[]> {
  const items: Experience[] = [];
  let offset = 0;

  while (true) {
    const response = await listExperiences(practiceId, { limit: 100, offset });
    if (response.status !== 200) {
      throw new ApiResponseError(
        response.status,
        response.data.code,
        response.data.message,
      );
    }
    items.push(...response.data.items);
    if (!response.data.has_more) {
      return items;
    }
    offset += response.data.limit;
  }
}
