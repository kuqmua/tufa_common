// use crate::helpers::http_request::http_request_error::HttpRequestOriginError;
// use crate::helpers::http_request::request_builder_methods::cookies::http_request_cookies_error::CookiesErrorEnum;
// use crate::global_variables::runtime::config::CONFIG;
// use crate::global_variables::compile_time::git_info::GIT_INFO;
// use reqwest::blocking::RequestBuilder;
// use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
// use tufa_common::common::where_was::WhereWas;

// #[deny(
//     clippy::indexing_slicing,
//     clippy::unwrap_used,
//     clippy::integer_arithmetic,
//     clippy::float_arithmetic
// )]
// pub fn sync_cookies(
//     request_builder: RequestBuilder,
//     should_trace: bool,
// ) -> Result<reqwest::cookie::Cookie, Box<CookiesError>> {
//     match request_builder.send() {
//         Err(e) => Err(Box::new(
//             CookiesError::init_error_with_possible_trace(
//                 CookiesErrorEnum::RequestBuilderSend(e),
//                 WhereWas {
//                     time: std::time::SystemTime::now()
//                         .duration_since(std::time::UNIX_EPOCH)
//                         .expect("cannot convert time to unix_epoch"),
//                         file: String::from(file!()),
//                         line: line!(),
//                         column: column!(),
//                 },
//                 &CONFIG.source_place_type,
//                 &GIT_INFO,
//                 should_trace,
//             ),
//         )),
//         Ok(res) => {
//             if let Err(e) = res.error_for_status_ref() {
//                 return Err(Box::new(
//                     CookiesError::init_error_with_possible_trace(
//                         CookiesErrorEnum::StatusCode(e),
//                         WhereWas {
//                             time: std::time::SystemTime::now()
//                                 .duration_since(std::time::UNIX_EPOCH)
//                                 .expect("cannot convert time to unix_epoch"),
//                             file: String::from(file!()),
//                             line: line!(),
//                             column: column!(),
//                         },
//                         &CONFIG.source_place_type,
//                         &GIT_INFO,
//                         should_trace,
//                     ),
//                 ));
//             }
//             Ok(res.cookies())
//         }
//     }
// }