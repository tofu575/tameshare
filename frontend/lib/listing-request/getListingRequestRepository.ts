import "server-only";

import type { ListingRequestRepository } from "./ListingRequestRepository";

export async function getListingRequestRepository(): Promise<ListingRequestRepository> {
  const { MockListingRequestRepository } = await import(
    "@/lib/dev/MockListingRequestRepository"
  );
  return new MockListingRequestRepository();
}
