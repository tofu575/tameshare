use diesel_async::{
    AsyncPgConnection,
    pooled_connection::{AsyncDieselConnectionManager, bb8},
};
use domain_usecase::gateway::RepositoryError;

/// PostgreSQL非同期connectionを共有するpool型。
pub type PgPool = bb8::Pool<AsyncPgConnection>;

/// Command/Query両PortをDieselで実装するPostgreSQL Gateway。
#[derive(Clone)]
pub struct PostgresqlGateway {
    pool: PgPool,
}

impl PostgresqlGateway {
    /// 必須のDatabase URLからconnection poolを構築する。
    pub async fn connect(database_url: &str) -> Result<Self, RepositoryError> {
        let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
        let pool = bb8::Pool::builder()
            .build(manager)
            .await
            .map_err(|error| RepositoryError::Unavailable(error.to_string()))?;
        Ok(Self { pool })
    }

    /// 構築済みconnection poolからGatewayを作成する。
    pub fn from_pool(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Gateway操作に使用するconnectionをpoolから取得する。
    pub(crate) async fn connection(
        &self,
    ) -> Result<bb8::PooledConnection<'_, AsyncPgConnection>, RepositoryError> {
        self.pool
            .get()
            .await
            .map_err(|error| RepositoryError::Unavailable(error.to_string()))
    }
}
