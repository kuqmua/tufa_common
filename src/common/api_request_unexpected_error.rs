#[derive(Debug)]
pub enum ApiRequestUnexpectedError {
    StatusCode {
        status_code: http::StatusCode,
    },
    DeserializeBody {
        reqwest: reqwest::Error,
        status_code: http::StatusCode,
    },
}
