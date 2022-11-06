use crate::lazy_static::config::CONFIG;
use crate::lazy_static::git_info::GIT_INFO;
use crate::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use crate::traits::where_was_trait::WhereWasTrait;
use crate::where_was::WhereWas;
use impl_display_for_error_struct::ImplDisplayForErrorStruct;
use impl_error_with_tracing_for_struct_without_get_source::ImplErrorWithTracingForStructWithoutGetSourceFromCrate;
use impl_get_source_without_method::ImplGetSourceWithoutMethodFromCrate;
use impl_get_where_was_one_or_many_one_for_error_struct::ImplGetWhereWasOneOrManyOneForErrorStructFromCrate;
use init_error::InitErrorFromCrate;
use std::io::Write;

#[derive(
    Debug,
    InitErrorFromCrate,
    ImplErrorWithTracingForStructWithoutGetSourceFromCrate,
    ImplGetWhereWasOneOrManyOneForErrorStructFromCrate,
    ImplGetSourceWithoutMethodFromCrate,
    ImplDisplayForErrorStruct,
)]
pub struct WriteBytesIntoFileSyncError {
    source: std::io::Error,
    where_was: WhereWas,
}
#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub fn write_bytes_into_file_sync(
    path: &std::path::Path,
    bytes: &[u8],
    source_place_type: &crate::config_mods::source_place_type::SourcePlaceType,
    should_trace: bool,
) -> Result<(), Box<WriteBytesIntoFileSyncError>> {
    if let Some(prefix) = path.parent() {
        if let Err(e) = std::fs::create_dir_all(prefix) {
            return Err(Box::new(
                WriteBytesIntoFileSyncError::init_error_with_possible_trace(
                    e,
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
    }
    match std::fs::File::create(path) {
        Err(e) => {
            return Err(Box::new(
                WriteBytesIntoFileSyncError::init_error_with_possible_trace(
                    e,
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
        Ok(mut file) => {
            if let Err(e) = file.write_all(bytes) {
                return Err(Box::new(
                    WriteBytesIntoFileSyncError::init_error_with_possible_trace(
                        e,
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
            if let Err(e) = file.sync_all() {
                return Err(Box::new(
                    WriteBytesIntoFileSyncError::init_error_with_possible_trace(
                        e,
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
            Ok(())
        }
    }
}
