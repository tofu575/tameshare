use std::sync::Arc;

use domain_usecase::Interactor;
use dummy_gateway::DummyGateway;

/// Composition root: replace `DummyGateway` when wiring a real application.
pub fn build_interactor() -> Interactor {
    Interactor::new(Arc::new(DummyGateway))
}
