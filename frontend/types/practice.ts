export type Source = {
  url: string;
};

export type Practice = {
  id: string;
  title: string;
  description: string;
  createdAt?: string;
  sources?: Source[];
};
