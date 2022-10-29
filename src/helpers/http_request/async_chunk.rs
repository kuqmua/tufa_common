use crate::lazy_static::config::CONFIG;
use crate::lazy_static::git_info::GIT_INFO;
use impl_display_for_error_struct::ImplDisplayForErrorStruct;
use impl_error_with_tracing_for_struct_without_get_source::ImplErrorWithTracingForStructWithoutGetSourceFromTufaCommon;
use impl_get_source_without_method::ImplGetSourceWithoutMethodFromTufaCommon;
use impl_get_where_was_one_or_many_one_for_error_struct::ImplGetWhereWasOneOrManyOneForErrorStructFromTufaCommon;
use init_error::InitErrorFromTufaCommon;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::traits::where_was_trait::WhereWasTrait;
use tufa_common::where_was::WhereWas;

#[derive(
    Debug,
    InitErrorFromTufaCommon,
    ImplErrorWithTracingForStructWithoutGetSourceFromTufaCommon,
    ImplGetWhereWasOneOrManyOneForErrorStructFromTufaCommon,
    ImplGetSourceWithoutMethodFromTufaCommon,
    ImplDisplayForErrorStruct,
)]
pub struct HttpRequestChunkError {
    source: reqwest::Error,
    where_was: WhereWas,
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn async_chunk(
    response: &mut reqwest::Response,
    should_trace: bool,
) -> Result<Option<bytes::Bytes>, Box<HttpRequestChunkError>> {
    match response.chunk().await {
        Err(e) => Err(Box::new(
            HttpRequestChunkError::init_error_with_possible_trace(
                e,
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
        Ok(bytes) => Ok(bytes),
    }
}
