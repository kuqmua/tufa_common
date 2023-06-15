pub trait IntoActixWebHttpResponse {
    //todo - config here
    fn into_actix_web_http_response(self) -> actix_web::HttpResponse;
}
