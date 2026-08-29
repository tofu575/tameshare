"use server";

import { ApiResponseError } from "@/lib/api/ApiResponseError";
import { getListingRequestRepository } from "@/lib/listing-request/getListingRequestRepository";

export type ListingRequestActionResult = {
  ok: boolean;
  message: string;
};

// URLを検証してBackendへ掲載リクエストを送信する。
export async function createListingRequestAction(
  sourceUrl: string,
): Promise<ListingRequestActionResult> {
  let parsedUrl: URL;
  try {
    parsedUrl = new URL(sourceUrl);
  } catch {
    return { ok: false, message: "有効なURLを入力してください。" };
  }
  if (
    !["http:", "https:"].includes(parsedUrl.protocol) ||
    !parsedUrl.hostname
  ) {
    return {
      ok: false,
      message: "httpまたはhttpsの絶対URLを入力してください。",
    };
  }

  try {
    const repository = await getListingRequestRepository();
    const result = await repository.create(sourceUrl);
    if (result.created) {
      return { ok: true, message: "掲載リクエストを受け付けました。" };
    }
    if (result.status === "pending") {
      return { ok: true, message: "このURLはすでにリクエストされています。" };
    }
    if (result.status === "accepted") {
      return { ok: true, message: "このURLはすでに掲載処理済みです。" };
    }
    return { ok: true, message: "このURLは掲載対象外として処理済みです。" };
  } catch (error) {
    if (error instanceof ApiResponseError) {
      if (error.status === 422) {
        return { ok: false, message: "有効なURLを入力してください。" };
      }
      if (error.status === 401) {
        return {
          ok: false,
          message: "認証を確認できませんでした。もう一度お試しください。",
        };
      }
    }
    return {
      ok: false,
      message: "送信できませんでした。時間をおいてお試しください。",
    };
  }
}
