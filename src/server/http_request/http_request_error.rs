use crate::common::where_was::WhereWas;
use crate::global_variables::runtime::config::CONFIG;
use crate::traits::where_was_methods::WhereWasMethods;
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

//whats duplication of HttpRequestOriginError. need to use this instead of HttpRequestOriginError later
#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum HttpRequestOriginErrorNamed<'a> {
    Reqwest {
        #[eo_display_foreign_type]
        reqwest_error: reqwest::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}