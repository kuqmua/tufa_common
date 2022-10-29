// use crate::helpers::http_request::http_request_error::HttpRequestClientRequestBuilderPrepError;
// use crate::helpers::http_request::request_builder_methods::extensions_mut::http_request_extensions_mut_error::ExtensionsMutError;
// use crate::lazy_static::config::CONFIG;
// use impl_error_with_tracing_for_struct_with_get_source_with_get_where_was::ImplErrorWithTracingForStructWithGetSourceWithGetWhereWasFromTufaCommon;
// use impl_get_source_with_method::ImplGetSourceWithMethodFromTufaCommon;
// use impl_get_where_was_one_or_many_with_method::ImplGetWhereWasOneOrManyWithMethodFromTufaCommon;
// use init_error::InitErrorFromTufaCommon;
// use tufa_common::traits::get_log_with_additional_where_was::GetLogWithAdditionalWhereWas;
// use tufa_common::traits::get_source::GetSource;
// use tufa_common::traits::where_was_trait::WhereWasTrait;
// use tufa_common::where_was::WhereWas;

// #[derive(
//     Debug,
//     InitErrorFromTufaCommon,
//     ImplErrorWithTracingForStructWithGetSourceWithGetWhereWasFromTufaCommon,
//     ImplGetWhereWasOneOrManyWithMethodFromTufaCommon,
//     ImplGetSourceWithMethodFromTufaCommon,
// )]
// pub struct HttpRequestWrapperExtensionsMutError {
//     source: HttpRequestWrapperExtensionsMutErrorEnum,
//     where_was: WhereWas,
// }

// #[derive(Debug, ImplGetSourceWithMethodFromTufaCommon, ImplGetWhereWasOneOrManyWithMethodFromTufaCommon)]
// pub enum HttpRequestWrapperExtensionsMutErrorEnum {
//     Prep(HttpRequestClientRequestBuilderPrepError),
//     ExtensionsMut(ExtensionsMutError),
// }
