use actix_web::{HttpResponse, Responder, get, web};
use domain_usecase::Interactor;
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

#[get("/health")]
pub async fn health() -> impl Responder {
    web::Json(HealthResponse { status: "ok" })
}

/// Example of an Actix handler invoking the application layer.
#[get("/example")]
pub async fn example(interactor: web::Data<Interactor>) -> actix_web::Result<HttpResponse> {
    interactor.execute().await.map_err(|error| {
        tracing::error!(%error, "use case failed");
        actix_web::error::ErrorInternalServerError("internal server error")
    })?;

    Ok(HttpResponse::NoContent().finish())
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(health).service(example);
}
