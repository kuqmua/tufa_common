use crate::common::where_was::WhereWas;
use crate::global_variables::runtime::config::CONFIG;
use crate::traits::where_was_trait::WhereWasTrait;
use impl_error_with_tracing_for_struct_without_get_source::ImplErrorWithTracingForStructWithoutGetSourceFromCrate;
use impl_get_source::ImplGetSourceFromCrate;
use impl_get_where_was_one_or_many_with_method::ImplGetWhereWasOneOrManyWithMethodFromCrate;
use init_error::InitErrorFromCrate;

#[derive(
    Debug,
    InitErrorFromCrate,
    ImplErrorWithTracingForStructWithoutGetSourceFromCrate,
    ImplGetWhereWasOneOrManyWithMethodFromCrate,
    ImplGetSourceFromCrate,
)]
pub struct HttpRequestOriginError {
    pub source: reqwest::Error,
    pub where_was: WhereWas,
}
