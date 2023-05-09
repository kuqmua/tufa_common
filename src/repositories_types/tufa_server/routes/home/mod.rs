pub async fn home() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok()
        .content_type(actix_web::http::header::ContentType::html())
        .body(include_str!("home.html"))
}
