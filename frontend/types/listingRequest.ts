export type ListingRequestStatus = "pending" | "accepted" | "rejected";

export type ListingRequestResult = {
  id: string;
  status: ListingRequestStatus;
  created: boolean;
};
