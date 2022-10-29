use crate::helpers::http_request::http_request_error::HttpRequestClientRequestBuilderPrepError;
use crate::helpers::http_request::request_builder_methods::error_for_status::error_for_status_error::ErrorForStatusError;
use crate::lazy_static::config::CONFIG;
use impl_error_with_tracing_for_struct_with_get_source_with_get_where_was::ImplErrorWithTracingForStructWithGetSourceWithGetWhereWasFromTufaCommon;
use impl_get_source_with_method::ImplGetSourceWithMethodFromTufaCommon;
use impl_get_where_was_one_or_many_with_method::ImplGetWhereWasOneOrManyWithMethodFromTufaCommon;
use init_error::InitErrorFromTufaCommon;
use crate::traits::get_log_with_additional_where_was::GetLogWithAdditionalWhereWas;
use crate::traits::get_source::GetSource;
use crate::where_was::WhereWas;

#[derive(
    Debug,
    InitErrorFromCrate,
    ImplErrorWithTracingForStructWithGetSourceWithGetWhereWasFromCrate,
    ImplGetWhereWasOneOrManyWithMethodFromCrate,
    ImplGetSourceWithMethodFromCrate,
)]
pub struct HttpRequestWrapperErrorForStatusError {
    source: HttpRequestWrapperErrorForStatusErrorEnum,
    where_was: WhereWas,
}

#[derive(Debug, ImplGetSourceWithMethodFromCrate, ImplGetWhereWasOneOrManyWithMethodFromCrate)]
pub enum HttpRequestWrapperErrorForStatusErrorEnum {
    Prep(HttpRequestClientRequestBuilderPrepError),
    ErrorForStatus(ErrorForStatusError),
}
