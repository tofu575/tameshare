"use server";

import { revalidatePath } from "next/cache";
import { getExperienceRepository } from "@/lib/experience/getExperienceRepository";

export type ExperienceActionResult = {
  ok: boolean;
  message: string;
};

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

  const repository = await getExperienceRepository();
  await repository.saveMine({
    practiceId,
    practiceTitle,
    note: note.length > 0 ? note : null,
  });
  revalidatePath(`/practices/${practiceId}`);
  revalidatePath("/me/experiences");
  return { ok: true, message: "Experienceを保存しました。" };
}
