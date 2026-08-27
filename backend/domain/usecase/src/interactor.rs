use std::sync::Arc;

use crate::{Gateway, GatewayError};

/// テンプレートのHTTP依存方向を確認する最小Interactor。
#[derive(Clone)]
pub struct Interactor {
    gateway: Arc<dyn Gateway>,
}

impl Interactor {
    /// Interactorへ必須Gatewayを注入する。
    pub fn new(gateway: Arc<dyn Gateway>) -> Self {
        Self { gateway }
    }

    /// テンプレート用Gateway操作を実行する。
    pub async fn execute(&self) -> Result<(), GatewayError> {
        self.gateway.execute().await
    }
}
