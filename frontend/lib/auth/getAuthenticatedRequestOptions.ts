import "server-only";

import { getOrCreateAnonymousToken } from "./getOrCreateAnonymousToken";

// 変更系APIへ署名付きの匿名Bearer tokenを付与する。
export async function getAuthenticatedRequestOptions(): Promise<RequestInit> {
  const token = await getOrCreateAnonymousToken();
  return { headers: { Authorization: `Bearer ${token}` } };
}
