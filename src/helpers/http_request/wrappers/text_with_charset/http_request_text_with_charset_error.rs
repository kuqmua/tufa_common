use crate::helpers::http_request::http_request_error::HttpRequestClientRequestBuilderPrepError;
use crate::helpers::http_request::request_builder_methods::text_with_charset::text_with_charset_error::TextWithCharsetError;
use crate::lazy_static::config::CONFIG;
use impl_error_with_tracing_for_struct_with_get_source_with_get_where_was::ImplErrorWithTracingForStructWithGetSourceWithGetWhereWasFromCrate;
use impl_get_source_with_method::ImplGetSourceWithMethodFromCrate;
use impl_get_where_was_one_or_many_with_method::ImplGetWhereWasOneOrManyWithMethodFromCrate;
use init_error::InitErrorFromCrate;
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
pub struct HttpRequestWrapperTextWithCharsetError {
    source: HttpRequestWrapperTextWithCharsetErrorEnum,
    where_was: WhereWas,
}

#[derive(Debug, ImplGetSourceWithMethodFromCrate, ImplGetWhereWasOneOrManyWithMethodFromCrate)]
pub enum HttpRequestWrapperTextWithCharsetErrorEnum {
    Prep(HttpRequestClientRequestBuilderPrepError),
    TextWithCharset(TextWithCharsetError),
}
