import "server-only";

import { ApiResponseError } from "@/lib/api/ApiResponseError";
import { getAuthenticatedRequestOptions } from "@/lib/auth/getAuthenticatedRequestOptions";
import { getCurrentAnonymousUserId } from "@/lib/auth/getCurrentAnonymousUserId";
import {
  createExperience,
  updateExperience,
} from "@/lib/generated/api/experiences/experiences";
import type { Experience as ExperienceDto } from "@/lib/generated/model";
import { listPracticeDtos } from "@/lib/practice/listPracticeDtos";
import type { Experience } from "@/types/experience";
import type {
  ExperienceRepository,
  SaveExperienceInput,
} from "./ExperienceRepository";
import { listExperienceDtos } from "./listExperienceDtos";

// Generated ClientからUI用Experienceを取得・保存するRepository。
export class ApiExperienceRepository implements ExperienceRepository {
  /** Practiceに紐づくExperienceを全件取得する。 */
  async findByPracticeId(practiceId: string): Promise<Experience[]> {
    const currentUserId = await getCurrentAnonymousUserId();
    return (await listExperienceDtos(practiceId)).map((experience) =>
      this.toExperience(experience, "", currentUserId),
    );
  }

  /** 現在の匿名Userが記録したExperienceを取得する。 */
  async findMineByPracticeId(practiceId: string): Promise<Experience | null> {
    const currentUserId = await getCurrentAnonymousUserId();
    if (!currentUserId) {
      return null;
    }
    const experience = (await listExperienceDtos(practiceId)).find(
      (item) => item.user_id === currentUserId,
    );
    return experience ? this.toExperience(experience, "", currentUserId) : null;
  }

  /** 現在の匿名Userが記録した全PracticeのExperienceを取得する。 */
  async findMine(): Promise<Experience[]> {
    const currentUserId = await getCurrentAnonymousUserId();
    if (!currentUserId) {
      return [];
    }
    const practices = await listPracticeDtos();
    const experienceGroups = await Promise.all(
      practices.map(async (practice) => ({
        practice,
        experiences: await listExperienceDtos(practice.id),
      })),
    );
    return experienceGroups.flatMap(({ practice, experiences }) =>
      experiences
        .filter((experience) => experience.user_id === currentUserId)
        .map((experience) =>
          this.toExperience(experience, practice.title, currentUserId),
        ),
    );
  }

  /** 現在の匿名UserのExperienceを作成または更新する。 */
  async saveMine(input: SaveExperienceInput): Promise<Experience> {
    const existing = await this.findMineByPracticeId(input.practiceId);
    const requestOptions = await getAuthenticatedRequestOptions();
    const response = existing
      ? await updateExperience(
          existing.id,
          { note: input.note },
          requestOptions,
        )
      : await createExperience(
          input.practiceId,
          { note: input.note },
          requestOptions,
        );

    if (response.status !== 200 && response.status !== 201) {
      throw new ApiResponseError(
        response.status,
        response.data.code,
        response.data.message,
      );
    }
    return this.toExperience(
      response.data,
      input.practiceTitle,
      response.data.user_id,
    );
  }

  /** Generated DTOを既存UI Modelへ変換する。 */
  private toExperience(
    experience: ExperienceDto,
    practiceTitle: string,
    currentUserId: string | null,
  ): Experience {
    return {
      id: experience.id,
      practiceId: experience.practice_id,
      practiceTitle,
      userId: experience.user_id,
      note: experience.note,
      isMine: experience.user_id === currentUserId,
      createdAt: experience.created_at,
      updatedAt: experience.updated_at,
    };
  }
}
