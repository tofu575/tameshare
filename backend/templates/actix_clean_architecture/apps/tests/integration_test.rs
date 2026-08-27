use actix_web::{App, http::StatusCode, test, web};
use app::configure_app;
use libs::wire::build_interactor;

#[actix_web::test]
async fn health_route_returns_ok() {
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(build_interactor()))
            .configure(configure_app),
    )
    .await;

    let response =
        test::call_service(&app, test::TestRequest::get().uri("/health").to_request()).await;

    assert_eq!(response.status(), StatusCode::OK);
}

#[actix_web::test]
async fn example_route_calls_the_dummy_gateway() {
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(build_interactor()))
            .configure(configure_app),
    )
    .await;

    let response =
        test::call_service(&app, test::TestRequest::get().uri("/example").to_request()).await;

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
