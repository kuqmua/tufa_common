use super::text_with_charset_error::TextWithCharsetError;
use crate::helpers::http_request::request_builder_methods::text_with_charset::text_with_charset_error::TextWithCharsetErrorEnum;
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
pub async fn async_text_with_charset(
    request_builder: RequestBuilder,
    default_encoding: &str,
    should_trace: bool,
) -> Result<String, Box<TextWithCharsetError>> {
    match request_builder.send().await {
        Err(e) => Err(Box::new(
            TextWithCharsetError::init_error_with_possible_trace(
                TextWithCharsetErrorEnum::RequestBuilderSend(e),
                WhereWas {
                    time: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .expect("cannot convert time to unix_epoch"),
                    location: *core::panic::Location::caller(),
                },
                &CONFIG.source_place_type,
                &GIT_INFO.data,
                should_trace,
            ),
        )),
        Ok(res) => {
            if let Err(e) = res.error_for_status_ref() {
                return Err(Box::new(
                    TextWithCharsetError::init_error_with_possible_trace(
                        TextWithCharsetErrorEnum::StatusCode(e),
                        WhereWas {
                            time: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .expect("cannot convert time to unix_epoch"),
                            location: *core::panic::Location::caller(),
                        },
                        &CONFIG.source_place_type,
                        &GIT_INFO.data,
                        should_trace,
                    ),
                ));
            }
            match res.text_with_charset(default_encoding).await {
                Err(e) => Err(Box::new(
                    TextWithCharsetError::init_error_with_possible_trace(
                        TextWithCharsetErrorEnum::ResponseTextWithCharset(e),
                        WhereWas {
                            time: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .expect("cannot convert time to unix_epoch"),
                            location: *core::panic::Location::caller(),
                        },
                        &CONFIG.source_place_type,
                        &GIT_INFO.data,
                        should_trace,
                    ),
                )),
                Ok(text_with_charset) => Ok(text_with_charset),
            }
        }
    }
}
