use crate::common::where_was::WhereWas;
use crate::global_variables::compile_time::git_info::GIT_INFO;
use crate::global_variables::runtime::config::CONFIG;
use crate::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use crate::traits::where_was_trait::WhereWasTrait;
use impl_display_for_error_struct::ImplDisplayForErrorStruct;
use impl_error_with_tracing::ImplErrorWithTracingFromCrate;
use impl_get_source::ImplGetSourceFromCrate;
use impl_get_where_was_origin_or_wrapper::ImplGetWhereWasOriginOrWrapperFromCrate;
use init_error::InitErrorFromCrate;
use std::io::Error;

#[derive(
    Debug,
    InitErrorFromCrate,
    ImplErrorWithTracingFromCrate,
    ImplGetWhereWasOriginOrWrapperFromCrate,
    ImplGetSourceFromCrate,
    ImplDisplayForErrorStruct,
)]
pub struct CreateDirIfItDoesntExistOriginError {
    pub source: Error,
    pub where_was: WhereWas,
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub fn create_dir_if_it_doesnt_exist(
    path: &str,
    source_place_type: &crate::config_mods::source_place_type::SourcePlaceType,
    should_trace: bool,
) -> Result<(), Box<CreateDirIfItDoesntExistOriginError>> {
    if std::path::Path::new(path).exists() {
        return Ok(());
    }
    if let Err(e) = std::fs::create_dir_all(path) {
        return Err(Box::new(
            CreateDirIfItDoesntExistOriginError::init_error_with_possible_trace(
                e,
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
    Ok(())
}
