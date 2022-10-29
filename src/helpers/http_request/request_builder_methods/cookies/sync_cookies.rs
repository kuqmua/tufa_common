// use super::http_request_cookies_error::CookiesError;
// use crate::helpers::http_request::request_builder_methods::cookies::http_request_cookies_error::CookiesErrorEnum;
// use crate::lazy_static::config::CONFIG;
// use crate::lazy_static::git_info::GIT_INFO;
// use reqwest::blocking::RequestBuilder;
// use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
// use tufa_common::where_was::WhereWas;

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
//                     location: *core::panic::Location::caller(),
//                 },
//                 &CONFIG.source_place_type,
//                 &GIT_INFO.data,
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
//                             location: *core::panic::Location::caller(),
//                         },
//                         &CONFIG.source_place_type,
//                         &GIT_INFO.data,
//                         should_trace,
//                     ),
//                 ));
//             }
//             Ok(res.cookies())
//         }
//     }
// }
