use crate::common::where_was::WhereWas;
use crate::global_variables::runtime::config::CONFIG;
use crate::traits::where_was_trait::WhereWasTrait;
use impl_error_with_tracing_for_struct_without_get_source::ImplErrorWithTracingForStructWithoutGetSourceFromCrate;
use impl_get_source_with_method::ImplGetSourceWithMethodFromCrate;
use impl_get_where_was_one_or_many_one_for_error_struct::ImplGetWhereWasOneOrManyOneForErrorStructFromCrate;
use init_error::InitErrorFromCrate;

#[derive(
    Debug,
    InitErrorFromCrate,
    ImplErrorWithTracingForStructWithoutGetSourceFromCrate,
    ImplGetWhereWasOneOrManyOneForErrorStructFromCrate,
    ImplGetSourceWithMethodFromCrate,
)]
pub struct HttpRequestOriginError {
    pub source: reqwest::Error,
    pub where_was: WhereWas,
}
