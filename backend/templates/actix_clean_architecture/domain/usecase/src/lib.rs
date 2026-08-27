use std::sync::Arc;

use async_trait::async_trait;
use thiserror::Error;

/// Outbound port implemented by a gateway crate.
#[async_trait]
pub trait Gateway: Send + Sync {
    async fn execute(&self) -> Result<(), GatewayError>;
}

/// Replace this error with errors meaningful to your application.
#[derive(Debug, Error)]
#[error("gateway operation failed: {0}")]
pub struct GatewayError(pub String);

/// Application entry point used by the HTTP layer.
///
/// Add application-specific use case methods here. `execute` only exists to
/// demonstrate the dependency flow in this template.
#[derive(Clone)]
pub struct Interactor {
    gateway: Arc<dyn Gateway>,
}

impl Interactor {
    pub fn new(gateway: Arc<dyn Gateway>) -> Self {
        Self { gateway }
    }

    pub async fn execute(&self) -> Result<(), GatewayError> {
        self.gateway.execute().await
    }
}
