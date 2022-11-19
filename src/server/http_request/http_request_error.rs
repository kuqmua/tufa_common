use crate::common::where_was::WhereWas;
use crate::global_variables::runtime::config::CONFIG;
use crate::traits::where_was_trait::WhereWasTrait;
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
)]
pub struct HttpRequestOriginError {
    pub source: reqwest::Error,
    pub where_was: WhereWas,
}
