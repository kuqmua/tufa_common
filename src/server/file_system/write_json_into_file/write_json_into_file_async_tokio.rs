use crate::common::where_was::WhereWas;
use crate::global_variables::compile_time::git_info::GIT_INFO;
use crate::global_variables::runtime::config::CONFIG;
use crate::traits::get_source::GetSource;
use crate::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use crate::traits::where_was_trait::WhereWasTrait;
use impl_display_for_error_struct::ImplDisplayForErrorStruct;
use impl_display_for_simple_error_enum::ImplDisplayForSimpleErrorEnum;
use impl_error_with_tracing::ImplErrorWithTracingFromCrate;
use impl_get_source::ImplGetSourceFromCrate;
use impl_get_where_was_origin_or_wrapper::ImplGetWhereWasOriginOrWrapperFromCrate;
use init_error::InitErrorFromCrate;

#[derive(
    Debug,
    InitErrorFromCrate,
    ImplErrorWithTracingFromCrate,
    ImplGetWhereWasOriginOrWrapperFromCrate,
    ImplGetSourceFromCrate,
    ImplDisplayForErrorStruct,
)]
pub struct WriteJsonIntoFileAsyncTokioWrapperError {
    source: WriteJsonIntoFileAsyncTokioOriginErrorEnum,
    where_was: WhereWas,
}

#[derive(
    Debug,
    ImplGetSourceFromCrate,
    ImplDisplayForSimpleErrorEnum,
    ImplGetWhereWasOriginOrWrapperFromCrate,
)]
pub enum WriteJsonIntoFileAsyncTokioOriginErrorEnum {
    SerdeJsonOrigin(serde_json::Error),
    StdIoOrigin(std::io::Error),
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn write_json_into_file_async_tokio(
    path: &std::path::Path,
    json_object: serde_json::Value,
    source_place_type: &crate::config_mods::source_place_type::SourcePlaceType,
    should_trace: bool,
) -> Result<(), Box<WriteJsonIntoFileAsyncTokioWrapperError>> {
    match serde_json::to_string_pretty(&json_object) {
        Err(e) => {
            return Err(Box::new(
                WriteJsonIntoFileAsyncTokioWrapperError::init_error_with_possible_trace(
                    WriteJsonIntoFileAsyncTokioOriginErrorEnum::SerdeJsonOrigin(e),
                    WhereWas {
                        time: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .expect("cannot convert time to unix_epoch"),
                        location: *core::panic::Location::caller(),
                    },
                    source_place_type,
                    &GIT_INFO,
                    should_trace,
                ),
            ));
        }
        Ok(stringified_json) => {
            match crate::server::file_system::write_bytes_into_file::write_bytes_into_file_async_tokio::write_bytes_into_file_async_tokio(
                path,
                stringified_json.as_bytes(),
                source_place_type,
                should_trace,
            )
            .await {
                Err(e) => {
                    return Err(Box::new(
                        WriteJsonIntoFileAsyncTokioWrapperError::init_error_with_possible_trace(
                            WriteJsonIntoFileAsyncTokioOriginErrorEnum::StdIoOrigin(e.source),
                            WhereWas {
                                time: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .expect("cannot convert time to unix_epoch"),
                        location: *core::panic::Location::caller(),
                    },
                    source_place_type,
                    &GIT_INFO,
                    should_trace,
                ),
            ));
                },
                Ok(_) => Ok(())
            }
        },
    }
}
