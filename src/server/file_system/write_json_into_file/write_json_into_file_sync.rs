use crate::common::where_was::WhereWas;
use crate::lazy_static::config::CONFIG;
use crate::lazy_static::git_info::GIT_INFO;
use crate::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use crate::traits::where_was_trait::WhereWasTrait;
use impl_display_for_error_struct::ImplDisplayForErrorStruct;
use impl_display_for_simple_error_enum::ImplDisplayForSimpleErrorEnum;
use impl_error_with_tracing_for_struct_without_get_source::ImplErrorWithTracingForStructWithoutGetSourceFromCrate;
use impl_get_source_with_method::ImplGetSourceWithMethodFromCrate;
use impl_get_source_without_method::ImplGetSourceWithoutMethodFromCrate;
use impl_get_where_was_one_or_many_one_for_error_struct::ImplGetWhereWasOneOrManyOneForErrorStructFromCrate;
use init_error::InitErrorFromCrate;

#[derive(
    Debug,
    InitErrorFromCrate,
    ImplErrorWithTracingForStructWithoutGetSourceFromCrate,
    ImplGetWhereWasOneOrManyOneForErrorStructFromCrate,
    ImplGetSourceWithMethodFromCrate,
    ImplDisplayForErrorStruct,
)]
pub struct WriteJsonIntoFileSyncError {
    source: WriteJsonIntoFileSyncErrorEnum,
    where_was: WhereWas,
}

#[derive(Debug, ImplGetSourceWithoutMethodFromCrate, ImplDisplayForSimpleErrorEnum)]
pub enum WriteJsonIntoFileSyncErrorEnum {
    SerdeJsonError(serde_json::Error),
    StdIoError(std::io::Error),
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
) -> Result<(), Box<WriteJsonIntoFileSyncError>> {
    match serde_json::to_string_pretty(&json_object) {
        Err(e) => {
            return Err(Box::new(
                WriteJsonIntoFileSyncError::init_error_with_possible_trace(
                    WriteJsonIntoFileSyncErrorEnum::SerdeJsonError(e),
                    WhereWas {
                        time: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .expect("cannot convert time to unix_epoch"),
                        location: *core::panic::Location::caller(),
                    },
                    source_place_type,
                    &GIT_INFO.data,
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
                        WriteJsonIntoFileSyncError::init_error_with_possible_trace(
                            WriteJsonIntoFileSyncErrorEnum::StdIoError(e.source),
                            WhereWas {
                                time: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .expect("cannot convert time to unix_epoch"),
                                location: *core::panic::Location::caller(),
                            },
                            source_place_type,
                  &GIT_INFO.data,
                           should_trace,
                        ),
                    )
                );
            }
            Ok(())
        }
    }
}
