import "server-only";

import { cookies } from "next/headers";
import { ApiResponseError } from "@/lib/api/ApiResponseError";
import { createAnonymousSession } from "@/lib/generated/api/experiences/experiences";
import { anonymousUserCookie } from "./anonymousUserCookie";

// Backendから署名付き匿名tokenを取得し、HTTP-only Cookieに保存する。
export async function getOrCreateAnonymousToken(): Promise<string> {
  const cookieStore = await cookies();
  const existing = cookieStore.get(anonymousUserCookie.name)?.value;
  if (existing) {
    return existing;
  }

  const response = await createAnonymousSession();
  if (response.status !== 201) {
    throw new ApiResponseError(
      response.status,
      response.data.code,
      response.data.message,
    );
  }
  cookieStore.set(
    anonymousUserCookie.name,
    response.data.token,
    anonymousUserCookie.options,
  );
  return response.data.token;
}
