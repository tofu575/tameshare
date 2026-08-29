import "server-only";

import { getOrCreateAnonymousUserId } from "./getOrCreateAnonymousUserId";

// 変更系APIへ匿名UserのBearer UUIDを付与する。
export async function getAuthenticatedRequestOptions(): Promise<RequestInit> {
  const userId = await getOrCreateAnonymousUserId();
  return { headers: { Authorization: `Bearer ${userId}` } };
}
