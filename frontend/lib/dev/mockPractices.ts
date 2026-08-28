import "server-only";

import type { Practice } from "@/types/practice";

export const mockPractices: Practice[] = [
  {
    id: "practice-1",
    title: "朝のストレッチ",
    description: "一日の始まりに体をほぐす練習です。",
  },
  {
    id: "practice-2",
    title: "振り返りを書く",
    description: "今日試したことと気づきを記録します。",
  },
];
