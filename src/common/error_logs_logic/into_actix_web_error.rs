pub trait IntoActixWebError {
    fn into_actix_web_error(self) -> actix_web::Error;
}
