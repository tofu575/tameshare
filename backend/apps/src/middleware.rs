use actix_web::middleware::Logger;

/// A domain-independent middleware example.
pub fn request_logger() -> Logger {
    Logger::default()
}
