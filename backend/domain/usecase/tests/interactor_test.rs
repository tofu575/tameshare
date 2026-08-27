use async_trait::async_trait;
use domain_model::{
    Experience, ExperienceId, ListingRequest, ListingRequestId, Practice, PracticeId, Source,
    SourceUrl, UserId,
};
use domain_usecase::{
    Interactor, PageInput, UseCaseError,
    gateway::{
        ExperienceRepository, ListingRequestRepository, PracticeRepository,
        PracticeSourceRepository, RepositoryError,
    },
};
use std::sync::{Arc, Mutex};

/// UseCase単体テスト用の本番Repository interface実装。
#[derive(Default)]
struct FakeRepository {
    practices: Mutex<Vec<Practice>>,
    experiences: Mutex<Vec<Experience>>,
    requests: Mutex<Vec<ListingRequest>>,
}

#[async_trait]
impl PracticeRepository for FakeRepository {
    async fn list(&self, limit: u32, offset: u64) -> Result<Vec<Practice>, RepositoryError> {
        Ok(self
            .practices
            .lock()
            .unwrap()
            .iter()
            .skip(offset as usize)
            .take(limit as usize)
            .cloned()
            .collect())
    }
    async fn insert(&self, value: &Practice) -> Result<(), RepositoryError> {
        self.practices.lock().unwrap().push(value.clone());
        Ok(())
    }
    async fn find(&self, id: PracticeId) -> Result<Option<Practice>, RepositoryError> {
        Ok(self
            .practices
            .lock()
            .unwrap()
            .iter()
            .find(|x| x.id() == id)
            .cloned())
    }
}

#[async_trait]
impl PracticeSourceRepository for FakeRepository {
    async fn link(&self, _: PracticeId, _: domain_model::SourceId) -> Result<(), RepositoryError> {
        Ok(())
    }
    async fn find_sources(&self, _: PracticeId) -> Result<Vec<Source>, RepositoryError> {
        Ok(vec![Source::new(
            SourceUrl::try_from("https://example.com").unwrap(),
        )])
    }
}

#[async_trait]
impl ExperienceRepository for FakeRepository {
    async fn list_by_practice(
        &self,
        id: PracticeId,
        limit: u32,
        offset: u64,
    ) -> Result<Vec<Experience>, RepositoryError> {
        Ok(self
            .experiences
            .lock()
            .unwrap()
            .iter()
            .filter(|x| x.practice_id() == id)
            .skip(offset as usize)
            .take(limit as usize)
            .cloned()
            .collect())
    }
    async fn insert(&self, value: &Experience) -> Result<(), RepositoryError> {
        self.experiences.lock().unwrap().push(value.clone());
        Ok(())
    }
    async fn find(&self, id: ExperienceId) -> Result<Option<Experience>, RepositoryError> {
        Ok(self
            .experiences
            .lock()
            .unwrap()
            .iter()
            .find(|x| x.id() == id)
            .cloned())
    }
    async fn update(&self, value: &Experience) -> Result<(), RepositoryError> {
        let mut values = self.experiences.lock().unwrap();
        let item = values
            .iter_mut()
            .find(|x| x.id() == value.id())
            .ok_or(RepositoryError::NotFound)?;
        *item = value.clone();
        Ok(())
    }
}

#[async_trait]
impl ListingRequestRepository for FakeRepository {
    async fn insert(&self, value: &ListingRequest) -> Result<(), RepositoryError> {
        let mut values = self.requests.lock().unwrap();
        if values.iter().any(|x| x.source_url() == value.source_url()) {
            return Err(RepositoryError::DuplicateListingRequest);
        }
        values.push(value.clone());
        Ok(())
    }
    async fn find(&self, id: ListingRequestId) -> Result<Option<ListingRequest>, RepositoryError> {
        Ok(self
            .requests
            .lock()
            .unwrap()
            .iter()
            .find(|x| x.id() == id)
            .cloned())
    }
    async fn find_by_source_url(
        &self,
        url: &SourceUrl,
    ) -> Result<Option<ListingRequest>, RepositoryError> {
        Ok(self
            .requests
            .lock()
            .unwrap()
            .iter()
            .find(|x| x.source_url() == url)
            .cloned())
    }
    async fn update(&self, _: &ListingRequest) -> Result<(), RepositoryError> {
        Ok(())
    }
}

/// 同一Fakeを全てのRepository abstractionとして注入する。
fn fixture() -> (Interactor, Arc<FakeRepository>, Practice) {
    let repository = Arc::new(FakeRepository::default());
    let practice = Practice::new("朝に散歩する");
    repository.practices.lock().unwrap().push(practice.clone());
    (
        Interactor::new(
            repository.clone(),
            repository.clone(),
            repository.clone(),
            repository.clone(),
        ),
        repository,
        practice,
    )
}

/// Practice一覧とSource付き詳細、およびNot Foundを確認する。
#[tokio::test]
async fn reads_practices_and_detail() {
    let (usecase, _, practice) = fixture();
    assert_eq!(
        usecase
            .list_practices(PageInput::new(20, 0).unwrap())
            .await
            .unwrap()
            .items
            .len(),
        1
    );
    assert_eq!(
        usecase
            .get_practice(practice.id())
            .await
            .unwrap()
            .sources
            .len(),
        1
    );
    assert!(matches!(
        usecase.get_practice(PracticeId::generate()).await,
        Err(UseCaseError::NotFound)
    ));
}

/// Experience作成、Domain validation、本人更新、他人拒否を確認する。
#[tokio::test]
async fn creates_validates_and_authorizes_experience() {
    let (usecase, _, practice) = fixture();
    let owner = UserId::generate();
    let created = usecase
        .create_experience(owner, practice.id(), Some("続けやすい".into()))
        .await
        .unwrap();
    assert!(matches!(
        usecase
            .create_experience(owner, practice.id(), Some("あ".repeat(101)))
            .await,
        Err(UseCaseError::Validation(_))
    ));
    assert!(
        usecase
            .update_experience(owner, created.id, Some("おいしかった".into()))
            .await
            .is_ok()
    );
    assert!(matches!(
        usecase
            .update_experience(UserId::generate(), created.id, None)
            .await,
        Err(UseCaseError::Forbidden)
    ));
}

/// 同一URLのListingRequestが既存結果として返ることを確認する。
#[tokio::test]
async fn listing_request_is_idempotent() {
    let (usecase, _, _) = fixture();
    let first = usecase
        .create_listing_request(UserId::generate(), "https://example.com/new".into())
        .await
        .unwrap();
    let duplicate = usecase
        .create_listing_request(UserId::generate(), "https://example.com/new".into())
        .await
        .unwrap();
    assert!(first.created);
    assert!(!duplicate.created);
    assert_eq!(first.id, duplicate.id);
}
