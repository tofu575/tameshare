use app::run;
use libs::wire::build_interactor;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .init();

    let listener = std::net::TcpListener::bind(("0.0.0.0", 8080))?;
    tracing::info!(address = %listener.local_addr()?, "server started");

    let interactor = build_interactor()
        .await
        .map_err(|error| std::io::Error::other(error.to_string()))?;
    run(listener, interactor).await
}
