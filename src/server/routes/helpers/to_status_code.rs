pub trait ToStatusCode {
    fn to_status_code(&self) -> http::StatusCode;
}
