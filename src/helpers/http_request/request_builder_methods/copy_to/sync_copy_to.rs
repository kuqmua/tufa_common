use super::copy_to_error::CopyToError;
use crate::helpers::http_request::request_builder_methods::copy_to::copy_to_error::CopyToErrorEnum;
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
pub fn sync_copy_to<W: ?Sized>(
    request_builder: RequestBuilder,
    w: &mut W,
    should_trace: bool,
) -> Result<u64, Box<CopyToError>>
where
    W: std::io::Write,
{
    match request_builder.send() {
        Err(e) => Err(Box::new(CopyToError::init_error_with_possible_trace(
            CopyToErrorEnum::RequestBuilderSend(e),
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
                return Err(Box::new(CopyToError::init_error_with_possible_trace(
                    CopyToErrorEnum::StatusCode(e),
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
            match res.copy_to(w) {
                Err(e) => Err(Box::new(CopyToError::init_error_with_possible_trace(
                    CopyToErrorEnum::ResponseCopyTo(e),
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
                Ok(copy_to) => Ok(copy_to),
            }
        }
    }
}
