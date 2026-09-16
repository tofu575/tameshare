import "server-only";

import type { Practice } from "@/types/practice";

export const mockPractices: Practice[] = [
  {
    id: "practice-1",
    title: "朝起きたら水を飲む",
    createdAt: "2026-08-27T01:23:45Z",
    sources: [
      { url: "https://example.com/hydration" },
      { url: "https://example.com/morning-habits" },
    ],
  },
  {
    id: "practice-2",
    title: "振り返りを書く",
    createdAt: "2026-08-27T02:23:45Z",
    sources: [{ url: "https://example.com/reflection" }],
  },
  {
    id: "practice-3",
    title: "寝る前に画面を見ない",
    createdAt: "2026-08-27T03:23:45Z",
    sources: [],
  },
];
