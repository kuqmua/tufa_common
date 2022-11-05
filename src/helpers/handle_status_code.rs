#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub fn handle_status_code(error_status_code: reqwest::StatusCode, link: &str) -> bool {
    if error_status_code == reqwest::StatusCode::CONTINUE {
        println!("status 100(Continue) link: {link}");
    }
    if error_status_code == reqwest::StatusCode::SWITCHING_PROTOCOLS {
        println!("status 101(Switching Protocols) link: {link}");
    }
    if error_status_code == reqwest::StatusCode::PROCESSING {
        println!("status 102(Processing) link: {link}");
    }
    // if error_status_code == reqwest::StatusCode:: {//for some reason there is no 103 status code in reqwest
    //     println!("status 103(Early Hints) link: {link}"); //early metainformation
    // }
    if error_status_code == reqwest::StatusCode::OK {
        println!("status 200(Success) link: {link}");
    }
    if error_status_code == reqwest::StatusCode::CREATED {
        println!("status 201(Created) link: {link}");
    }
    if error_status_code == reqwest::StatusCode::ACCEPTED {
        println!("status 202(Accepted) link: {link}");
    }
    if error_status_code == reqwest::StatusCode::NON_AUTHORITATIVE_INFORMATION {
        println!("status 203(Non-Authoritative Information) link: {link}");
    }
    if error_status_code == reqwest::StatusCode::NO_CONTENT {
        println!("status 204(No Content) link: {link}");
    }
    if error_status_code == reqwest::StatusCode::RESET_CONTENT {
        println!("status 205(Reset Content) link: {link}");
    }
    if error_status_code == reqwest::StatusCode::PARTIAL_CONTENT {
        println!("status 206(Partial Content) link: {link}");
    }
    if error_status_code == reqwest::StatusCode::MULTI_STATUS {
        println!("status 207(Multi-Status) link: {link}");
    }
    if error_status_code == reqwest::StatusCode::ALREADY_REPORTED {
        println!("status 208(Already Reported) link: {link}");
    }
    if error_status_code == reqwest::StatusCode::IM_USED {
        println!("status 226(IM Used) link: {link}");
    }
    //Redirection
    if error_status_code == reqwest::StatusCode::MULTIPLE_CHOICES {
        println!("status 300(Multiple Choices) link: {link}");
    }
    if error_status_code == reqwest::StatusCode::MOVED_PERMANENTLY {
        println!("status 301(Moved Permanently) link: {link}");
    }
    if error_status_code == reqwest::StatusCode::FOUND {
        println!("status 302(Moved Temporarily) link: {link}");
    }
    if error_status_code == reqwest::StatusCode::SEE_OTHER {
        println!("status 303(See Other) link: {link}");
    }
    if error_status_code == reqwest::StatusCode::NOT_MODIFIED {
        println!("status 304(Not Modified) link: {link}");
    }
    if error_status_code == reqwest::StatusCode::USE_PROXY {
        println!("status 305(Use Proxy) link: {link}");
    }
    // if error_status_code == reqwest::StatusCode:: {//for some reason there is no 306 status code in reqwest
    //     println!("status 306() link: {link}"); //— reserved (code used only in early specifications)
    // }
    if error_status_code == reqwest::StatusCode::TEMPORARY_REDIRECT {
        println!("status 307(Temporary Redirect) link: {link}");
    }
    if error_status_code == reqwest::StatusCode::PERMANENT_REDIRECT {
        println!("status 308(Permanent Redirect) link: {link}");
    }
    // 4xx: Client Error:
    if error_status_code == reqwest::StatusCode::BAD_REQUEST {
        println!("status 400(Bad Request) link: {link}");
    }
    if error_status_code == reqwest::StatusCode::UNAUTHORIZED {
        println!("status 401(Unauthorized) link: {link}");
    }
    if error_status_code == reqwest::StatusCode::PAYMENT_REQUIRED {
        println!("status 402(Payment Required) link: {link}");
    }
    if error_status_code == reqwest::StatusCode::FORBIDDEN {
        println!("status 403(Forbidden) link: {link}");
    }
    if error_status_code == reqwest::StatusCode::NOT_FOUND {
        println!("status 404(Not Found) link: {link}");
    }
    if error_status_code == reqwest::StatusCode::METHOD_NOT_ALLOWED {
        println!("status 405(Method Not Allowed) link: {link}");
    }
    if error_status_code == reqwest::StatusCode::NOT_ACCEPTABLE {
        println!("status 406(Not Acceptable) link: {link}");
    }
    if error_status_code == reqwest::StatusCode::PROXY_AUTHENTICATION_REQUIRED {
        println!("status 407(Proxy Authentication Required) link: {link}");
    }
    if error_status_code == reqwest::StatusCode::REQUEST_TIMEOUT {
        println!("status 408(Request Timeout) link: {link}");
    }
    if error_status_code == reqwest::StatusCode::CONFLICT {
        println!("status 409(Conflict) link: {link}");
    }
    if error_status_code == reqwest::StatusCode::GONE {
        println!("status 410(Gone) link: {link}");
    }
    if error_status_code == reqwest::StatusCode::LENGTH_REQUIRED {
        println!("status 411(Length Required) link: {link}");
    }
    if error_status_code == reqwest::StatusCode::PRECONDITION_FAILED {
        println!("status 412(Precondition Failed) link: {link}");
    }
    if error_status_code == reqwest::StatusCode::PAYLOAD_TOO_LARGE {
        println!("status 413(Payload Too Large) link: {link}");
    }
    if error_status_code == reqwest::StatusCode::URI_TOO_LONG {
        println!("status 414(URI Too Long) link: {link}");
    }
    if error_status_code == reqwest::StatusCode::UNSUPPORTED_MEDIA_TYPE {
        println!("status 415(Unsupported Media Type) link: {link}");
    }
    if error_status_code == reqwest::StatusCode::RANGE_NOT_SATISFIABLE {
        println!("status 416(Range Not Satisfiable) link: {link}");
    }
    if error_status_code == reqwest::StatusCode::EXPECTATION_FAILED {
        println!("status 417(Expectation Failed) link: {link}");
    }
    if error_status_code == reqwest::StatusCode::IM_A_TEAPOT {
        //wtf???????

        println!("status 418(I’m a teapot) link: {link}"); //I’m a teapot
    }
    // if error_status_code == reqwest::StatusCode:: {//for some reason there is no 419 status code in reqwest

    //     println!("status 419() link: {link}"); //Authentication Timeout
    // }
    //there is no 420 status code
    if error_status_code == reqwest::StatusCode::MISDIRECTED_REQUEST {
        println!("status 421(Misdirected Request) link: {link}");
    }
    if error_status_code == reqwest::StatusCode::UNPROCESSABLE_ENTITY {
        println!("status 422(Unprocessable Entity) link: {link}");
    }
    if error_status_code == reqwest::StatusCode::LOCKED {
        println!("status 423(Locked) link: {link}");
    }
    if error_status_code == reqwest::StatusCode::FAILED_DEPENDENCY {
        println!("status 424(Failed Dependency) link: {link}");
    }
    // if error_status_code == reqwest::StatusCode:: {//for some reason there is no 425 status code in reqwest

    //     println!("status 425(Too Early) link: {link}");
    // }
    if error_status_code == reqwest::StatusCode::UPGRADE_REQUIRED {
        println!("status 426(Upgrade Required) link: {link}");
    }
    //there is no 427 status code
    if error_status_code == reqwest::StatusCode::PRECONDITION_REQUIRED {
        println!("status 428(Precondition Required) link: {link}");
    }
    if error_status_code == reqwest::StatusCode::TOO_MANY_REQUESTS {
        println!("status 429(Too Many Requests) link: {link}");
    }
    //there is no 430 status code
    if error_status_code == reqwest::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE {
        println!("status 431(Request Header Fields Too Large) link: {link}");
    }
    // if error_status_code == reqwest::StatusCode:: {//for some reason there is no 449 status code in reqwest

    //     println!("status 449(Retry With) link: {link}");
    // }
    if error_status_code == reqwest::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS {
        println!("status 451(Unavailable For Legal Reasons) link: {link}");
    }
    // if error_status_code == reqwest::StatusCode:: {//for some reason there is no 499 status code in reqwest

    //     println!("status 499(Client Closed Request) link: {link}");
    // }
    // 5xx: Server Error:
    if error_status_code == reqwest::StatusCode::INTERNAL_SERVER_ERROR {
        println!("status 500(Internal Server Error) link: {link}");
    }
    if error_status_code == reqwest::StatusCode::NOT_IMPLEMENTED {
        println!("status 501(Not Implemented) link: {link}");
    }
    if error_status_code == reqwest::StatusCode::BAD_GATEWAY {
        println!("status 502(Bad Gateway) link: {link}");
    }
    if error_status_code == reqwest::StatusCode::SERVICE_UNAVAILABLE {
        println!("status 503(Service Unavailable) link: {link}");
    }
    if error_status_code == reqwest::StatusCode::GATEWAY_TIMEOUT {
        println!("status 504(Gateway Timeout) link: {link}");
    }
    if error_status_code == reqwest::StatusCode::HTTP_VERSION_NOT_SUPPORTED {
        println!("status 505(HTTP Version Not Supported) link: {link}");
    }
    if error_status_code == reqwest::StatusCode::VARIANT_ALSO_NEGOTIATES {
        println!("status 506(Variant Also Negotiates) link: {link}");
    }
    if error_status_code == reqwest::StatusCode::INSUFFICIENT_STORAGE {
        println!("status 507(Insufficient Storage) link: {link}");
    }
    if error_status_code == reqwest::StatusCode::LOOP_DETECTED {
        println!("status 508(Loop Detected) link: {link}");
    }
    // if error_status_code == reqwest::StatusCode:: {//for some reason there is no 509 status code in reqwest

    //     println!("status 509(Bandwidth Limit Exceeded) link: {link}");
    // }
    if error_status_code == reqwest::StatusCode::NOT_EXTENDED {
        println!("status 510(Not Extended) link: {link}");
    }
    if error_status_code == reqwest::StatusCode::NETWORK_AUTHENTICATION_REQUIRED {
        println!("status 511(Network Authentication Required) link: {link}");
    }
    // if error_status_code == reqwest::StatusCode:: {

    //     println!("status 520(Unknown Error) link: {link}");
    // }
    // if error_status_code == reqwest::StatusCode:: {

    //     println!("status 521(Web Server Is Down) link: {link}");
    // }
    // if error_status_code == reqwest::StatusCode:: {

    //     println!("status 522(Connection Timed Out) link: {link}");
    // }
    // if error_status_code == reqwest::StatusCode:: {

    //     println!("status 523(Origin Is Unreachable) link: {link}");
    // }
    // if error_status_code == reqwest::StatusCode:: {

    //     println!("status 524(A Timeout Occurred) link: {link}");
    // }
    // if error_status_code == reqwest::StatusCode:: {

    //     println!("status 525(SSL Handshake Failed) link: {link}");
    // }
    // if error_status_code == reqwest::StatusCode:: {

    //     println!("status 526(Invalid SSL Certificate) link: {link}");
    // }
    false
}

// match error_status_code {
//         reqwest::StatusCode::CONTINUE => {
//             println!("status 100(Continue) link: {link}");
//         }
//         reqwest::StatusCode::SWITCHING_PROTOCOLS => {
//             println!("status 101(Switching Protocols) link: {link}");
//         }
//         reqwest::StatusCode::PROCESSING => {
//             println!("status 102(Processing) link: {link}");
//         }
//         reqwest::StatusCode::OK => {
//             println!("status 200(Ok) link: {link}");
//         }
//         reqwest::StatusCode::CREATED => {
//             println!("status 201(Created) link: {link}");
//         }
//         reqwest::StatusCode::ACCEPTED => {
//             println!("status 202(Accepted) link: {link}");
//         }
//         reqwest::StatusCode::NON_AUTHORITATIVE_INFORMATION => {
//             println!("status 203(Non-Authoritative Information) link: {link}");
//         }
//         reqwest::StatusCode::NO_CONTENT => {
//             println!("status 204(No Content) link: {link}");
//         }
//         reqwest::StatusCode::RESET_CONTENT => {
//             println!("status 205(Reset Content) link: {link}");
//         }
//         reqwest::StatusCode::PARTIAL_CONTENT => {
//             println!("status 206(Partial Content) link: {link}");
//         }
//         reqwest::StatusCode::MULTI_STATUS => {
//             println!("status 207(Multi-Status) link: {link}");
//         }
//         reqwest::StatusCode::ALREADY_REPORTED => {
//             println!("status 208(Already Reported) link: {link}");
//         }
//         reqwest::StatusCode::IM_USED => {
//             println!("status 226(IM Used) link: {link}");
//         }
//         reqwest::StatusCode::MULTIPLE_CHOICES => {
//             println!("status 300(Multiple Choices) link: {link}");
//         }
//         reqwest::StatusCode::MOVED_PERMANENTLY => {
//             println!("status 301(Moved Permanently) link: {link}");
//         }
//         reqwest::StatusCode::FOUND => {
//             println!("status 302(Moved Temporarily) link: {link}");
//         }
//         reqwest::StatusCode::SEE_OTHER => {
//             println!("status 303(See Other) link: {link}");
//         }
//         reqwest::StatusCode::NOT_MODIFIED => {
//             println!("status 304(Not Modified) link: {link}");
//         }
//         reqwest::StatusCode::USE_PROXY => {
//             println!("status 305(Use Proxy) link: {link}");
//         }
//         reqwest::StatusCode::TEMPORARY_REDIRECT => {
//             println!("status 307(Temporary Redirect) link: {link}");
//         }
//         reqwest::StatusCode::PERMANENT_REDIRECT => {
//             println!("status 308(Permanent Redirect) link: {link}");
//         }
//         reqwest::StatusCode::BAD_REQUEST => {
//             println!("status 400(Bad Request) link: {link}");
//         }
//         reqwest::StatusCode::UNAUTHORIZED => {
//             println!("status 401(Unauthorized) link: {link}");
//         }
//         reqwest::StatusCode::PAYMENT_REQUIRED => {
//             println!("status 402(Payment Required) link: {link}");
//         }
//         reqwest::StatusCode::FORBIDDEN => {
//             println!("status 403(Forbidden) link: {link}");
//         }
//         reqwest::StatusCode::NOT_FOUND => {
//             println!("status 404(Not Found) link: {link}");
//         }
//         reqwest::StatusCode::METHOD_NOT_ALLOWED => {
//             println!("status 405(Method Not Allowed) link: {link}");
//         }
//         reqwest::StatusCode::NOT_ACCEPTABLE => {
//             println!("status 406(Not Acceptable) link: {link}");
//         }
//         reqwest::StatusCode::PROXY_AUTHENTICATION_REQUIRED => {
//             println!("status 407(Proxy Authentication Required) link: {link}");
//         }
//         reqwest::StatusCode::REQUEST_TIMEOUT => {
//             println!("status 408(Request Timeout) link: {link}");
//         }
//         reqwest::StatusCode::CONFLICT => {
//             println!("status 409(Conflict) link: {link}");
//         }
//         reqwest::StatusCode::GONE => {
//             println!("status 410(Gone) link: {link}");
//         }
//         reqwest::StatusCode::LENGTH_REQUIRED => {
//             println!("status 411(Length Required) link: {link}");
//         }
//         reqwest::StatusCode::PRECONDITION_FAILED => {
//             println!("status 412(Precondition Failed) link: {link}");
//         }
//         reqwest::StatusCode::PAYLOAD_TOO_LARGE => {
//             println!("status 413(Payload Too Large) link: {link}");
//         }
//         reqwest::StatusCode::URI_TOO_LONG => {
//             println!("status 414(URI Too Long) link: {link}");
//         }
//         reqwest::StatusCode::UNSUPPORTED_MEDIA_TYPE => {
//             println!("status 415(Unsupported Media Type) link: {link}");
//         }
//         reqwest::StatusCode::RANGE_NOT_SATISFIABLE => {
//             println!("status 416(Range Not Satisfiable) link: {link}");
//         }
//         reqwest::StatusCode::EXPECTATION_FAILED => {
//             println!("status 417(Expectation Failed) link: {link}");
//         }
//         reqwest::StatusCode::IM_A_TEAPOT => {
//             println!("status 418(I’m a teapot) link: {link}");
//         }
//         reqwest::StatusCode::MISDIRECTED_REQUEST => {
//             println!("status 421(Misdirected Request) link: {link}");
//         }
//         reqwest::StatusCode::UNPROCESSABLE_ENTITY => {
//             println!("status 422(Unprocessable Entity) link: {link}");
//         }
//         reqwest::StatusCode::LOCKED => {
//             println!("status 423(Locked) link: {link}");
//         }
//         reqwest::StatusCode::FAILED_DEPENDENCY => {
//             println!("status 424(Failed Dependency) link: {link}");
//         }
//         reqwest::StatusCode::UPGRADE_REQUIRED => {
//             println!("status 426(Upgrade Required) link: {link}");
//         }
//         reqwest::StatusCode::PRECONDITION_REQUIRED => {
//             println!("status 428(Precondition Required) link: {link}");
//         }
//         reqwest::StatusCode::TOO_MANY_REQUESTS => {
//             println!("status 429(Too Many Requests) link: {link}");
//         }
//         reqwest::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE => {
//             println!("status 431(Request Header Fields Too Large) link: {link}");
//         }
//         reqwest::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS => {
//             println!("status 451(Unavailable For Legal Reasons) link: {link}");
//         }
//         reqwest::StatusCode::INTERNAL_SERVER_ERROR => {
//             println!("status 500(Internal Server Error) link: {link}");
//         }
//         reqwest::StatusCode::NOT_IMPLEMENTED => {
//             println!("status 501(Not Implemented) link: {link}");
//         }
//         reqwest::StatusCode::BAD_GATEWAY => {
//             println!("status 502(Bad Gateway) link: {link}");
//         }
//         reqwest::StatusCode::SERVICE_UNAVAILABLE => {
//             println!("status 503(Service Unavailable) link: {link}");
//         }
//         reqwest::StatusCode::GATEWAY_TIMEOUT => {
//             println!("status 504(Gateway Timeout) link: {link}");
//         }
//         reqwest::StatusCode::HTTP_VERSION_NOT_SUPPORTED => {
//             println!("status 505(HTTP Version Not Supported) link: {link}");
//         }
//         reqwest::StatusCode::VARIANT_ALSO_NEGOTIATES => {
//             println!("status 506(Variant Also Negotiates) link: {link}");
//         }
//         reqwest::StatusCode::INSUFFICIENT_STORAGE => {
//             println!("status 507(Insufficient Storage) link: {link}");
//         }
//         reqwest::StatusCode::LOOP_DETECTED => {
//             println!("status 508(Loop Detected) link: {link}");
//         }
//         reqwest::StatusCode::NOT_EXTENDED => {
//             println!("status 510(Not Extended) link: {link}");
//         }
//         reqwest::StatusCode::NETWORK_AUTHENTICATION_REQUIRED => {
//             println!("status 511(Network Authentication Required) link: {link}");
//         }
//     }
