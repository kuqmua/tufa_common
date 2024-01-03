#[derive(
    Debug,
    thiserror::Error,
    error_occurence::ErrorOccurence,
    type_variants_from_reqwest_response::TypeVariantsFromReqwestResponseFromChecker, //(rust analyzer does not work if type_variants_from_reqwest_response::TypeVariantsFromReqwestResponseFromChecker macro works for some reason)
)]
#[type_variants_from_reqwest_response::type_variants_from_reqwest_response_from_checker_paths(
    crate::repositories_types::tufa_server::routes::api::cats::TryCreateMany,
    crate::repositories_types::tufa_server::routes::api::cats::TryCreateOne,
    crate::repositories_types::tufa_server::routes::api::cats::TryReadMany,
    crate::repositories_types::tufa_server::routes::api::cats::TryReadOne,
    crate::repositories_types::tufa_server::routes::api::cats::TryUpdateMany,
    crate::repositories_types::tufa_server::routes::api::cats::TryUpdateOne,
    crate::repositories_types::tufa_server::routes::api::cats::TryDeleteMany,
    crate::repositories_types::tufa_server::routes::api::cats::TryDeleteOne
)]
pub enum JsonExtractorErrorNamed {
    #[tvfrr_400_bad_request]
    JsonDataError {
        #[eo_display_with_serialize_deserialize]
        json_data_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    JsonSyntaxError {
        #[eo_display_with_serialize_deserialize]
        json_syntax_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    MissingJsonContentType {
        #[eo_display_with_serialize_deserialize]
        missing_json_content_type: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    BytesRejection {
        #[eo_display_with_serialize_deserialize]
        bytes_rejection: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //#[non_exhaustive] case
    #[tvfrr_500_internal_server_error]
    UnexpectedCase {
        #[eo_display_with_serialize_deserialize]
        unexpected_case: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}

impl std::convert::From<axum::extract::rejection::JsonRejection> for JsonExtractorErrorNamed {
    fn from(e: axum::extract::rejection::JsonRejection) -> JsonExtractorErrorNamed {
        match e {
            axum::extract::rejection::JsonRejection::JsonDataError(json_data_error) => JsonExtractorErrorNamed::serde_json_error_response(json_data_error),
            axum::extract::rejection::JsonRejection::JsonSyntaxError(json_syntax_error) => JsonExtractorErrorNamed::serde_json_error_response(json_syntax_error),
            axum::extract::rejection::JsonRejection::MissingJsonContentType(_) => Self::MissingJsonContentType {
                missing_json_content_type: crate::server::routes::helpers::hardcode::MISSING_CONTENT_TYPE_APPLICATION_JSON_HEADER.to_string(),
                code_occurence: crate::code_occurence_tufa_common!(),
            },
            axum::extract::rejection::JsonRejection::BytesRejection(_) => {
                Self::BytesRejection {
                    bytes_rejection:
                        crate::server::routes::helpers::hardcode::FAILED_TO_BUFFER_REQUEST_BODY
                            .to_string(),
                    code_occurence: crate::code_occurence_tufa_common!(),
                }
            }
            _ => Self::UnexpectedCase {
                unexpected_case: crate::server::routes::helpers::hardcode::UNKNOWN_ERROR
                    .to_string(),
                code_occurence: crate::code_occurence_tufa_common!(),
            },
        }
    }
}

// attempt to extract the inner `serde_path_to_error::Error<serde_json::Error>`,
// if that succeeds we can provide a more specific error.
//
// `Json` uses `serde_path_to_error` so the error will be wrapped in `serde_path_to_error::Error`.
impl JsonExtractorErrorNamed {
    fn serde_json_error_response<E>(err: E) -> Self
    where
        E: std::error::Error + 'static,
    {
        if let Some(find_error_source_err) =
            find_error_source::<serde_path_to_error::Error<serde_json::Error>>(&err)
        {
            JsonExtractorErrorNamed::JsonDataError {
                json_data_error: format!("{err}: {}", find_error_source_err.inner()),
                code_occurence: crate::code_occurence_tufa_common!(),
            }
        } else {
            JsonExtractorErrorNamed::UnexpectedCase {
                unexpected_case: crate::server::routes::helpers::hardcode::UNKNOWN_ERROR
                    .to_string(),
                code_occurence: crate::code_occurence_tufa_common!(),
            }
        }
    }
}

// attempt to downcast `err` into a `T` and if that fails recursively try and
// downcast `err`'s source
fn find_error_source<'a, T>(err: &'a (dyn std::error::Error + 'static)) -> Option<&'a T>
where
    T: std::error::Error + 'static,
{
    if let Some(err) = err.downcast_ref::<T>() {
        Some(err)
    } else if let Some(source) = err.source() {
        find_error_source(source)
    } else {
        None
    }
}

pub trait JsonValueResultExtractor<OkGeneric, ErrorGeneric> {
    fn try_extract_value(
        self,
        app_info: &axum::extract::State<crate::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync>,
    ) -> Result<OkGeneric, ErrorGeneric>;
}

impl<OkGeneric, ErrorGeneric> JsonValueResultExtractor<OkGeneric, ErrorGeneric>
    for Result<axum::Json<OkGeneric>, axum::extract::rejection::JsonRejection>
where
    ErrorGeneric: std::convert::From<
            crate::server::routes::helpers::json_extractor_error::JsonExtractorErrorNamed,
        > + axum::response::IntoResponse,
{
    fn try_extract_value(
        self,
        app_info: &axum::extract::State<crate::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync>,
    ) -> Result<OkGeneric, ErrorGeneric> {
        match self {
            Ok(axum::Json(payload)) => Ok(payload),
            Err(err) => {
                let error = crate::server::routes::helpers::json_extractor_error::JsonExtractorErrorNamed::from(err);
                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                    &error,
                    app_info.as_ref(),
                );
                Err(ErrorGeneric::from(error))
            }
        }
    }
}

// ////////////////////////////todo remove it (rust analyzer does not work if type_variants_from_reqwest_response::TypeVariantsFromReqwestResponseFromChecker macro works for some reason)
// impl std::convert::From<JsonExtractorErrorNamed>
//     for crate::repositories_types::tufa_server::routes::api::cats::TryCreateOneResponseVariants
// {
//     fn from(val: JsonExtractorErrorNamed) -> Self {
//         match val.into_serialize_deserialize_version() {
//             JsonExtractorErrorNamedWithSerializeDeserialize::JsonDataError {
//                 json_data_error,
//                 code_occurence,
//             } => Self::JsonDataError {
//                 json_data_error,
//                 code_occurence,
//             },
//             JsonExtractorErrorNamedWithSerializeDeserialize::JsonSyntaxError {
//                 json_syntax_error,
//                 code_occurence,
//             } => Self::JsonSyntaxError {
//                 json_syntax_error,
//                 code_occurence,
//             },
//             JsonExtractorErrorNamedWithSerializeDeserialize::MissingJsonContentType {
//                 missing_json_content_type,
//                 code_occurence,
//             } => Self::MissingJsonContentType {
//                 missing_json_content_type,
//                 code_occurence,
//             },
//             JsonExtractorErrorNamedWithSerializeDeserialize::BytesRejection {
//                 bytes_rejection,
//                 code_occurence,
//             } => Self::BytesRejection {
//                 bytes_rejection,
//                 code_occurence,
//             },
//             JsonExtractorErrorNamedWithSerializeDeserialize::UnexpectedCase {
//                 unexpected_case,
//                 code_occurence,
//             } => Self::UnexpectedCase {
//                 unexpected_case,
//                 code_occurence,
//             },
//         }
//     }
// }
// impl std::convert::From<JsonExtractorErrorNamed>
//     for crate::repositories_types::tufa_server::routes::api::cats::TryCreateManyResponseVariants
// {
//     fn from(val: JsonExtractorErrorNamed) -> Self {
//         match val.into_serialize_deserialize_version() {
//             JsonExtractorErrorNamedWithSerializeDeserialize::JsonDataError {
//                 json_data_error,
//                 code_occurence,
//             } => Self::JsonDataError {
//                 json_data_error,
//                 code_occurence,
//             },
//             JsonExtractorErrorNamedWithSerializeDeserialize::JsonSyntaxError {
//                 json_syntax_error,
//                 code_occurence,
//             } => Self::JsonSyntaxError {
//                 json_syntax_error,
//                 code_occurence,
//             },
//             JsonExtractorErrorNamedWithSerializeDeserialize::MissingJsonContentType {
//                 missing_json_content_type,
//                 code_occurence,
//             } => Self::MissingJsonContentType {
//                 missing_json_content_type,
//                 code_occurence,
//             },
//             JsonExtractorErrorNamedWithSerializeDeserialize::BytesRejection {
//                 bytes_rejection,
//                 code_occurence,
//             } => Self::BytesRejection {
//                 bytes_rejection,
//                 code_occurence,
//             },
//             JsonExtractorErrorNamedWithSerializeDeserialize::UnexpectedCase {
//                 unexpected_case,
//                 code_occurence,
//             } => Self::UnexpectedCase {
//                 unexpected_case,
//                 code_occurence,
//             },
//         }
//     }
// }
// impl std::convert::From<JsonExtractorErrorNamed>
//     for crate::repositories_types::tufa_server::routes::api::cats::TryUpdateOneResponseVariants
// {
//     fn from(val: JsonExtractorErrorNamed) -> Self {
//         match val.into_serialize_deserialize_version() {
//             JsonExtractorErrorNamedWithSerializeDeserialize::JsonDataError {
//                 json_data_error,
//                 code_occurence,
//             } => Self::JsonDataError {
//                 json_data_error,
//                 code_occurence,
//             },
//             JsonExtractorErrorNamedWithSerializeDeserialize::JsonSyntaxError {
//                 json_syntax_error,
//                 code_occurence,
//             } => Self::JsonSyntaxError {
//                 json_syntax_error,
//                 code_occurence,
//             },
//             JsonExtractorErrorNamedWithSerializeDeserialize::MissingJsonContentType {
//                 missing_json_content_type,
//                 code_occurence,
//             } => Self::MissingJsonContentType {
//                 missing_json_content_type,
//                 code_occurence,
//             },
//             JsonExtractorErrorNamedWithSerializeDeserialize::BytesRejection {
//                 bytes_rejection,
//                 code_occurence,
//             } => Self::BytesRejection {
//                 bytes_rejection,
//                 code_occurence,
//             },
//             JsonExtractorErrorNamedWithSerializeDeserialize::UnexpectedCase {
//                 unexpected_case,
//                 code_occurence,
//             } => Self::UnexpectedCase {
//                 unexpected_case,
//                 code_occurence,
//             },
//         }
//     }
// }
// impl std::convert::From<JsonExtractorErrorNamed>
//     for crate::repositories_types::tufa_server::routes::api::cats::TryUpdateManyResponseVariants
// {
//     fn from(val: JsonExtractorErrorNamed) -> Self {
//         match val.into_serialize_deserialize_version() {
//             JsonExtractorErrorNamedWithSerializeDeserialize::JsonDataError {
//                 json_data_error,
//                 code_occurence,
//             } => Self::JsonDataError {
//                 json_data_error,
//                 code_occurence,
//             },
//             JsonExtractorErrorNamedWithSerializeDeserialize::JsonSyntaxError {
//                 json_syntax_error,
//                 code_occurence,
//             } => Self::JsonSyntaxError {
//                 json_syntax_error,
//                 code_occurence,
//             },
//             JsonExtractorErrorNamedWithSerializeDeserialize::MissingJsonContentType {
//                 missing_json_content_type,
//                 code_occurence,
//             } => Self::MissingJsonContentType {
//                 missing_json_content_type,
//                 code_occurence,
//             },
//             JsonExtractorErrorNamedWithSerializeDeserialize::BytesRejection {
//                 bytes_rejection,
//                 code_occurence,
//             } => Self::BytesRejection {
//                 bytes_rejection,
//                 code_occurence,
//             },
//             JsonExtractorErrorNamedWithSerializeDeserialize::UnexpectedCase {
//                 unexpected_case,
//                 code_occurence,
//             } => Self::UnexpectedCase {
//                 unexpected_case,
//                 code_occurence,
//             },
//         }
//     }
// }
// impl std :: convert :: From < JsonExtractorErrorNamed > for crate ::
// repositories_types :: tufa_server :: routes :: api :: cats ::
// TryReadManyResponseVariants
// {
//     fn from(val : JsonExtractorErrorNamed) -> Self
//     {
//         match val.into_serialize_deserialize_version()
//         {
//             JsonExtractorErrorNamedWithSerializeDeserialize :: JsonDataError
//             { json_data_error, code_occurence } => Self :: JsonDataError
//             { json_data_error, code_occurence },
//             JsonExtractorErrorNamedWithSerializeDeserialize :: JsonSyntaxError
//             { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
//             { json_syntax_error, code_occurence },
//             JsonExtractorErrorNamedWithSerializeDeserialize ::
//             MissingJsonContentType { missing_json_content_type, code_occurence } =>
//             Self :: MissingJsonContentType
//             { missing_json_content_type, code_occurence },
//             JsonExtractorErrorNamedWithSerializeDeserialize :: BytesRejection
//             { bytes_rejection, code_occurence } => Self :: BytesRejection
//             { bytes_rejection, code_occurence },
//             JsonExtractorErrorNamedWithSerializeDeserialize :: UnexpectedCase
//             { unexpected_case, code_occurence } => Self :: UnexpectedCase
//             { unexpected_case, code_occurence }
//         }
//     }
// }
// impl std :: convert :: From < JsonExtractorErrorNamed > for crate ::
// repositories_types :: tufa_server :: routes :: api :: cats ::
// TryDeleteManyResponseVariants
// {
//     fn from(val : JsonExtractorErrorNamed) -> Self
//     {
//         match val.into_serialize_deserialize_version()
//         {
//             JsonExtractorErrorNamedWithSerializeDeserialize :: JsonDataError
//             { json_data_error, code_occurence } => Self :: JsonDataError
//             { json_data_error, code_occurence },
//             JsonExtractorErrorNamedWithSerializeDeserialize :: JsonSyntaxError
//             { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
//             { json_syntax_error, code_occurence },
//             JsonExtractorErrorNamedWithSerializeDeserialize ::
//             MissingJsonContentType { missing_json_content_type, code_occurence } =>
//             Self :: MissingJsonContentType
//             { missing_json_content_type, code_occurence },
//             JsonExtractorErrorNamedWithSerializeDeserialize :: BytesRejection
//             { bytes_rejection, code_occurence } => Self :: BytesRejection
//             { bytes_rejection, code_occurence },
//             JsonExtractorErrorNamedWithSerializeDeserialize :: UnexpectedCase
//             { unexpected_case, code_occurence } => Self :: UnexpectedCase
//             { unexpected_case, code_occurence }
//         }
//     }
// }
// impl std :: convert :: From < JsonExtractorErrorNamed > for crate ::
// repositories_types :: tufa_server :: routes :: api :: cats ::
// TryCreateOneWithSerializeDeserialize
// {
//     fn from(val : JsonExtractorErrorNamed) -> Self
//     {
//         match val.into_serialize_deserialize_version()
//         {
//             JsonExtractorErrorNamedWithSerializeDeserialize :: JsonDataError
//             { json_data_error, code_occurence } => Self :: JsonDataError
//             { json_data_error, code_occurence },
//             JsonExtractorErrorNamedWithSerializeDeserialize :: JsonSyntaxError
//             { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
//             { json_syntax_error, code_occurence },
//             JsonExtractorErrorNamedWithSerializeDeserialize ::
//             MissingJsonContentType { missing_json_content_type, code_occurence } =>
//             Self :: MissingJsonContentType
//             { missing_json_content_type, code_occurence },
//             JsonExtractorErrorNamedWithSerializeDeserialize :: BytesRejection
//             { bytes_rejection, code_occurence } => Self :: BytesRejection
//             { bytes_rejection, code_occurence },
//             JsonExtractorErrorNamedWithSerializeDeserialize :: UnexpectedCase
//             { unexpected_case, code_occurence } => Self :: UnexpectedCase
//             { unexpected_case, code_occurence }
//         }
//     }
// }
// impl std :: convert :: From < JsonExtractorErrorNamed > for crate ::
// repositories_types :: tufa_server :: routes :: api :: cats ::
// TryCreateManyWithSerializeDeserialize
// {
//     fn from(val : JsonExtractorErrorNamed) -> Self
//     {
//         match val.into_serialize_deserialize_version()
//         {
//             JsonExtractorErrorNamedWithSerializeDeserialize :: JsonDataError
//             { json_data_error, code_occurence } => Self :: JsonDataError
//             { json_data_error, code_occurence },
//             JsonExtractorErrorNamedWithSerializeDeserialize :: JsonSyntaxError
//             { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
//             { json_syntax_error, code_occurence },
//             JsonExtractorErrorNamedWithSerializeDeserialize ::
//             MissingJsonContentType { missing_json_content_type, code_occurence } =>
//             Self :: MissingJsonContentType
//             { missing_json_content_type, code_occurence },
//             JsonExtractorErrorNamedWithSerializeDeserialize :: BytesRejection
//             { bytes_rejection, code_occurence } => Self :: BytesRejection
//             { bytes_rejection, code_occurence },
//             JsonExtractorErrorNamedWithSerializeDeserialize :: UnexpectedCase
//             { unexpected_case, code_occurence } => Self :: UnexpectedCase
//             { unexpected_case, code_occurence }
//         }
//     }
// }
// impl std :: convert :: From < JsonExtractorErrorNamed > for crate ::
// repositories_types :: tufa_server :: routes :: api :: cats ::
// TryUpdateOneWithSerializeDeserialize
// {
//     fn from(val : JsonExtractorErrorNamed) -> Self
//     {
//         match val.into_serialize_deserialize_version()
//         {
//             JsonExtractorErrorNamedWithSerializeDeserialize :: JsonDataError
//             { json_data_error, code_occurence } => Self :: JsonDataError
//             { json_data_error, code_occurence },
//             JsonExtractorErrorNamedWithSerializeDeserialize :: JsonSyntaxError
//             { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
//             { json_syntax_error, code_occurence },
//             JsonExtractorErrorNamedWithSerializeDeserialize ::
//             MissingJsonContentType { missing_json_content_type, code_occurence } =>
//             Self :: MissingJsonContentType
//             { missing_json_content_type, code_occurence },
//             JsonExtractorErrorNamedWithSerializeDeserialize :: BytesRejection
//             { bytes_rejection, code_occurence } => Self :: BytesRejection
//             { bytes_rejection, code_occurence },
//             JsonExtractorErrorNamedWithSerializeDeserialize :: UnexpectedCase
//             { unexpected_case, code_occurence } => Self :: UnexpectedCase
//             { unexpected_case, code_occurence }
//         }
//     }
// }
// impl std :: convert :: From < JsonExtractorErrorNamed > for crate ::
// repositories_types :: tufa_server :: routes :: api :: cats ::
// TryUpdateManyWithSerializeDeserialize
// {
//     fn from(val : JsonExtractorErrorNamed) -> Self
//     {
//         match val.into_serialize_deserialize_version()
//         {
//             JsonExtractorErrorNamedWithSerializeDeserialize :: JsonDataError
//             { json_data_error, code_occurence } => Self :: JsonDataError
//             { json_data_error, code_occurence },
//             JsonExtractorErrorNamedWithSerializeDeserialize :: JsonSyntaxError
//             { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
//             { json_syntax_error, code_occurence },
//             JsonExtractorErrorNamedWithSerializeDeserialize ::
//             MissingJsonContentType { missing_json_content_type, code_occurence } =>
//             Self :: MissingJsonContentType
//             { missing_json_content_type, code_occurence },
//             JsonExtractorErrorNamedWithSerializeDeserialize :: BytesRejection
//             { bytes_rejection, code_occurence } => Self :: BytesRejection
//             { bytes_rejection, code_occurence },
//             JsonExtractorErrorNamedWithSerializeDeserialize :: UnexpectedCase
//             { unexpected_case, code_occurence } => Self :: UnexpectedCase
//             { unexpected_case, code_occurence }
//         }
//     }
// }
// impl std :: convert :: From < JsonExtractorErrorNamed > for crate ::
// repositories_types :: tufa_server :: routes :: api :: cats ::
// TryReadManyWithSerializeDeserialize
// {
//     fn from(val : JsonExtractorErrorNamed) -> Self
//     {
//         match val.into_serialize_deserialize_version()
//         {
//             JsonExtractorErrorNamedWithSerializeDeserialize :: JsonDataError
//             { json_data_error, code_occurence } => Self :: JsonDataError
//             { json_data_error, code_occurence },
//             JsonExtractorErrorNamedWithSerializeDeserialize :: JsonSyntaxError
//             { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
//             { json_syntax_error, code_occurence },
//             JsonExtractorErrorNamedWithSerializeDeserialize ::
//             MissingJsonContentType { missing_json_content_type, code_occurence } =>
//             Self :: MissingJsonContentType
//             { missing_json_content_type, code_occurence },
//             JsonExtractorErrorNamedWithSerializeDeserialize :: BytesRejection
//             { bytes_rejection, code_occurence } => Self :: BytesRejection
//             { bytes_rejection, code_occurence },
//             JsonExtractorErrorNamedWithSerializeDeserialize :: UnexpectedCase
//             { unexpected_case, code_occurence } => Self :: UnexpectedCase
//             { unexpected_case, code_occurence }
//         }
//     }
// }
// impl std :: convert :: From < JsonExtractorErrorNamed > for crate ::
// repositories_types :: tufa_server :: routes :: api :: cats ::
// TryDeleteManyWithSerializeDeserialize
// {
//     fn from(val : JsonExtractorErrorNamed) -> Self
//     {
//         match val.into_serialize_deserialize_version()
//         {
//             JsonExtractorErrorNamedWithSerializeDeserialize :: JsonDataError
//             { json_data_error, code_occurence } => Self :: JsonDataError
//             { json_data_error, code_occurence },
//             JsonExtractorErrorNamedWithSerializeDeserialize :: JsonSyntaxError
//             { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
//             { json_syntax_error, code_occurence },
//             JsonExtractorErrorNamedWithSerializeDeserialize ::
//             MissingJsonContentType { missing_json_content_type, code_occurence } =>
//             Self :: MissingJsonContentType
//             { missing_json_content_type, code_occurence },
//             JsonExtractorErrorNamedWithSerializeDeserialize :: BytesRejection
//             { bytes_rejection, code_occurence } => Self :: BytesRejection
//             { bytes_rejection, code_occurence },
//             JsonExtractorErrorNamedWithSerializeDeserialize :: UnexpectedCase
//             { unexpected_case, code_occurence } => Self :: UnexpectedCase
//             { unexpected_case, code_occurence }
//         }
//     }
// }
// impl From<&JsonExtractorErrorNamed> for http::StatusCode {
//     fn from(val: &JsonExtractorErrorNamed) -> Self {
//         match &val {
//             JsonExtractorErrorNamed::JsonDataError {
//                 json_data_error: _,
//                 code_occurence: _,
//             } => http::StatusCode::BAD_REQUEST,
//             JsonExtractorErrorNamed::JsonSyntaxError {
//                 json_syntax_error: _,
//                 code_occurence: _,
//             } => http::StatusCode::BAD_REQUEST,
//             JsonExtractorErrorNamed::MissingJsonContentType {
//                 missing_json_content_type: _,
//                 code_occurence: _,
//             } => http::StatusCode::BAD_REQUEST,
//             JsonExtractorErrorNamed::BytesRejection {
//                 bytes_rejection: _,
//                 code_occurence: _,
//             } => http::StatusCode::INTERNAL_SERVER_ERROR,
//             JsonExtractorErrorNamed::UnexpectedCase {
//                 unexpected_case: _,
//                 code_occurence: _,
//             } => http::StatusCode::INTERNAL_SERVER_ERROR,
//         }
//     }
// }
// #[allow(clippy::enum_variant_names, dead_code)]
// enum JsonExtractorErrorNamedStatusCodesChecker {
//     JsonDataErrorTvfrr400BadRequest,
//     JsonSyntaxErrorTvfrr400BadRequest,
//     MissingJsonContentTypeTvfrr400BadRequest,
//     BytesRejectionTvfrr500InternalServerError,
//     UnexpectedCaseTvfrr500InternalServerError,
// }
// impl std::convert::From<JsonExtractorErrorNamedStatusCodesChecker>
//     for crate::repositories_types::tufa_server::routes::api::cats::TryCreateOneStatusCodesChecker
// {
//     fn from(val: JsonExtractorErrorNamedStatusCodesChecker) -> Self {
//         match val
//         {
//             JsonExtractorErrorNamedStatusCodesChecker ::
//             JsonDataErrorTvfrr400BadRequest => crate :: repositories_types ::
//             tufa_server :: routes :: api :: cats ::
//             TryCreateOneStatusCodesChecker :: JsonDataErrorTvfrr400BadRequest,
//             JsonExtractorErrorNamedStatusCodesChecker ::
//             JsonSyntaxErrorTvfrr400BadRequest => crate :: repositories_types
//             :: tufa_server :: routes :: api :: cats ::
//             TryCreateOneStatusCodesChecker ::
//             JsonSyntaxErrorTvfrr400BadRequest,
//             JsonExtractorErrorNamedStatusCodesChecker ::
//             MissingJsonContentTypeTvfrr400BadRequest => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryCreateOneStatusCodesChecker ::
//             MissingJsonContentTypeTvfrr400BadRequest,
//             JsonExtractorErrorNamedStatusCodesChecker ::
//             BytesRejectionTvfrr500InternalServerError => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryCreateOneStatusCodesChecker ::
//             BytesRejectionTvfrr500InternalServerError,
//             JsonExtractorErrorNamedStatusCodesChecker ::
//             UnexpectedCaseTvfrr500InternalServerError => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryCreateOneStatusCodesChecker ::
//             UnexpectedCaseTvfrr500InternalServerError
//         }
//     }
// }
// impl std::convert::From<JsonExtractorErrorNamedStatusCodesChecker>
//     for crate::repositories_types::tufa_server::routes::api::cats::TryCreateManyStatusCodesChecker
// {
//     fn from(val: JsonExtractorErrorNamedStatusCodesChecker) -> Self {
//         match val
//         {
//             JsonExtractorErrorNamedStatusCodesChecker ::
//             JsonDataErrorTvfrr400BadRequest => crate :: repositories_types ::
//             tufa_server :: routes :: api :: cats ::
//             TryCreateManyStatusCodesChecker ::
//             JsonDataErrorTvfrr400BadRequest,
//             JsonExtractorErrorNamedStatusCodesChecker ::
//             JsonSyntaxErrorTvfrr400BadRequest => crate :: repositories_types
//             :: tufa_server :: routes :: api :: cats ::
//             TryCreateManyStatusCodesChecker ::
//             JsonSyntaxErrorTvfrr400BadRequest,
//             JsonExtractorErrorNamedStatusCodesChecker ::
//             MissingJsonContentTypeTvfrr400BadRequest => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryCreateManyStatusCodesChecker ::
//             MissingJsonContentTypeTvfrr400BadRequest,
//             JsonExtractorErrorNamedStatusCodesChecker ::
//             BytesRejectionTvfrr500InternalServerError => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryCreateManyStatusCodesChecker ::
//             BytesRejectionTvfrr500InternalServerError,
//             JsonExtractorErrorNamedStatusCodesChecker ::
//             UnexpectedCaseTvfrr500InternalServerError => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryCreateManyStatusCodesChecker ::
//             UnexpectedCaseTvfrr500InternalServerError
//         }
//     }
// }
// impl std::convert::From<JsonExtractorErrorNamedStatusCodesChecker>
//     for crate::repositories_types::tufa_server::routes::api::cats::TryUpdateOneStatusCodesChecker
// {
//     fn from(val: JsonExtractorErrorNamedStatusCodesChecker) -> Self {
//         match val
//         {
//             JsonExtractorErrorNamedStatusCodesChecker ::
//             JsonDataErrorTvfrr400BadRequest => crate :: repositories_types ::
//             tufa_server :: routes :: api :: cats ::
//             TryUpdateOneStatusCodesChecker :: JsonDataErrorTvfrr400BadRequest,
//             JsonExtractorErrorNamedStatusCodesChecker ::
//             JsonSyntaxErrorTvfrr400BadRequest => crate :: repositories_types
//             :: tufa_server :: routes :: api :: cats ::
//             TryUpdateOneStatusCodesChecker ::
//             JsonSyntaxErrorTvfrr400BadRequest,
//             JsonExtractorErrorNamedStatusCodesChecker ::
//             MissingJsonContentTypeTvfrr400BadRequest => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryUpdateOneStatusCodesChecker ::
//             MissingJsonContentTypeTvfrr400BadRequest,
//             JsonExtractorErrorNamedStatusCodesChecker ::
//             BytesRejectionTvfrr500InternalServerError => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryUpdateOneStatusCodesChecker ::
//             BytesRejectionTvfrr500InternalServerError,
//             JsonExtractorErrorNamedStatusCodesChecker ::
//             UnexpectedCaseTvfrr500InternalServerError => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryUpdateOneStatusCodesChecker ::
//             UnexpectedCaseTvfrr500InternalServerError
//         }
//     }
// }
// impl std::convert::From<JsonExtractorErrorNamedStatusCodesChecker>
//     for crate::repositories_types::tufa_server::routes::api::cats::TryUpdateManyStatusCodesChecker
// {
//     fn from(val: JsonExtractorErrorNamedStatusCodesChecker) -> Self {
//         match val
//         {
//             JsonExtractorErrorNamedStatusCodesChecker ::
//             JsonDataErrorTvfrr400BadRequest => crate :: repositories_types ::
//             tufa_server :: routes :: api :: cats ::
//             TryUpdateManyStatusCodesChecker ::
//             JsonDataErrorTvfrr400BadRequest,
//             JsonExtractorErrorNamedStatusCodesChecker ::
//             JsonSyntaxErrorTvfrr400BadRequest => crate :: repositories_types
//             :: tufa_server :: routes :: api :: cats ::
//             TryUpdateManyStatusCodesChecker ::
//             JsonSyntaxErrorTvfrr400BadRequest,
//             JsonExtractorErrorNamedStatusCodesChecker ::
//             MissingJsonContentTypeTvfrr400BadRequest => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryUpdateManyStatusCodesChecker ::
//             MissingJsonContentTypeTvfrr400BadRequest,
//             JsonExtractorErrorNamedStatusCodesChecker ::
//             BytesRejectionTvfrr500InternalServerError => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryUpdateManyStatusCodesChecker ::
//             BytesRejectionTvfrr500InternalServerError,
//             JsonExtractorErrorNamedStatusCodesChecker ::
//             UnexpectedCaseTvfrr500InternalServerError => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryUpdateManyStatusCodesChecker ::
//             UnexpectedCaseTvfrr500InternalServerError
//         }
//     }
// }
// impl std :: convert :: From < JsonExtractorErrorNamedStatusCodesChecker >
// for crate :: repositories_types :: tufa_server :: routes :: api :: cats ::
// TryReadManyStatusCodesChecker
// {
//     fn from(val : JsonExtractorErrorNamedStatusCodesChecker,) -> Self
//     {
//         match val
//         {
//             JsonExtractorErrorNamedStatusCodesChecker ::
//             JsonDataErrorTvfrr400BadRequest => crate :: repositories_types ::
//             tufa_server :: routes :: api :: cats ::
//             TryReadManyStatusCodesChecker ::
//             JsonDataErrorTvfrr400BadRequest,
//             JsonExtractorErrorNamedStatusCodesChecker ::
//             JsonSyntaxErrorTvfrr400BadRequest => crate :: repositories_types
//             :: tufa_server :: routes :: api :: cats ::
//             TryReadManyStatusCodesChecker ::
//             JsonSyntaxErrorTvfrr400BadRequest,
//             JsonExtractorErrorNamedStatusCodesChecker ::
//             MissingJsonContentTypeTvfrr400BadRequest => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryReadManyStatusCodesChecker ::
//             MissingJsonContentTypeTvfrr400BadRequest,
//             JsonExtractorErrorNamedStatusCodesChecker ::
//             BytesRejectionTvfrr500InternalServerError => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryReadManyStatusCodesChecker ::
//             BytesRejectionTvfrr500InternalServerError,
//             JsonExtractorErrorNamedStatusCodesChecker ::
//             UnexpectedCaseTvfrr500InternalServerError => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryReadManyStatusCodesChecker ::
//             UnexpectedCaseTvfrr500InternalServerError
//         }
//     }
// }
// impl std :: convert :: From < JsonExtractorErrorNamedStatusCodesChecker >
// for crate :: repositories_types :: tufa_server :: routes :: api :: cats ::
// TryDeleteManyStatusCodesChecker
// {
//     fn from(val : JsonExtractorErrorNamedStatusCodesChecker,) -> Self
//     {
//         match val
//         {
//             JsonExtractorErrorNamedStatusCodesChecker ::
//             JsonDataErrorTvfrr400BadRequest => crate :: repositories_types ::
//             tufa_server :: routes :: api :: cats ::
//             TryDeleteManyStatusCodesChecker ::
//             JsonDataErrorTvfrr400BadRequest,
//             JsonExtractorErrorNamedStatusCodesChecker ::
//             JsonSyntaxErrorTvfrr400BadRequest => crate :: repositories_types
//             :: tufa_server :: routes :: api :: cats ::
//             TryDeleteManyStatusCodesChecker ::
//             JsonSyntaxErrorTvfrr400BadRequest,
//             JsonExtractorErrorNamedStatusCodesChecker ::
//             MissingJsonContentTypeTvfrr400BadRequest => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryDeleteManyStatusCodesChecker ::
//             MissingJsonContentTypeTvfrr400BadRequest,
//             JsonExtractorErrorNamedStatusCodesChecker ::
//             BytesRejectionTvfrr500InternalServerError => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryDeleteManyStatusCodesChecker ::
//             BytesRejectionTvfrr500InternalServerError,
//             JsonExtractorErrorNamedStatusCodesChecker ::
//             UnexpectedCaseTvfrr500InternalServerError => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryDeleteManyStatusCodesChecker ::
//             UnexpectedCaseTvfrr500InternalServerError
//         }
//     }
// }
