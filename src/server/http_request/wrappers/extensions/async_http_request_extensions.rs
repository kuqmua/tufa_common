// use crate::helpers::http_request::http_request_error::HttpRequestOriginError;
// use crate::helpers::http_request::request_builder_methods::extensions::http_request_extensions_error::ExtensionsErrorEnum;
// use crate::global_variables::runtime::config::CONFIG;
// use crate::global_variables::compile_time::git_info::GIT_INFO;
// use reqwest::RequestBuilder;
// use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
// use tufa_common::common::where_was::WhereWas;

// #[deny(
//     clippy::indexing_slicing,
//     clippy::unwrap_used,
//     clippy::integer_arithmetic,
//     clippy::float_arithmetic
// )]
// pub async fn async_extensions(
//     request_builder: RequestBuilder,
//     should_trace: bool,
// ) -> Result<http::Extensions, Box<ExtensionsError>> {
//     match request_builder.send().await {
//         Err(e) => Err(Box::new(
//             ExtensionsError::init_error_with_possible_trace(
//                 ExtensionsErrorEnum::RequestBuilderSend(e),
//                 WhereWas {
//                     time: std::time::SystemTime::now()
//                         .duration_since(std::time::UNIX_EPOCH)
//                         .expect("cannot convert time to unix_epoch"),
//                     location: *core::panic::Location::caller(),
//                 },
//                 &CONFIG.source_place_type,
//                 &GIT_INFO,
//                 should_trace,
//             ),
//         )),
//         Ok(res) => {
//             if let Err(e) = res.error_for_status_ref() {
//                 return Err(Box::new(
//                     ExtensionsError::init_error_with_possible_trace(
//                         ExtensionsErrorEnum::StatusCode(e),
//                         WhereWas {
//                             time: std::time::SystemTime::now()
//                                 .duration_since(std::time::UNIX_EPOCH)
//                                 .expect("cannot convert time to unix_epoch"),
//                             location: *core::panic::Location::caller(),
//                         },
//                         &CONFIG.source_place_type,
//                         &GIT_INFO,
//                         should_trace,
//                     ),
//                 ));
//             }
//             Ok(res.extensions().clone())
//         }
//     }
// }
