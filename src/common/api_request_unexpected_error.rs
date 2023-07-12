#[derive(Debug)]
pub enum ApiRequestUnexpectedError {
    StatusCode {
        status_code: http::StatusCode,
        headers: reqwest::header::HeaderMap,
        response_text: std::string::String,
    },
    DeserializeBody {
        serde: serde_json::Error,
        status_code: http::StatusCode,
        headers: reqwest::header::HeaderMap,
        response_text: std::string::String,
    },
}
