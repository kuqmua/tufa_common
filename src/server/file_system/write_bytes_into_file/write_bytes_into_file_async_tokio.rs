use crate::common::where_was::WhereWas;
use crate::once_cell_globals::config::CONFIG;
use crate::once_cell_globals::git_info::GIT_INFO;
use crate::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use crate::traits::where_was_trait::WhereWasTrait;
use impl_display_for_error_struct::ImplDisplayForErrorStruct;
use impl_error_with_tracing_for_struct_without_get_source::ImplErrorWithTracingForStructWithoutGetSourceFromCrate;
use impl_get_source_without_method::ImplGetSourceWithoutMethodFromCrate;
use impl_get_where_was_one_or_many_one_for_error_struct::ImplGetWhereWasOneOrManyOneForErrorStructFromCrate;
use init_error::InitErrorFromCrate;
use tokio::io::AsyncWriteExt;

#[derive(
    Debug,
    InitErrorFromCrate,
    ImplErrorWithTracingForStructWithoutGetSourceFromCrate,
    ImplGetWhereWasOneOrManyOneForErrorStructFromCrate,
    ImplGetSourceWithoutMethodFromCrate,
    ImplDisplayForErrorStruct,
)]
pub struct WriteBytesIntoFileAsyncTokioError {
    pub source: std::io::Error,
    pub where_was: WhereWas,
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn write_bytes_into_file_async_tokio(
    path: &std::path::Path,
    bytes: &[u8],
    source_place_type: &crate::config_mods::source_place_type::SourcePlaceType,
    should_trace: bool,
) -> Result<(), Box<WriteBytesIntoFileAsyncTokioError>> {
    if let Some(prefix) = path.parent() {
        if let Err(e) = std::fs::create_dir_all(prefix) {
            return Err(Box::new(
                WriteBytesIntoFileAsyncTokioError::init_error_with_possible_trace(
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
    match tokio::fs::File::open(path).await {
        Err(e) => {
            return Err(Box::new(
                WriteBytesIntoFileAsyncTokioError::init_error_with_possible_trace(
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
            if let Err(e) = file.write_all(bytes).await {
                return Err(Box::new(
                    WriteBytesIntoFileAsyncTokioError::init_error_with_possible_trace(
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
