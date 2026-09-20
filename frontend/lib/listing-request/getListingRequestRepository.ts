import "server-only";

import type { ListingRequestRepository } from "./ListingRequestRepository";

// Generated Clientを利用するListingRequest Repositoryを返す。
export async function getListingRequestRepository(): Promise<ListingRequestRepository> {
  const { ApiListingRequestRepository } = await import(
    "./ApiListingRequestRepository"
  );
  return new ApiListingRequestRepository();
}
