import { expect, test } from "@playwright/test";

// Practiceの閲覧からExperienceの作成・表示・本人更新までブラウザで検査する。
test("PracticeからExperienceを保存して更新できる", async ({ page }) => {
  const practiceId = "01a0acc8-b771-7d65-b857-c4dc1b6dd92d";
  const practiceTitle = "料理の待ち時間に、使い終わった道具だけ洗う";
  const detailPath = `/practices/${practiceId}`;
  const initialNote = `E2E 作成 ${Date.now()}`;
  const updatedNote = `E2E 更新 ${Date.now()}`;

  await page.goto("/practices");
  await expect(
    page.getByRole("heading", { name: "Practiceを探す" }),
  ).toBeVisible();
  await expect(
    page.getByRole("heading", { name: practiceTitle }),
  ).toBeVisible();
  await page.goto(detailPath);
  await expect(
    page.getByRole("heading", { name: practiceTitle }),
  ).toBeVisible();
  await page.getByRole("link", { name: "試した結果を残す" }).click();

  await page
    .getByRole("textbox", { name: "ひとこと（任意）" })
    .fill(initialNote);
  await page.getByRole("button", { name: "保存する" }).click();
  await expect(page.getByText("Experienceを保存しました。")).toBeVisible();
  await page.goto(detailPath);
  const myExperience = page
    .getByRole("heading", { name: "自分のExperience" })
    .locator("..");
  await expect(myExperience.getByText(initialNote)).toBeVisible();

  await myExperience.getByRole("link", { name: "編集する" }).click();
  await page
    .getByRole("textbox", { name: "ひとこと（任意）" })
    .fill(updatedNote);
  await page.getByRole("button", { name: "更新する" }).click();
  await expect(page.getByText("Experienceを保存しました。")).toBeVisible();
  await page.goto(detailPath);
  await expect(
    page
      .getByRole("heading", { name: "自分のExperience" })
      .locator("..")
      .getByText(updatedNote),
  ).toBeVisible();
  await page.goto("/me/experiences");
  await expect(page.getByText(updatedNote)).toBeVisible();
});
