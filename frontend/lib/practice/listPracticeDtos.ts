import "server-only";

import { ApiResponseError } from "@/lib/api/ApiResponseError";
import { listPractices } from "@/lib/generated/api/practices/practices";
import type { Practice } from "@/lib/generated/model";

// 全ページのPractice DTOをGenerated Clientで取得する。
export async function listPracticeDtos(): Promise<Practice[]> {
  const items: Practice[] = [];
  let offset = 0;

  while (true) {
    const response = await listPractices({ limit: 100, offset });
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
