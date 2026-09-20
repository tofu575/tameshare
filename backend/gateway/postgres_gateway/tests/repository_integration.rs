use diesel::sql_types::Text;
use diesel::{Connection, PgConnection, QueryableByName, RunQueryDsl};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use domain_model::{
    Experience, ExperienceNote, ListingRequest, Practice, Source, SourceUrl, UserId,
};
use domain_usecase::gateway::{CommandGateway, QueryGateway, RepositoryError};
use postgres_gateway::PostgresqlGateway;

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
    let gateway = PostgresqlGateway::connect(&database_url).await.unwrap();

    let first_practice = Practice::new("25分だけ集中する");
    let second_practice = Practice::new("作業前に机を片付ける");
    CommandGateway::insert_practice(&gateway, &first_practice)
        .await
        .unwrap();
    CommandGateway::insert_practice(&gateway, &second_practice)
        .await
        .unwrap();
    assert_eq!(
        QueryGateway::find_practice(&gateway, first_practice.id())
            .await
            .unwrap(),
        Some(first_practice.clone())
    );

    let first_source = Source::new(SourceUrl::try_from("https://example.com/focus").unwrap());
    let second_source = Source::new(SourceUrl::try_from("https://example.com/cleanup").unwrap());
    CommandGateway::insert_source(&gateway, &first_source)
        .await
        .unwrap();
    CommandGateway::insert_source(&gateway, &second_source)
        .await
        .unwrap();
    CommandGateway::link_practice_source(&gateway, first_practice.id(), first_source.id())
        .await
        .unwrap();
    CommandGateway::link_practice_source(&gateway, first_practice.id(), second_source.id())
        .await
        .unwrap();
    CommandGateway::link_practice_source(&gateway, second_practice.id(), first_source.id())
        .await
        .unwrap();
    // 複合主キーによるlinkの冪等性も同時に確認する。
    CommandGateway::link_practice_source(&gateway, first_practice.id(), first_source.id())
        .await
        .unwrap();
    let first_practice_sources =
        QueryGateway::find_sources_by_practice(&gateway, first_practice.id())
            .await
            .unwrap();
    let second_practice_sources =
        QueryGateway::find_sources_by_practice(&gateway, second_practice.id())
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
    let (saved, created) = CommandGateway::save_experience(&gateway, &experience)
        .await
        .unwrap();
    assert!(created);
    assert_eq!(saved, experience);
    assert_eq!(
        QueryGateway::find_experience(&gateway, experience.id())
            .await
            .unwrap(),
        Some(experience.clone())
    );
    experience.update_note(None);
    CommandGateway::update_experience(&gateway, &experience)
        .await
        .unwrap();
    assert_eq!(
        QueryGateway::find_experience(&gateway, experience.id())
            .await
            .unwrap()
            .unwrap()
            .note(),
        None
    );

    let repeated = Experience::new(
        first_practice.id(),
        experience.user_id(),
        Some(ExperienceNote::try_from("再投稿").unwrap()),
    );
    let (saved, created) = CommandGateway::save_experience(&gateway, &repeated)
        .await
        .unwrap();
    assert!(!created);
    assert_eq!(saved.id(), experience.id());
    assert_eq!(saved.note().unwrap().as_str(), "再投稿");
    let concurrent_user = UserId::generate();
    let first = Experience::new(
        first_practice.id(),
        concurrent_user,
        Some(ExperienceNote::try_from("並行A").unwrap()),
    );
    let second = Experience::new(
        first_practice.id(),
        concurrent_user,
        Some(ExperienceNote::try_from("並行B").unwrap()),
    );
    let (first_result, second_result) = tokio::join!(
        CommandGateway::save_experience(&gateway, &first),
        CommandGateway::save_experience(&gateway, &second),
    );
    let (first_saved, first_created) = first_result.unwrap();
    let (second_saved, second_created) = second_result.unwrap();
    assert_ne!(first_created, second_created);
    assert_eq!(first_saved.id(), second_saved.id());
    let matches: Vec<_> =
        QueryGateway::list_experiences_by_practice(&gateway, first_practice.id(), 100, 0)
            .await
            .unwrap()
            .into_iter()
            .filter(|value| value.user_id() == concurrent_user)
            .collect();
    assert_eq!(matches.len(), 1);

    let listing_url = SourceUrl::try_from("https://example.com/request-me").unwrap();
    let mut first = ListingRequest::new(listing_url.clone(), UserId::generate());
    first.reject();
    CommandGateway::insert_listing_request(&gateway, &first)
        .await
        .unwrap();
    let duplicate = ListingRequest::new(listing_url.clone(), UserId::generate());
    assert_eq!(
        CommandGateway::insert_listing_request(&gateway, &duplicate).await,
        Err(RepositoryError::DuplicateListingRequest)
    );
    assert_eq!(
        QueryGateway::find_listing_request_by_source_url(&gateway, &listing_url)
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
        CommandGateway::insert_listing_request(&gateway, &same_id_with_different_url).await,
        Err(RepositoryError::AlreadyExists)
    );
}
