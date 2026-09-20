import "server-only";

import { ApiResponseError } from "@/lib/api/ApiResponseError";
import { getAuthenticatedRequestOptions } from "@/lib/auth/getAuthenticatedRequestOptions";
import { createListingRequest } from "@/lib/generated/api/listing-requests/listing-requests";
import type { ListingRequestResult } from "@/types/listingRequest";
import type { ListingRequestRepository } from "./ListingRequestRepository";

// Generated Clientから掲載リクエストを送信するRepository。
export class ApiListingRequestRepository implements ListingRequestRepository {
  /** Source URLの掲載リクエストを作成する。 */
  async create(sourceUrl: string): Promise<ListingRequestResult> {
    const response = await createListingRequest(
      { source_url: sourceUrl },
      await getAuthenticatedRequestOptions(),
    );
    if (response.status !== 200 && response.status !== 201) {
      throw new ApiResponseError(
        response.status,
        response.data.code,
        response.data.message,
      );
    }
    return response.data;
  }
}
