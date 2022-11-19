use crate::common::where_was::WhereWas;
use crate::global_variables::compile_time::git_info::GIT_INFO;
use crate::global_variables::runtime::config::CONFIG;
use crate::traits::get_source::GetSource;
use crate::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use crate::traits::where_was_trait::WhereWasTrait;
use impl_display_for_error::ImplDisplayForError;
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
    ImplDisplayForError,
)]
pub struct WriteJsonIntoFileSyncWrapperError {
    source: WriteJsonIntoFileSyncOriginErrorEnum,
    where_was: WhereWas,
}

#[derive(Debug, ImplGetSourceFromCrate, ImplDisplayForError)]
pub enum WriteJsonIntoFileSyncOriginErrorEnum {
    SerdeJsonOrigin(serde_json::Error),
    StdIoOrigin(std::io::Error),
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub fn write_json_into_file_async(
    path: &std::path::Path,
    json_object: serde_json::Value,
    source_place_type: &crate::config_mods::source_place_type::SourcePlaceType,
    should_trace: bool,
) -> Result<(), Box<WriteJsonIntoFileSyncWrapperError>> {
    match serde_json::to_string_pretty(&json_object) {
        Err(e) => {
            return Err(Box::new(
                WriteJsonIntoFileSyncWrapperError::init_error_with_possible_trace(
                    WriteJsonIntoFileSyncOriginErrorEnum::SerdeJsonOrigin(e),
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
            if let Err(e) = crate::server::file_system::write_bytes_into_file::write_bytes_into_file_sync::write_bytes_into_file_sync(
                path,
                stringified_json.as_bytes(),
                source_place_type,
                should_trace,
            ) {
                return Err(
                    Box::new(
                        WriteJsonIntoFileSyncWrapperError::init_error_with_possible_trace(
                            WriteJsonIntoFileSyncOriginErrorEnum::StdIoOrigin(e.source),
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
                    )
                );
            }
            Ok(())
        }
    }
}
