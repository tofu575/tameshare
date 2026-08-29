use async_trait::async_trait;
use thiserror::Error;

/// テンプレートの依存方向確認用に外部操作を抽象化するGateway。
#[async_trait]
pub trait Gateway: Send + Sync {
    /// テンプレート用の外部操作を実行する。
    async fn execute(&self) -> Result<(), GatewayError>;
}

/// テンプレート用Gatewayで発生した失敗を表す。
#[derive(Debug, Error)]
#[error("gateway operation failed: {0}")]
pub struct GatewayError(pub String);
