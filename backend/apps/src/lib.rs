mod cors_config;
mod middleware;

use actix_web::{App, HttpServer, web};
use domain_usecase::Interactor;
use http_handlers::{AnonymousAuth, configure_routes};

use crate::{cors_config::build_cors, middleware::request_logger};

pub fn configure_app(cfg: &mut web::ServiceConfig) {
    configure_routes(cfg);
}

pub async fn run(
    listener: std::net::TcpListener,
    interactor: Interactor,
    auth: AnonymousAuth,
) -> std::io::Result<()> {
    let interactor = web::Data::new(interactor);
    let auth = web::Data::new(auth);

    HttpServer::new(move || {
        App::new()
            .wrap(request_logger())
            .wrap(build_cors())
            .app_data(interactor.clone())
            .app_data(auth.clone())
            .configure(configure_app)
    })
    .listen(listener)?
    .run()
    .await
}
