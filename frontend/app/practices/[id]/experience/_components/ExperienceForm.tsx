"use client";

import { useState, useTransition } from "react";
import type { Experience } from "@/types/experience";
import { saveExperienceAction } from "../actions";

type Props = {
  practiceId: string;
  practiceTitle: string;
  experience: Experience | null;
};

export function ExperienceForm({
  practiceId,
  practiceTitle,
  experience,
}: Props) {
  const [note, setNote] = useState(experience?.note ?? "");
  const [message, setMessage] = useState("");
  const [isError, setIsError] = useState(false);
  const [isPending, startTransition] = useTransition();
  const characterCount = Array.from(note).length;

  function handleSubmit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
    if (characterCount > 100) {
      setIsError(true);
      setMessage("ひとことは100文字以内で入力してください。");
      return;
    }
    if (/https?:\/\/|www\./i.test(note)) {
      setIsError(true);
      setMessage("ひとことにURLは入力できません。");
      return;
    }

    startTransition(async () => {
      const result = await saveExperienceAction(
        practiceId,
        practiceTitle,
        note,
      );
      setIsError(!result.ok);
      setMessage(result.message);
    });
  }

  return (
    <form onSubmit={handleSubmit}>
      <div className="form-field">
        <label htmlFor="experience-note">ひとこと（任意）</label>
        <textarea
          id="experience-note"
          name="note"
          rows={4}
          maxLength={100}
          value={note}
          onChange={(event) => setNote(event.target.value)}
          aria-describedby="note-count note-help"
        />
        <small id="note-help">URLは入力できません。</small>
        <output id="note-count">{characterCount} / 100</output>
      </div>
      <button type="submit" disabled={isPending}>
        {isPending ? "保存中..." : experience ? "更新する" : "保存する"}
      </button>
      {message && (
        <p className={isError ? "form-error" : "form-message"}>{message}</p>
      )}
    </form>
  );
}
