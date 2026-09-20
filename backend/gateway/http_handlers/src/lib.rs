mod anonymous_auth;
#[cfg(test)]
mod anonymous_authorization_test;

pub use anonymous_auth::AnonymousAuth;

use actix_web::{HttpRequest, HttpResponse, ResponseError, http::StatusCode, web};
use domain_model::{ExperienceId, PracticeId, UserId};
use domain_usecase::{
    ExperienceOutput, Interactor, ListingRequestOutput, Page, PageInput, PracticeDetail,
    PracticeSummary, UseCaseError,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// offset paginationのquery DTO。
#[derive(Debug, Deserialize)]
struct PageQuery {
    limit: Option<u32>,
    offset: Option<u64>,
}

/// Experienceの作成・更新request DTO。
#[derive(Debug, Deserialize)]
struct ExperienceRequest {
    note: Option<String>,
}

/// 新規匿名セッションの発行結果。
#[derive(Debug, Serialize)]
struct AnonymousSessionResponse {
    user_id: String,
    token: String,
}

/// ListingRequest作成request DTO。
#[derive(Debug, Deserialize)]
struct ListingRequestBody {
    source_url: String,
}

/// HTTP契約専用のPractice summary DTO。
#[derive(Debug, Serialize)]
struct PracticeResponse {
    id: String,
    title: String,
    created_at: String,
}

/// HTTP契約専用のSource DTO。
#[derive(Debug, Serialize)]
struct SourceResponse {
    url: String,
}

/// HTTP契約専用のPractice detail DTO。
#[derive(Debug, Serialize)]
struct PracticeDetailResponse {
    id: String,
    title: String,
    created_at: String,
    sources: Vec<SourceResponse>,
}

/// HTTP契約専用のExperience DTO。
#[derive(Debug, Serialize)]
struct ExperienceResponse {
    id: String,
    practice_id: String,
    user_id: String,
    note: Option<String>,
    created_at: String,
    updated_at: String,
}

/// 一覧responseのpagination metadata。
#[derive(Debug, Serialize)]
struct PageResponse<T> {
    items: Vec<T>,
    limit: u32,
    offset: u64,
    has_more: bool,
}

/// ListingRequest作成結果DTO。
#[derive(Debug, Serialize)]
struct ListingRequestResponse {
    id: String,
    status: &'static str,
    created: bool,
}

/// Clientへ安定したcodeを返すerror DTO。
#[derive(Debug, Serialize)]
struct ErrorResponse {
    code: &'static str,
    message: String,
}

/// Presentation境界で分類したHTTP error。
#[derive(Debug, Error)]
#[error("{message}")]
struct ApiError {
    status: StatusCode,
    code: &'static str,
    message: String,
}

impl ResponseError for ApiError {
    /// error分類に対応するHTTP statusを返す。
    fn status_code(&self) -> StatusCode {
        self.status
    }

    /// Infrastructure詳細を含めないJSON errorを返す。
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status).json(ErrorResponse {
            code: self.code,
            message: self.message.clone(),
        })
    }
}

/// GET /v1/practices: Practiceを新着順でページ取得する。
async fn list_practices(
    interactor: web::Data<Interactor>,
    query: web::Query<PageQuery>,
) -> Result<HttpResponse, ApiError> {
    let page = page_input(&query)?;
    let output = interactor.list_practices(page).await.map_err(api_error)?;
    Ok(HttpResponse::Ok().json(page_practices(output)))
}

/// GET /v1/practices/{id}: Sourceを含むPractice詳細を返す。
async fn get_practice(
    interactor: web::Data<Interactor>,
    path: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let id = PracticeId::try_from(path.into_inner()).map_err(|_| invalid_id())?;
    Ok(HttpResponse::Ok().json(practice_detail(
        interactor.get_practice(id).await.map_err(api_error)?,
    )))
}

/// GET /v1/practices/{id}/experiences: Experienceをページ取得する。
async fn list_experiences(
    interactor: web::Data<Interactor>,
    path: web::Path<String>,
    query: web::Query<PageQuery>,
) -> Result<HttpResponse, ApiError> {
    let id = PracticeId::try_from(path.into_inner()).map_err(|_| invalid_id())?;
    let output = interactor
        .list_experiences(id, page_input(&query)?)
        .await
        .map_err(api_error)?;
    Ok(HttpResponse::Ok().json(page_experiences(output)))
}

/// POST /v1/practices/{id}/experiences: 認証ユーザーのExperienceを作成する。
async fn create_experience(
    request: HttpRequest,
    auth: web::Data<AnonymousAuth>,
    interactor: web::Data<Interactor>,
    path: web::Path<String>,
    body: web::Json<ExperienceRequest>,
) -> Result<HttpResponse, ApiError> {
    let user_id = authenticated_user(&request, &auth)?;
    let practice_id = PracticeId::try_from(path.into_inner()).map_err(|_| invalid_id())?;
    let (output, created) = interactor
        .create_experience(user_id, practice_id, body.into_inner().note)
        .await
        .map_err(api_error)?;
    let status = if created {
        StatusCode::CREATED
    } else {
        StatusCode::OK
    };
    Ok(HttpResponse::build(status).json(experience_response(output)))
}

/// PATCH /v1/experiences/{id}: 投稿者本人のExperienceを更新する。
async fn update_experience(
    request: HttpRequest,
    auth: web::Data<AnonymousAuth>,
    interactor: web::Data<Interactor>,
    path: web::Path<String>,
    body: web::Json<ExperienceRequest>,
) -> Result<HttpResponse, ApiError> {
    let user_id = authenticated_user(&request, &auth)?;
    let id = ExperienceId::try_from(path.into_inner()).map_err(|_| invalid_id())?;
    let output = interactor
        .update_experience(user_id, id, body.into_inner().note)
        .await
        .map_err(api_error)?;
    Ok(HttpResponse::Ok().json(experience_response(output)))
}

/// POST /v1/listing-requests: URL掲載依頼を競合安全に作成する。
async fn create_listing_request(
    request: HttpRequest,
    auth: web::Data<AnonymousAuth>,
    interactor: web::Data<Interactor>,
    body: web::Json<ListingRequestBody>,
) -> Result<HttpResponse, ApiError> {
    let output = interactor
        .create_listing_request(
            authenticated_user(&request, &auth)?,
            body.into_inner().source_url,
        )
        .await
        .map_err(api_error)?;
    let status = if output.created {
        StatusCode::CREATED
    } else {
        StatusCode::OK
    };
    Ok(HttpResponse::build(status).json(listing_request_response(output)))
}

/// POST /v1/anonymous-sessions: 署名済みの匿名Bearer tokenを発行する。
async fn create_anonymous_session(auth: web::Data<AnonymousAuth>) -> HttpResponse {
    let (user_id, token) = auth.issue();
    HttpResponse::Created().json(AnonymousSessionResponse {
        user_id: user_id.to_string(),
        token,
    })
}

/// 公開API routeをまとめて登録する。
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.route(
        "/health",
        web::get().to(|| async { HttpResponse::Ok().json(serde_json::json!({"status":"ok"})) }),
    )
    .service(
        web::scope("/v1")
            .route(
                "/anonymous-sessions",
                web::post().to(create_anonymous_session),
            )
            .route("/practices", web::get().to(list_practices))
            .route("/practices/{id}", web::get().to(get_practice))
            .route(
                "/practices/{id}/experiences",
                web::get().to(list_experiences),
            )
            .route(
                "/practices/{id}/experiences",
                web::post().to(create_experience),
            )
            .route("/experiences/{id}", web::patch().to(update_experience))
            .route("/listing-requests", web::post().to(create_listing_request)),
    );
}

/// query DTOを制約済みUseCase paginationへ変換する。
fn page_input(query: &PageQuery) -> Result<PageInput, ApiError> {
    PageInput::new(
        query.limit.unwrap_or(PageInput::DEFAULT_LIMIT),
        query.offset.unwrap_or(0),
    )
    .map_err(api_error)
}

/// 署名済みBearer tokenから認証済みUserIdを抽出する。
fn authenticated_user(request: &HttpRequest, auth: &AnonymousAuth) -> Result<UserId, ApiError> {
    let value = request
        .headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .ok_or_else(|| ApiError {
            status: StatusCode::UNAUTHORIZED,
            code: "unauthenticated",
            message: "a bearer token is required".into(),
        })?;
    auth.verify(value).ok_or_else(|| ApiError {
        status: StatusCode::UNAUTHORIZED,
        code: "unauthenticated",
        message: "the bearer token is invalid".into(),
    })
}

/// UseCase errorを一般的なHTTP semanticsへ写像する。
fn api_error(error: UseCaseError) -> ApiError {
    match error {
        UseCaseError::Validation(message) => ApiError {
            status: StatusCode::UNPROCESSABLE_ENTITY,
            code: "validation_error",
            message,
        },
        UseCaseError::NotFound => ApiError {
            status: StatusCode::NOT_FOUND,
            code: "not_found",
            message: error.to_string(),
        },
        UseCaseError::Forbidden => ApiError {
            status: StatusCode::FORBIDDEN,
            code: "forbidden",
            message: error.to_string(),
        },
        UseCaseError::Conflict => ApiError {
            status: StatusCode::CONFLICT,
            code: "conflict",
            message: error.to_string(),
        },
        UseCaseError::Infrastructure => {
            tracing::error!("use case infrastructure failure");
            ApiError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                code: "internal_error",
                message: "internal server error".into(),
            }
        }
    }
}

/// path UUIDのparse失敗を400 errorにする。
fn invalid_id() -> ApiError {
    ApiError {
        status: StatusCode::BAD_REQUEST,
        code: "invalid_id",
        message: "ID must be a UUID".into(),
    }
}

/// Practice pageをPresentation DTOへ変換する。
fn page_practices(page: Page<PracticeSummary>) -> PageResponse<PracticeResponse> {
    PageResponse {
        items: page
            .items
            .into_iter()
            .map(|x| PracticeResponse {
                id: x.id.to_string(),
                title: x.title,
                created_at: x.created_at.to_rfc3339(),
            })
            .collect(),
        limit: page.limit,
        offset: page.offset,
        has_more: page.has_more,
    }
}

/// Experience pageをPresentation DTOへ変換する。
fn page_experiences(page: Page<ExperienceOutput>) -> PageResponse<ExperienceResponse> {
    PageResponse {
        items: page.items.into_iter().map(experience_response).collect(),
        limit: page.limit,
        offset: page.offset,
        has_more: page.has_more,
    }
}

/// Practice詳細をPresentation DTOへ変換する。
fn practice_detail(x: PracticeDetail) -> PracticeDetailResponse {
    PracticeDetailResponse {
        id: x.id.to_string(),
        title: x.title,
        created_at: x.created_at.to_rfc3339(),
        sources: x
            .sources
            .into_iter()
            .map(|source| SourceResponse { url: source.url })
            .collect(),
    }
}

/// ExperienceをPresentation DTOへ変換する。
fn experience_response(x: ExperienceOutput) -> ExperienceResponse {
    ExperienceResponse {
        id: x.id.to_string(),
        practice_id: x.practice_id.to_string(),
        user_id: x.user_id.to_string(),
        note: x.note,
        created_at: x.created_at.to_rfc3339(),
        updated_at: x.updated_at.to_rfc3339(),
    }
}

/// ListingRequestをPresentation DTOへ変換する。
fn listing_request_response(x: ListingRequestOutput) -> ListingRequestResponse {
    ListingRequestResponse {
        id: x.id.to_string(),
        status: match x.status {
            domain_model::ListingRequestStatus::Pending => "pending",
            domain_model::ListingRequestStatus::Accepted => "accepted",
            domain_model::ListingRequestStatus::Rejected => "rejected",
        },
        created: x.created,
    }
}

#[cfg(test)]
mod tests {
    use actix_web::{App, http::StatusCode, test, web};
    use async_trait::async_trait;
    use domain_model::{
        Experience, ExperienceId, ListingRequest, ListingRequestId, Practice, PracticeId, Source,
        SourceId, SourceUrl,
    };
    use domain_usecase::gateway::{CommandGateway, QueryGateway, RepositoryError};
    use std::sync::Arc;

    use super::*;

    /// HTTP境界テストでRepositoryへ到達しない異常系を検証するStub。
    struct Stub;

    #[async_trait]
    impl CommandGateway for Stub {
        async fn insert_practice(&self, _: &Practice) -> Result<(), RepositoryError> {
            Ok(())
        }
        async fn insert_source(&self, _: &Source) -> Result<(), RepositoryError> {
            Ok(())
        }
        async fn link_practice_source(
            &self,
            _: PracticeId,
            _: SourceId,
        ) -> Result<(), RepositoryError> {
            Ok(())
        }
        async fn save_experience(
            &self,
            value: &Experience,
        ) -> Result<(Experience, bool), RepositoryError> {
            Ok((value.clone(), true))
        }
        async fn update_experience(&self, _: &Experience) -> Result<(), RepositoryError> {
            Ok(())
        }
        async fn insert_listing_request(&self, _: &ListingRequest) -> Result<(), RepositoryError> {
            Ok(())
        }
        async fn update_listing_request(&self, _: &ListingRequest) -> Result<(), RepositoryError> {
            Ok(())
        }
    }
    #[async_trait]
    impl QueryGateway for Stub {
        async fn list_practices(&self, _: u32, _: u64) -> Result<Vec<Practice>, RepositoryError> {
            Ok(vec![])
        }
        async fn find_practice(&self, _: PracticeId) -> Result<Option<Practice>, RepositoryError> {
            Ok(None)
        }
        async fn find_source(&self, _: SourceId) -> Result<Option<Source>, RepositoryError> {
            Ok(None)
        }
        async fn find_sources_by_practice(
            &self,
            _: PracticeId,
        ) -> Result<Vec<Source>, RepositoryError> {
            Ok(vec![])
        }
        async fn list_experiences_by_practice(
            &self,
            _: PracticeId,
            _: u32,
            _: u64,
        ) -> Result<Vec<Experience>, RepositoryError> {
            Ok(vec![])
        }
        async fn find_experience(
            &self,
            _: ExperienceId,
        ) -> Result<Option<Experience>, RepositoryError> {
            Ok(None)
        }
        async fn find_listing_request(
            &self,
            _: ListingRequestId,
        ) -> Result<Option<ListingRequest>, RepositoryError> {
            Ok(None)
        }
        async fn find_listing_request_by_source_url(
            &self,
            _: &SourceUrl,
        ) -> Result<Option<ListingRequest>, RepositoryError> {
            Ok(None)
        }
    }

    /// Stubを本番と同じinterfaceからInteractorへ注入する。
    fn interactor() -> Interactor {
        let repository = Arc::new(Stub);
        Interactor::new(repository.clone(), repository)
    }

    /// HTTPテスト専用の固定秘密鍵で匿名tokenを発行する。
    fn test_auth() -> AnonymousAuth {
        AnonymousAuth::new("http-handler-test-secret-at-least-32-bytes".into()).unwrap()
    }

    /// malformed UUIDが400になることを確認する。
    #[actix_web::test]
    async fn invalid_practice_id_returns_bad_request() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(interactor()))
                .app_data(web::Data::new(test_auth()))
                .configure(configure_routes),
        )
        .await;
        let response = test::call_service(
            &app,
            test::TestRequest::get()
                .uri("/v1/practices/not-a-uuid")
                .to_request(),
        )
        .await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    /// 認証が必要なAPIでBearerがなければ401になることを確認する。
    #[actix_web::test]
    async fn create_experience_requires_authentication() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(interactor()))
                .app_data(web::Data::new(test_auth()))
                .configure(configure_routes),
        )
        .await;
        let uri = format!("/v1/practices/{}/experiences", PracticeId::generate());
        let response = test::call_service(
            &app,
            test::TestRequest::post()
                .uri(&uri)
                .set_json(serde_json::json!({"note":"よかった"}))
                .to_request(),
        )
        .await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    /// JSON構文不正がframework境界で400になることを確認する。
    #[actix_web::test]
    async fn invalid_json_returns_bad_request() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(interactor()))
                .app_data(web::Data::new(test_auth()))
                .configure(configure_routes),
        )
        .await;
        let uri = format!("/v1/practices/{}/experiences", PracticeId::generate());
        let response = test::call_service(
            &app,
            test::TestRequest::post()
                .uri(&uri)
                .insert_header(("Authorization", format!("Bearer {}", test_auth().issue().1)))
                .insert_header(("Content-Type", "application/json"))
                .set_payload("{")
                .to_request(),
        )
        .await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
