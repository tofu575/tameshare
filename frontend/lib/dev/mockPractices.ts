import "server-only";

import type { Practice } from "@/types/practice";

export const mockPractices: Practice[] = [
  {
    id: "practice-1",
    title: "朝起きたら水を飲む",
    description: "起床後にコップ一杯の水を飲み、一日を始める方法です。",
    sources: [
      { url: "https://example.com/hydration" },
      { url: "https://example.com/morning-habits" },
    ],
  },
  {
    id: "practice-2",
    title: "振り返りを書く",
    description: "今日試したことと気づきを記録します。",
    sources: [{ url: "https://example.com/reflection" }],
  },
  {
    id: "practice-3",
    title: "寝る前に画面を見ない",
    description: "就寝前の30分はスマートフォンやPCから離れる方法です。",
    sources: [],
  },
];
