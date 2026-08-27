use actix_cors::Cors;

/// Development-friendly default. Restrict origins for production use.
pub fn build_cors() -> Cors {
    Cors::default()
        .allow_any_origin()
        .allowed_methods(["GET"])
        .allowed_headers(["Content-Type"])
        .max_age(3600)
}
