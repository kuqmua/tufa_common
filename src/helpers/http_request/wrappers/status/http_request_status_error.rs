use crate::lazy_static::config::CONFIG;
use crate::traits::where_was_trait::WhereWasTrait;
use crate::where_was::WhereWas;
use impl_error_with_tracing_for_struct_without_get_source::ImplErrorWithTracingForStructWithoutGetSourceFromCrate;
use impl_get_source_without_method::ImplGetSourceWithoutMethodFromCrate;
use impl_get_where_was_one_or_many_one_for_error_struct::ImplGetWhereWasOneOrManyOneForErrorStructFromCrate;
use init_error::InitErrorFromCrate;

#[derive(
    Debug,
    InitErrorFromCrate,
    ImplErrorWithTracingForStructWithoutGetSourceFromCrate,
    ImplGetWhereWasOneOrManyOneForErrorStructFromCrate,
    ImplGetSourceWithoutMethodFromCrate,
)]
pub struct HttpRequestWrapperStatusError {
    source: reqwest::Error,
    where_was: WhereWas,
}
