import "server-only";

import { cookies } from "next/headers";
import { anonymousUserCookie } from "./anonymousUserCookie";

// 現在の匿名User UUIDをCookieから取得する。
export async function getCurrentAnonymousUserId(): Promise<string | null> {
  return (await cookies()).get(anonymousUserCookie.name)?.value ?? null;
}
