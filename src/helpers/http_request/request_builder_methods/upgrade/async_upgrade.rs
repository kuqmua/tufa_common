use super::upgrade_error::UpgradeError;
use crate::helpers::http_request::request_builder_methods::upgrade::upgrade_error::UpgradeErrorEnum;
use crate::lazy_static::config::CONFIG;
use crate::lazy_static::git_info::GIT_INFO;
use reqwest::RequestBuilder;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::where_was::WhereWas;

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn async_upgrade(
    request_builder: RequestBuilder,
    should_trace: bool,
) -> Result<reqwest::Upgraded, Box<UpgradeError>> {
    match request_builder.send().await {
        Err(e) => Err(Box::new(UpgradeError::init_error_with_possible_trace(
            UpgradeErrorEnum::RequestBuilderSend(e),
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
                return Err(Box::new(UpgradeError::init_error_with_possible_trace(
                    UpgradeErrorEnum::StatusCode(e),
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
            match res.upgrade().await {
                Err(e) => Err(Box::new(UpgradeError::init_error_with_possible_trace(
                    UpgradeErrorEnum::ResponseUpgrade(e),
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
                Ok(upgrade) => Ok(upgrade),
            }
        }
    }
}
