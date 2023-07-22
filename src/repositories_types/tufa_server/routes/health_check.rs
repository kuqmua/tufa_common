pub async fn health_check() -> actix_web::HttpResponse {
    println!("health_check");
    actix_web::HttpResponse::Ok().finish()
}

pub async fn health_check_axum() -> axum::http::StatusCode {
    println!("health_check");
    axum::http::StatusCode::OK
}
