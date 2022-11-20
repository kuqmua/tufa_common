// use crate::helpers::http_request::http_request_error::HttpRequestOriginError;
// use crate::helpers::http_request::request_builder_methods::ExtensionsMut::http_request_extensions_mut_error::ExtensionsMutErrorEnum;
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
// pub async fn async_extensions_mut(
//     request_builder: RequestBuilder,
//     should_trace: bool,
// ) -> Result<http::Extensions, Box<ExtensionsMutError>> {
//     match request_builder.send().await {
//         Err(e) => Err(Box::new(
//             ExtensionsMutError::init_error_with_possible_trace(
//                 ExtensionsMutErrorEnum::RequestBuilderSend(e),
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
//                     ExtensionsMutError::init_error_with_possible_trace(
//                         ExtensionsMutErrorEnum::StatusCode(e),
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
//             Ok(res.extensions_mut().clone())
//         }
//     }
// }
