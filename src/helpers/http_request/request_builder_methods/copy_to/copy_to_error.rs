use crate::lazy_static::config::CONFIG;
use impl_display_for_error_struct::ImplDisplayForErrorStruct;
use impl_display_for_simple_error_enum::ImplDisplayForSimpleErrorEnum;
use impl_error_with_tracing_for_struct_with_get_source_without_get_where_was::ImplErrorWithTracingForStructWithGetSourceWithoutGetWhereWasFromTufaCommon;
use impl_get_source_without_method::ImplGetSourceWithoutMethodFromTufaCommon;
use impl_get_where_was_one_or_many_one_for_error_struct::ImplGetWhereWasOneOrManyOneForErrorStructFromTufaCommon;
use init_error::InitErrorFromTufaCommon;
use tufa_common::traits::get_source::GetSource;
use tufa_common::traits::where_was_trait::WhereWasTrait;
use tufa_common::where_was::WhereWas;

#[derive(
    Debug,
    InitErrorFromTufaCommon,
    ImplErrorWithTracingForStructWithGetSourceWithoutGetWhereWasFromTufaCommon,
    ImplGetWhereWasOneOrManyOneForErrorStructFromTufaCommon,
    ImplGetSourceWithoutMethodFromTufaCommon,
    ImplDisplayForErrorStruct,
)]
pub struct CopyToError {
    source: CopyToErrorEnum,
    where_was: WhereWas,
}

#[derive(Debug, ImplGetSourceWithoutMethodFromTufaCommon, ImplDisplayForSimpleErrorEnum)]
pub enum CopyToErrorEnum {
    RequestBuilderSend(reqwest::Error),
    StatusCode(reqwest::Error),
    ResponseCopyTo(reqwest::Error),
}
