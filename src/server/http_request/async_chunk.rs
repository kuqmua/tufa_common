use crate::common::where_was::WhereWas;
use crate::global_variables::compile_time::git_info::GIT_INFO;
use crate::global_variables::runtime::config::CONFIG;
use crate::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use crate::traits::where_was_methods::WhereWasMethods;
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
pub struct HttpRequestChunkOriginError {
    source: reqwest::Error,
    where_was: WhereWas,
}

pub async fn async_chunk(
    response: &mut reqwest::Response,
    source_place_type: &crate::config_mods::source_place_type::SourcePlaceType,
    should_trace: bool,
) -> Result<Option<bytes::Bytes>, Box<HttpRequestChunkOriginError>> {
    match response.chunk().await {
        Err(e) => Err(Box::new(
            HttpRequestChunkOriginError::init_error_with_possible_trace(
                e,
                WhereWas {
                    time: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .expect("cannot convert time to unix_epoch"),
                    file: String::from(file!()),
                    line: line!(),
                    column: column!(),
                    git_info: crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES.clone(),
                },
                source_place_type,
                should_trace,
            ),
        )),
        Ok(bytes) => Ok(bytes),
    }
}
