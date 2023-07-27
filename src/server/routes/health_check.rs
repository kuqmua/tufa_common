async fn health_check_axum() -> axum::http::StatusCode {
    println!("health_check");
    axum::http::StatusCode::OK
}

pub(crate) fn health_check_route() -> axum::Router {
    axum::Router::new().route("/health_check", axum::routing::get(health_check_axum))
}
