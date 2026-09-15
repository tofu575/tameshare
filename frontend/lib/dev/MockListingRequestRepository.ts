import "server-only";

import type { ListingRequestRepository } from "@/lib/listing-request/ListingRequestRepository";
import type { ListingRequestResult } from "@/types/listingRequest";

const requestedUrls = new Map([
  ["https://example.com/already-requested", "pending" as const],
]);

export class MockListingRequestRepository implements ListingRequestRepository {
  async create(sourceUrl: string): Promise<ListingRequestResult> {
    const existingStatus = requestedUrls.get(sourceUrl);
    if (existingStatus) {
      return {
        id: "0193c6f7-28b8-7d27-b210-26d13b74d503",
        status: existingStatus,
        created: false,
      };
    }
    requestedUrls.set(sourceUrl, "pending");
    return {
      id: "0193c6f7-28b8-7d27-b210-26d13b74d504",
      status: "pending",
      created: true,
    };
  }
}
