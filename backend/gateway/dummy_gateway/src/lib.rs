use async_trait::async_trait;
use domain_usecase::{Gateway, GatewayError};

/// Minimal gateway implementation for running the template as-is.
#[derive(Debug, Default)]
pub struct DummyGateway;

#[async_trait]
impl Gateway for DummyGateway {
    async fn execute(&self) -> Result<(), GatewayError> {
        Ok(())
    }
}
