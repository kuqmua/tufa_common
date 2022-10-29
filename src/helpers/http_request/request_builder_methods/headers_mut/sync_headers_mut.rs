use super::headers_mut_error::HeadersMutError;
use crate::helpers::http_request::request_builder_methods::headers_mut::headers_mut_error::HeadersMutErrorEnum;
use crate::lazy_static::config::CONFIG;
use crate::lazy_static::git_info::GIT_INFO;
use reqwest::blocking::RequestBuilder;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::where_was::WhereWas;

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub fn sync_headers_mut(
    request_builder: RequestBuilder,
    should_trace: bool,
) -> Result<reqwest::header::HeaderMap, Box<HeadersMutError>> {
    match request_builder.send() {
        Err(e) => Err(Box::new(HeadersMutError::init_error_with_possible_trace(
            HeadersMutErrorEnum::RequestBuilderSend(e),
            WhereWas {
                time: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .expect("cannot convert time to unix_epoch"),
                location: *core::panic::Location::caller(),
            },
            &CONFIG.source_place_type,
            &GIT_INFO.data,
            should_trace,
        ))),
        Ok(mut res) => {
            if let Err(e) = res.error_for_status_ref() {
                return Err(Box::new(HeadersMutError::init_error_with_possible_trace(
                    HeadersMutErrorEnum::StatusCode(e),
                    WhereWas {
                        time: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .expect("cannot convert time to unix_epoch"),
                        location: *core::panic::Location::caller(),
                    },
                    &CONFIG.source_place_type,
                    &GIT_INFO.data,
                    should_trace,
                )));
            }
            Ok(res.headers_mut().clone()) //todo do something with it
        }
    }
}
