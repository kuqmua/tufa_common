use super::version_error::VersionError;
use crate::helpers::http_request::request_builder_methods::version::version_error::VersionErrorEnum;
use crate::lazy_static::config::CONFIG;
use crate::lazy_static::git_info::GIT_INFO;
use crate::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use crate::where_was::WhereWas;

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn async_version(
    request_builder: reqwest::RequestBuilder,
    should_trace: bool,
) -> Result<reqwest::Version, Box<VersionError>> {
    match request_builder.send().await {
        Err(e) => Err(Box::new(VersionError::init_error_with_possible_trace(
            VersionErrorEnum::RequestBuilderSend(e),
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
        Ok(res) => {
            if let Err(e) = res.error_for_status_ref() {
                return Err(Box::new(VersionError::init_error_with_possible_trace(
                    VersionErrorEnum::StatusCode(e),
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
            Ok(res.version())
        }
    }
}