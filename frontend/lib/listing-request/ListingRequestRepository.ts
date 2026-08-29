import type { ListingRequestResult } from "@/types/listingRequest";

export interface ListingRequestRepository {
  create(sourceUrl: string): Promise<ListingRequestResult>;
}
