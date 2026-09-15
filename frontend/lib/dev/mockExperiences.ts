import "server-only";

import type { Experience } from "@/types/experience";

export const mockExperiences: Experience[] = [
  {
    id: "experience-1",
    practiceId: "practice-1",
    practiceTitle: "朝起きたら水を飲む",
    note: "思ったより気持ちよく一日を始められました。",
    userId: "current-user",
    isMine: true,
    createdAt: "2026-08-27T01:23:45Z",
    updatedAt: "2026-08-27T01:23:45Z",
  },
  {
    id: "experience-2",
    practiceId: "practice-1",
    practiceTitle: "朝起きたら水を飲む",
    note: "変化はまだ分かりませんが、続けやすいです。",
    userId: "another-user",
    isMine: false,
    createdAt: "2026-08-28T01:23:45Z",
    updatedAt: "2026-08-28T01:23:45Z",
  },
  {
    id: "experience-3",
    practiceId: "practice-2",
    practiceTitle: "振り返りを書く",
    note: "翌日の予定を考えやすくなりました。",
    userId: "another-user",
    isMine: false,
    createdAt: "2026-08-28T02:23:45Z",
    updatedAt: "2026-08-28T02:23:45Z",
  },
];
