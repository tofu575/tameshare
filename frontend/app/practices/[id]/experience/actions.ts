"use server";

import { revalidatePath } from "next/cache";
import { ApiResponseError } from "@/lib/api/ApiResponseError";
import { getExperienceRepository } from "@/lib/experience/getExperienceRepository";

export type ExperienceActionResult = {
  ok: boolean;
  message: string;
};

// Experience入力を検証してBackendへ作成または更新する。
export async function saveExperienceAction(
  practiceId: string,
  practiceTitle: string,
  note: string,
): Promise<ExperienceActionResult> {
  const characterCount = Array.from(note).length;
  if (characterCount > 100) {
    return { ok: false, message: "ひとことは100文字以内で入力してください。" };
  }
  if (
    ["http://", "https://", "www."].some((value) =>
      note.toLowerCase().includes(value),
    )
  ) {
    return { ok: false, message: "ひとことにURLは入力できません。" };
  }

  try {
    const repository = await getExperienceRepository();
    await repository.saveMine({
      practiceId,
      practiceTitle,
      note: note.length > 0 ? note : null,
    });
    revalidatePath(`/practices/${practiceId}`);
    revalidatePath("/me/experiences");
    return { ok: true, message: "Experienceを保存しました。" };
  } catch (error) {
    if (error instanceof ApiResponseError) {
      if (error.status === 422) {
        return {
          ok: false,
          message: "入力内容を確認してください。",
        };
      }
      if (error.status === 401 || error.status === 403) {
        return {
          ok: false,
          message: "認証を確認できませんでした。もう一度お試しください。",
        };
      }
      if (error.status === 404) {
        return { ok: false, message: "Practiceが見つかりません。" };
      }
    }
    return {
      ok: false,
      message: "保存できませんでした。時間をおいてお試しください。",
    };
  }
}
