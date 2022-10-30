use crate::lazy_static::config::CONFIG;
use crate::traits::get_source::GetSource;
use crate::traits::where_was_trait::WhereWasTrait;
use crate::where_was::WhereWas;
use impl_display_for_error_struct::ImplDisplayForErrorStruct;
use impl_display_for_simple_error_enum::ImplDisplayForSimpleErrorEnum;
use impl_error_with_tracing_for_struct_with_get_source_without_get_where_was::ImplErrorWithTracingForStructWithGetSourceWithoutGetWhereWasFromCrate;
use impl_get_source_without_method::ImplGetSourceWithoutMethodFromCrate;
use impl_get_where_was_one_or_many_one_for_error_struct::ImplGetWhereWasOneOrManyOneForErrorStructFromCrate;
use init_error::InitErrorFromCrate;

#[derive(
    Debug,
    InitErrorFromCrate,
    ImplErrorWithTracingForStructWithGetSourceWithoutGetWhereWasFromCrate,
    ImplGetWhereWasOneOrManyOneForErrorStructFromCrate,
    ImplGetSourceWithoutMethodFromCrate,
    ImplDisplayForErrorStruct,
)]
pub struct RemoteAddrError {
    source: RemoteAddrErrorEnum,
    where_was: WhereWas,
}

#[derive(Debug, ImplGetSourceWithoutMethodFromCrate, ImplDisplayForSimpleErrorEnum)]
pub enum RemoteAddrErrorEnum {
    RequestBuilderSend(reqwest::Error),
    StatusCode(reqwest::Error),
}
