import "server-only";

import { randomUUID } from "node:crypto";
import { cookies } from "next/headers";
import { anonymousUserCookie } from "./anonymousUserCookie";

// 匿名User UUIDを取得し、未作成ならHTTP-only Cookieへ保存する。
export async function getOrCreateAnonymousUserId(): Promise<string> {
  const cookieStore = await cookies();
  const existing = cookieStore.get(anonymousUserCookie.name)?.value;
  if (existing) {
    return existing;
  }

  const userId = randomUUID();
  cookieStore.set(
    anonymousUserCookie.name,
    userId,
    anonymousUserCookie.options,
  );
  return userId;
}
