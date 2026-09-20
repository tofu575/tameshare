import "server-only";

import { cookies } from "next/headers";
import { anonymousUserCookie } from "./anonymousUserCookie";

// 現在の署名付き匿名tokenから表示用User IDを取得する。
export async function getCurrentAnonymousUserId(): Promise<string | null> {
  const token = (await cookies()).get(anonymousUserCookie.name)?.value;
  return token?.split(".", 1)[0] ?? null;
}
