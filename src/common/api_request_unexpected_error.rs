#[derive(Debug)]
pub enum ApiRequestUnexpectedError {
    StatusCode {
        status_code: http::StatusCode,
        headers: reqwest::header::HeaderMap,
        response_text: std::string::String,
    },
    FailedToGetResponseText {
        reqwest: reqwest::Error,
        status_code: http::StatusCode,
        headers: reqwest::header::HeaderMap,
    },
    DeserializeBody {
        serde: serde_json::Error,
        status_code: http::StatusCode,
        headers: reqwest::header::HeaderMap,
        response_text: std::string::String,
    },
}
