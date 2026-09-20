use std::sync::{Arc, Mutex};

use actix_web::{App, http::StatusCode, test, web};
use async_trait::async_trait;
use domain_model::{
    Experience, ExperienceId, ListingRequest, ListingRequestId, Practice, PracticeId, Source,
    SourceId, SourceUrl,
};
use domain_usecase::{
    Interactor,
    gateway::{CommandGateway, QueryGateway, RepositoryError},
};

use crate::{AnonymousAuth, configure_routes};

/// 公開APIテストで保存内容を検査できるGateway。
struct FixtureGateway {
    practice: Practice,
    experience: Mutex<Option<Experience>>,
}

#[async_trait]
impl CommandGateway for FixtureGateway {
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
        let mut stored = self.experience.lock().unwrap();
        if let Some(existing) = stored.as_mut() {
            existing.update_note(value.note().cloned());
            Ok((existing.clone(), false))
        } else {
            *stored = Some(value.clone());
            Ok((value.clone(), true))
        }
    }
    async fn update_experience(&self, value: &Experience) -> Result<(), RepositoryError> {
        *self.experience.lock().unwrap() = Some(value.clone());
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
impl QueryGateway for FixtureGateway {
    async fn list_practices(&self, _: u32, _: u64) -> Result<Vec<Practice>, RepositoryError> {
        Ok(vec![self.practice.clone()])
    }
    async fn find_practice(&self, id: PracticeId) -> Result<Option<Practice>, RepositoryError> {
        Ok((id == self.practice.id()).then(|| self.practice.clone()))
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
        Ok(self
            .experience
            .lock()
            .unwrap()
            .clone()
            .into_iter()
            .collect())
    }
    async fn find_experience(
        &self,
        id: ExperienceId,
    ) -> Result<Option<Experience>, RepositoryError> {
        Ok(self
            .experience
            .lock()
            .unwrap()
            .clone()
            .filter(|value| value.id() == id))
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

/// 公開APIでは本人だけが更新でき、拒否された要求は保存内容を変えない。
#[actix_web::test]
async fn public_api_rejects_impersonation_without_mutating_storage() {
    let gateway = Arc::new(FixtureGateway {
        practice: Practice::new("権限テスト"),
        experience: Mutex::new(None),
    });
    let auth = AnonymousAuth::new("public-api-test-secret-at-least-32-bytes".into()).unwrap();
    let interactor = Interactor::new(gateway.clone(), gateway.clone());
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(interactor))
            .app_data(web::Data::new(auth))
            .configure(configure_routes),
    )
    .await;

    let owner_session = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/v1/anonymous-sessions")
            .to_request(),
    )
    .await;
    assert_eq!(owner_session.status(), StatusCode::CREATED);
    let owner: serde_json::Value = test::read_body_json(owner_session).await;
    let other_session = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/v1/anonymous-sessions")
            .to_request(),
    )
    .await;
    let other: serde_json::Value = test::read_body_json(other_session).await;
    let owner_token = owner["token"].as_str().unwrap();
    let other_token = other["token"].as_str().unwrap();
    let create_uri = format!("/v1/practices/{}/experiences", gateway.practice.id());
    let created = test::call_service(
        &app,
        test::TestRequest::post()
            .uri(&create_uri)
            .insert_header(("Authorization", format!("Bearer {owner_token}")))
            .set_json(serde_json::json!({"note":"original"}))
            .to_request(),
    )
    .await;
    assert_eq!(created.status(), StatusCode::CREATED);
    let created: serde_json::Value = test::read_body_json(created).await;
    let update_uri = format!("/v1/experiences/{}", created["id"].as_str().unwrap());
    let before = gateway.experience.lock().unwrap().clone();

    let public =
        test::call_service(&app, test::TestRequest::get().uri(&create_uri).to_request()).await;
    let public: serde_json::Value = test::read_body_json(public).await;
    assert_eq!(public["items"][0]["user_id"], owner["user_id"]);
    let signature = other_token.split_once('.').unwrap().1;
    let forged = format!(
        "{}.{}",
        public["items"][0]["user_id"].as_str().unwrap(),
        signature
    );
    for (token, expected) in [
        (Some(other_token), StatusCode::FORBIDDEN),
        (None, StatusCode::UNAUTHORIZED),
        (Some("invalid"), StatusCode::UNAUTHORIZED),
        (
            Some(public["items"][0]["user_id"].as_str().unwrap()),
            StatusCode::UNAUTHORIZED,
        ),
        (Some(forged.as_str()), StatusCode::UNAUTHORIZED),
    ] {
        let mut request = test::TestRequest::patch()
            .uri(&update_uri)
            .set_json(serde_json::json!({"note":"attacker"}));
        if let Some(token) = token {
            request = request.insert_header(("Authorization", format!("Bearer {token}")));
        }
        let response = test::call_service(&app, request.to_request()).await;
        assert_eq!(response.status(), expected);
        assert_eq!(*gateway.experience.lock().unwrap(), before);
    }

    let updated = test::call_service(
        &app,
        test::TestRequest::patch()
            .uri(&update_uri)
            .insert_header(("Authorization", format!("Bearer {owner_token}")))
            .set_json(serde_json::json!({"note":"owner update"}))
            .to_request(),
    )
    .await;
    assert_eq!(updated.status(), StatusCode::OK);
    assert_eq!(
        gateway
            .experience
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .note()
            .unwrap()
            .as_str(),
        "owner update"
    );
    let repeated = test::call_service(
        &app,
        test::TestRequest::post()
            .uri(&create_uri)
            .insert_header(("Authorization", format!("Bearer {owner_token}")))
            .set_json(serde_json::json!({"note":"second save"}))
            .to_request(),
    )
    .await;
    assert_eq!(repeated.status(), StatusCode::OK);
    let repeated: serde_json::Value = test::read_body_json(repeated).await;
    assert_eq!(repeated["id"], created["id"]);
    assert_eq!(repeated["note"], "second save");
}
