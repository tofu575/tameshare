use diesel::sql_types::Text;
use diesel::{Connection, PgConnection, QueryableByName, RunQueryDsl};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use domain_model::{
    Experience, ExperienceNote, ListingRequest, Practice, Source, SourceUrl, UserId,
};
use domain_usecase::gateway::{
    ExperienceRepository, ListingRequestRepository, PracticeRepository, PracticeSourceRepository,
    RepositoryError, SourceRepository,
};
use postgres_gateway::PostgresRepository;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../../migrations");

/// 接続先が明示的なtest DBか検証するためのprivate DTO。
#[derive(QueryableByName)]
struct CurrentDatabase {
    #[diesel(sql_type = Text)]
    current_database: String,
}

/// fallbackせず、Repository Integration Test専用URLを必須で取得する。
fn required_test_database_url() -> String {
    std::env::var("TEST_DATABASE_URL")
        .expect("TEST_DATABASE_URL must be set for PostgreSQL integration tests")
}

/// test専用と明示されたDBだけでmigrationの適用・rollback・再適用を行う。
fn reset_test_database_migrations(database_url: &str) {
    let mut connection = PgConnection::establish(database_url)
        .expect("integration test PostgreSQL must be reachable");
    let database = diesel::sql_query("SELECT current_database() AS current_database")
        .get_result::<CurrentDatabase>(&mut connection)
        .expect("current database name must be readable");
    assert!(
        database.current_database.ends_with("_test"),
        "refusing to reset migrations outside a *_test database"
    );
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("all migrations must apply");
    connection
        .revert_all_migrations(MIGRATIONS)
        .expect("all migrations must roll back");
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("all migrations must apply after rollback");
}

#[tokio::test]
/// migration往復、Repository保存取得、N:M、掲載依頼重複制約を実DBで確認する。
async fn repositories_round_trip_relations_and_enforce_listing_request_uniqueness() {
    let database_url = required_test_database_url();
    reset_test_database_migrations(&database_url);
    let repository = PostgresRepository::connect(&database_url).await.unwrap();

    let first_practice = Practice::new("25分だけ集中する");
    let second_practice = Practice::new("作業前に机を片付ける");
    PracticeRepository::insert(&repository, &first_practice)
        .await
        .unwrap();
    PracticeRepository::insert(&repository, &second_practice)
        .await
        .unwrap();
    assert_eq!(
        PracticeRepository::find(&repository, first_practice.id())
            .await
            .unwrap(),
        Some(first_practice.clone())
    );

    let first_source = Source::new(SourceUrl::try_from("https://example.com/focus").unwrap());
    let second_source = Source::new(SourceUrl::try_from("https://example.com/cleanup").unwrap());
    SourceRepository::insert(&repository, &first_source)
        .await
        .unwrap();
    SourceRepository::insert(&repository, &second_source)
        .await
        .unwrap();
    PracticeSourceRepository::link(&repository, first_practice.id(), first_source.id())
        .await
        .unwrap();
    PracticeSourceRepository::link(&repository, first_practice.id(), second_source.id())
        .await
        .unwrap();
    PracticeSourceRepository::link(&repository, second_practice.id(), first_source.id())
        .await
        .unwrap();
    // 複合主キーによるlinkの冪等性も同時に確認する。
    PracticeSourceRepository::link(&repository, first_practice.id(), first_source.id())
        .await
        .unwrap();
    let first_practice_sources =
        PracticeSourceRepository::find_sources(&repository, first_practice.id())
            .await
            .unwrap();
    let second_practice_sources =
        PracticeSourceRepository::find_sources(&repository, second_practice.id())
            .await
            .unwrap();
    assert_eq!(first_practice_sources.len(), 2);
    assert!(
        first_practice_sources
            .iter()
            .any(|source| source.id() == first_source.id())
    );
    assert!(
        first_practice_sources
            .iter()
            .any(|source| source.id() == second_source.id())
    );
    assert_eq!(second_practice_sources, vec![first_source]);

    let mut experience = Experience::new(
        first_practice.id(),
        UserId::generate(),
        Some(ExperienceNote::try_from("すぐに集中できた").unwrap()),
    );
    ExperienceRepository::insert(&repository, &experience)
        .await
        .unwrap();
    assert_eq!(
        ExperienceRepository::find(&repository, experience.id())
            .await
            .unwrap(),
        Some(experience.clone())
    );
    experience.update_note(None);
    ExperienceRepository::update(&repository, &experience)
        .await
        .unwrap();
    assert_eq!(
        ExperienceRepository::find(&repository, experience.id())
            .await
            .unwrap()
            .unwrap()
            .note(),
        None
    );

    let listing_url = SourceUrl::try_from("https://example.com/request-me").unwrap();
    let mut first = ListingRequest::new(listing_url.clone(), UserId::generate());
    first.reject();
    ListingRequestRepository::insert(&repository, &first)
        .await
        .unwrap();
    let duplicate = ListingRequest::new(listing_url.clone(), UserId::generate());
    assert_eq!(
        ListingRequestRepository::insert(&repository, &duplicate).await,
        Err(RepositoryError::DuplicateListingRequest)
    );
    assert_eq!(
        ListingRequestRepository::find_by_source_url(&repository, &listing_url)
            .await
            .unwrap(),
        Some(first.clone())
    );

    let same_id_with_different_url = ListingRequest::restore(
        first.id(),
        SourceUrl::try_from("https://example.com/different-url").unwrap(),
        duplicate.user_id(),
        duplicate.status(),
        duplicate.created_at(),
        duplicate.updated_at(),
    );
    assert_eq!(
        ListingRequestRepository::insert(&repository, &same_id_with_different_url).await,
        Err(RepositoryError::AlreadyExists)
    );
}
