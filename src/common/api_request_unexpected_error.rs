#[derive(Debug)]
pub enum ApiRequestUnexpectedError {
    StatusCode {
        status_code: http::StatusCode,
        headers: reqwest::header::HeaderMap,
        response_text: std::string::String,
    },
    FailedToGetResponseText {
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

// match response.text().await {
//     Ok(response_text) => match serde_json :: from_str :: <
// TryGetResponseVariantsTvfrr200Ok > (& response_text)
// {
//     Ok(value) => Ok(TryGetResponseVariants :: from(value)), Err(e) =>
//     Err(crate :: common :: api_request_unexpected_error ::
//     ApiRequestUnexpectedError :: DeserializeBody
//     { serde : e, status_code, headers, response_text }),
// },
//     Err(e) => Err(
//         crate::common::api_request_unexpected_error::ApiRequestUnexpectedError::FailedToGetResponseText {
//             status_code,
//             headers,
//         }
//     ),
// }
