"use server";

import { getListingRequestRepository } from "@/lib/listing-request/getListingRequestRepository";

export type ListingRequestActionResult = {
  ok: boolean;
  message: string;
};

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

  const repository = await getListingRequestRepository();
  const result = await repository.create(sourceUrl);
  if (!result.created) {
    return { ok: true, message: "このURLはすでにリクエストされています。" };
  }
  return { ok: true, message: "掲載リクエストを受け付けました。" };
}
