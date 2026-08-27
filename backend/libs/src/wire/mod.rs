use std::sync::Arc;

use domain_usecase::{Interactor, gateway::RepositoryError};
use postgres_gateway::PostgresRepository;

/// 必須DATABASE_URLから本番用Interactorを構築する。
pub async fn build_interactor() -> Result<Interactor, RepositoryError> {
    let database_url = std::env::var("DATABASE_URL")
        .map_err(|error| RepositoryError::Unavailable(error.to_string()))?;
    let repository = Arc::new(PostgresRepository::connect(&database_url).await?);
    Ok(Interactor::new(
        repository.clone(),
        repository.clone(),
        repository.clone(),
        repository,
    ))
}
