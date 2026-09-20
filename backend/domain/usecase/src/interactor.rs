use std::sync::Arc;

use chrono::{DateTime, Utc};
use domain_model::{
    Experience, ExperienceId, ExperienceNote, ExperienceNoteError, ListingRequest,
    ListingRequestId, ListingRequestStatus, PracticeId, SourceUrl, SourceUrlError, UserId,
};
use thiserror::Error;

use crate::gateway::{CommandGateway, QueryGateway, RepositoryError};

/// APIから独立した一覧ページ指定。
#[derive(Debug, Clone, Copy)]
pub struct PageInput {
    pub limit: u32,
    pub offset: u64,
}

impl PageInput {
    pub const DEFAULT_LIMIT: u32 = 20;
    pub const MAX_LIMIT: u32 = 100;

    /// 上限を超えないページ指定を生成する。
    pub fn new(limit: u32, offset: u64) -> Result<Self, UseCaseError> {
        if limit == 0 || limit > Self::MAX_LIMIT {
            return Err(UseCaseError::Validation(
                "limit must be between 1 and 100".into(),
            ));
        }
        Ok(Self { limit, offset })
    }
}

/// 一覧APIで次ページ有無を表現するUseCase出力。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub limit: u32,
    pub offset: u64,
    pub has_more: bool,
}

/// Practice一覧のUseCase出力。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PracticeSummary {
    pub id: PracticeId,
    pub title: String,
    pub created_at: DateTime<Utc>,
}

/// Practice詳細に含めるSource出力。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceOutput {
    pub url: String,
}

/// Practice詳細のUseCase出力。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PracticeDetail {
    pub id: PracticeId,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub sources: Vec<SourceOutput>,
}

/// ExperienceのUseCase出力。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExperienceOutput {
    pub id: ExperienceId,
    pub practice_id: PracticeId,
    pub user_id: UserId,
    pub note: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// ListingRequest作成結果を新規・既存で区別する。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListingRequestOutput {
    pub id: ListingRequestId,
    pub status: ListingRequestStatus,
    pub created: bool,
}

/// HTTPの詳細を含まないApplication-level error。
#[derive(Debug, Error)]
pub enum UseCaseError {
    #[error("validation failed: {0}")]
    Validation(String),
    #[error("resource not found")]
    NotFound,
    #[error("operation is not allowed")]
    Forbidden,
    #[error("resource conflicts with existing data")]
    Conflict,
    #[error("infrastructure failure")]
    Infrastructure,
}

impl From<ExperienceNoteError> for UseCaseError {
    /// DomainのNote制約違反を入力検証エラーへ変換する。
    fn from(error: ExperienceNoteError) -> Self {
        Self::Validation(error.to_string())
    }
}

impl From<SourceUrlError> for UseCaseError {
    /// DomainのURL制約違反を入力検証エラーへ変換する。
    fn from(error: SourceUrlError) -> Self {
        Self::Validation(error.to_string())
    }
}

/// MVPのユーザー操作をRepository境界上で調整するInteractor。
#[derive(Clone)]
pub struct Interactor {
    command: Arc<dyn CommandGateway>,
    query: Arc<dyn QueryGateway>,
}

impl Interactor {
    /// 状態変更と参照のGatewayをそれぞれ必須依存として注入する。
    pub fn new(command: Arc<dyn CommandGateway>, query: Arc<dyn QueryGateway>) -> Self {
        Self { command, query }
    }

    /// Practiceを新着順でページ取得する。
    pub async fn list_practices(
        &self,
        page: PageInput,
    ) -> Result<Page<PracticeSummary>, UseCaseError> {
        let rows = self
            .query
            .list_practices(page.limit + 1, page.offset)
            .await
            .map_err(map_repository_error)?;
        Ok(make_page(
            rows.into_iter()
                .map(|practice| PracticeSummary {
                    id: practice.id(),
                    title: practice.title().to_owned(),
                    created_at: practice.created_at(),
                })
                .collect(),
            page,
        ))
    }

    /// Practiceと関連Sourceを詳細出力として取得する。
    pub async fn get_practice(&self, id: PracticeId) -> Result<PracticeDetail, UseCaseError> {
        let practice = self
            .query
            .find_practice(id)
            .await
            .map_err(map_repository_error)?
            .ok_or(UseCaseError::NotFound)?;
        let sources = self
            .query
            .find_sources_by_practice(id)
            .await
            .map_err(map_repository_error)?;
        Ok(PracticeDetail {
            id,
            title: practice.title().to_owned(),
            created_at: practice.created_at(),
            sources: sources
                .into_iter()
                .map(|source| SourceOutput {
                    url: source.url().as_str().to_owned(),
                })
                .collect(),
        })
    }

    /// 指定Practiceの存在を確認してExperienceをページ取得する。
    pub async fn list_experiences(
        &self,
        practice_id: PracticeId,
        page: PageInput,
    ) -> Result<Page<ExperienceOutput>, UseCaseError> {
        if self
            .query
            .find_practice(practice_id)
            .await
            .map_err(map_repository_error)?
            .is_none()
        {
            return Err(UseCaseError::NotFound);
        }
        let rows = self
            .query
            .list_experiences_by_practice(practice_id, page.limit + 1, page.offset)
            .await
            .map_err(map_repository_error)?;
        Ok(make_page(
            rows.into_iter().map(experience_output).collect(),
            page,
        ))
    }

    /// 認証済みUserを所有者としてExperienceを作成する。
    pub async fn create_experience(
        &self,
        user_id: UserId,
        practice_id: PracticeId,
        note: Option<String>,
    ) -> Result<(ExperienceOutput, bool), UseCaseError> {
        if self
            .query
            .find_practice(practice_id)
            .await
            .map_err(map_repository_error)?
            .is_none()
        {
            return Err(UseCaseError::NotFound);
        }
        let note = note.map(ExperienceNote::try_from).transpose()?;
        let experience = Experience::new(practice_id, user_id, note);
        let (saved, created) = self
            .command
            .save_experience(&experience)
            .await
            .map_err(map_repository_error)?;
        Ok((experience_output(saved), created))
    }

    /// 投稿者本人であることを確認してExperience Noteを更新する。
    pub async fn update_experience(
        &self,
        user_id: UserId,
        id: ExperienceId,
        note: Option<String>,
    ) -> Result<ExperienceOutput, UseCaseError> {
        let mut experience = self
            .query
            .find_experience(id)
            .await
            .map_err(map_repository_error)?
            .ok_or(UseCaseError::NotFound)?;
        if experience.user_id() != user_id {
            return Err(UseCaseError::Forbidden);
        }
        experience.update_note(note.map(ExperienceNote::try_from).transpose()?);
        self.command
            .update_experience(&experience)
            .await
            .map_err(map_repository_error)?;
        Ok(experience_output(experience))
    }

    /// DB一意制約を競合判定の起点として掲載依頼を一度だけ作成する。
    pub async fn create_listing_request(
        &self,
        user_id: UserId,
        source_url: String,
    ) -> Result<ListingRequestOutput, UseCaseError> {
        let source_url = SourceUrl::try_from(source_url)?;
        let request = ListingRequest::new(source_url.clone(), user_id);
        match self.command.insert_listing_request(&request).await {
            Ok(()) => Ok(listing_request_output(request, true)),
            Err(RepositoryError::DuplicateListingRequest) => {
                let existing = self
                    .query
                    .find_listing_request_by_source_url(&source_url)
                    .await
                    .map_err(map_repository_error)?
                    .ok_or(UseCaseError::Infrastructure)?;
                Ok(listing_request_output(existing, false))
            }
            Err(error) => Err(map_repository_error(error)),
        }
    }
}

/// Repository固有の失敗をApplication errorへ閉じ込める。
fn map_repository_error(error: RepositoryError) -> UseCaseError {
    match error {
        RepositoryError::NotFound => UseCaseError::NotFound,
        RepositoryError::AlreadyExists | RepositoryError::DuplicateListingRequest => {
            UseCaseError::Conflict
        }
        RepositoryError::InvalidStoredData(_) | RepositoryError::Unavailable(_) => {
            UseCaseError::Infrastructure
        }
    }
}

/// 余分に取得した1件から次ページ有無を構築する。
fn make_page<T>(mut items: Vec<T>, input: PageInput) -> Page<T> {
    let has_more = items.len() > input.limit as usize;
    items.truncate(input.limit as usize);
    Page {
        items,
        limit: input.limit,
        offset: input.offset,
        has_more,
    }
}

/// Experience AggregateをUseCase出力へ変換する。
fn experience_output(value: Experience) -> ExperienceOutput {
    ExperienceOutput {
        id: value.id(),
        practice_id: value.practice_id(),
        user_id: value.user_id(),
        note: value.note().map(ToString::to_string),
        created_at: value.created_at(),
        updated_at: value.updated_at(),
    }
}

/// ListingRequest Aggregateを作成結果へ変換する。
fn listing_request_output(value: ListingRequest, created: bool) -> ListingRequestOutput {
    ListingRequestOutput {
        id: value.id(),
        status: value.status(),
        created,
    }
}
