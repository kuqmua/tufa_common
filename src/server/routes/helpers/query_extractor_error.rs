#[derive(
    Debug,
    thiserror::Error,
    error_occurence::ErrorOccurence,
    type_variants_from_reqwest_response::TypeVariantsFromReqwestResponseFromChecker,
)]
#[type_variants_from_reqwest_response::type_variants_from_reqwest_response_from_checker_paths(
    crate::repositories_types::tufa_server::routes::api::cats::TryReadOne
)]
pub enum QueryExtractorErrorNamed {
    #[tvfrr_400_bad_request]
    FailedToDeserializeQueryString {
        #[eo_display_with_serialize_deserialize]
        failed_to_deserialize_query_string: std::string::String,
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

impl std::convert::From<axum::extract::rejection::QueryRejection> for QueryExtractorErrorNamed {
    fn from(e: axum::extract::rejection::QueryRejection) -> QueryExtractorErrorNamed {
        match e {
            axum::extract::rejection::QueryRejection::FailedToDeserializeQueryString(
                failed_to_deserialize_query_string,
            ) => QueryExtractorErrorNamed::FailedToDeserializeQueryString {
                failed_to_deserialize_query_string: failed_to_deserialize_query_string.body_text(),
                code_occurence: crate::code_occurence_tufa_common!(),
            },
            _ => Self::UnexpectedCase {
                unexpected_case: crate::server::routes::helpers::hardcode::UNKNOWN_ERROR
                    .to_string(),
                code_occurence: crate::code_occurence_tufa_common!(),
            },
        }
    }
}

pub trait QueryValueResultExtractor<OkGeneric, ErrorGeneric> {
    fn try_extract_value(
        self,
        app_info: &axum::extract::State<crate::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync>,
    ) -> Result<OkGeneric, ErrorGeneric>;
}

impl<OkGeneric, ErrorGeneric> QueryValueResultExtractor<OkGeneric, ErrorGeneric>
    for Result<axum::extract::Query<OkGeneric>, axum::extract::rejection::QueryRejection>
where
    ErrorGeneric: std::convert::From<
            crate::server::routes::helpers::query_extractor_error::QueryExtractorErrorNamed,
        > + axum::response::IntoResponse,
{
    fn try_extract_value(
        self,
        app_info: &axum::extract::State<crate::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync>,
    ) -> Result<OkGeneric, ErrorGeneric> {
        match self {
            Ok(axum::extract::Query(payload)) => Ok(payload),
            Err(err) => {
                let error = crate::server::routes::helpers::query_extractor_error::QueryExtractorErrorNamed::from(err);
                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                    &error,
                    app_info.as_ref(),
                );
                Err(ErrorGeneric::from(error))
            }
        }
    }
}
// ///////////////////
// impl std::convert::From<QueryExtractorErrorNamed>
//     for crate::repositories_types::tufa_server::routes::api::cats::TryReadManyResponseVariants
// {
//     fn from(val: QueryExtractorErrorNamed) -> Self {
//         match val.into_serialize_deserialize_version() {
//             QueryExtractorErrorNamedWithSerializeDeserialize::FailedToDeserializeQueryString {
//                 failed_to_deserialize_query_string,
//                 code_occurence,
//             } => Self::FailedToDeserializeQueryString {
//                 failed_to_deserialize_query_string,
//                 code_occurence,
//             },
//             QueryExtractorErrorNamedWithSerializeDeserialize::UnexpectedCase {
//                 unexpected_case,
//                 code_occurence,
//             } => Self::UnexpectedCase {
//                 unexpected_case,
//                 code_occurence,
//             },
//         }
//     }
// }
// impl std::convert::From<QueryExtractorErrorNamed>
//     for crate::repositories_types::tufa_server::routes::api::cats::TryReadOneResponseVariants
// {
//     fn from(val: QueryExtractorErrorNamed) -> Self {
//         match val.into_serialize_deserialize_version() {
//             QueryExtractorErrorNamedWithSerializeDeserialize::FailedToDeserializeQueryString {
//                 failed_to_deserialize_query_string,
//                 code_occurence,
//             } => Self::FailedToDeserializeQueryString {
//                 failed_to_deserialize_query_string,
//                 code_occurence,
//             },
//             QueryExtractorErrorNamedWithSerializeDeserialize::UnexpectedCase {
//                 unexpected_case,
//                 code_occurence,
//             } => Self::UnexpectedCase {
//                 unexpected_case,
//                 code_occurence,
//             },
//         }
//     }
// }
// impl std::convert::From<QueryExtractorErrorNamed>
//     for crate::repositories_types::tufa_server::routes::api::cats::TryDeleteManyResponseVariants
// {
//     fn from(val: QueryExtractorErrorNamed) -> Self {
//         match val.into_serialize_deserialize_version() {
//             QueryExtractorErrorNamedWithSerializeDeserialize::FailedToDeserializeQueryString {
//                 failed_to_deserialize_query_string,
//                 code_occurence,
//             } => Self::FailedToDeserializeQueryString {
//                 failed_to_deserialize_query_string,
//                 code_occurence,
//             },
//             QueryExtractorErrorNamedWithSerializeDeserialize::UnexpectedCase {
//                 unexpected_case,
//                 code_occurence,
//             } => Self::UnexpectedCase {
//                 unexpected_case,
//                 code_occurence,
//             },
//         }
//     }
// }
// impl std :: convert :: From < QueryExtractorErrorNamed > for crate ::
// repositories_types :: tufa_server :: routes :: api :: cats ::
// TryReadManyWithSerializeDeserialize
// {
//     fn from(val : QueryExtractorErrorNamed) -> Self
//     {
//         match val.into_serialize_deserialize_version()
//         {
//             QueryExtractorErrorNamedWithSerializeDeserialize ::
//             FailedToDeserializeQueryString
//             { failed_to_deserialize_query_string, code_occurence } => Self ::
//             FailedToDeserializeQueryString
//             { failed_to_deserialize_query_string, code_occurence },
//             QueryExtractorErrorNamedWithSerializeDeserialize :: UnexpectedCase
//             { unexpected_case, code_occurence } => Self :: UnexpectedCase
//             { unexpected_case, code_occurence }
//         }
//     }
// }
// impl std :: convert :: From < QueryExtractorErrorNamed > for crate ::
// repositories_types :: tufa_server :: routes :: api :: cats ::
// TryReadOneWithSerializeDeserialize
// {
//     fn from(val : QueryExtractorErrorNamed) -> Self
//     {
//         match val.into_serialize_deserialize_version()
//         {
//             QueryExtractorErrorNamedWithSerializeDeserialize ::
//             FailedToDeserializeQueryString
//             { failed_to_deserialize_query_string, code_occurence } => Self ::
//             FailedToDeserializeQueryString
//             { failed_to_deserialize_query_string, code_occurence },
//             QueryExtractorErrorNamedWithSerializeDeserialize :: UnexpectedCase
//             { unexpected_case, code_occurence } => Self :: UnexpectedCase
//             { unexpected_case, code_occurence }
//         }
//     }
// }
// impl std :: convert :: From < QueryExtractorErrorNamed > for crate ::
// repositories_types :: tufa_server :: routes :: api :: cats ::
// TryDeleteManyWithSerializeDeserialize
// {
//     fn from(val : QueryExtractorErrorNamed) -> Self
//     {
//         match val.into_serialize_deserialize_version()
//         {
//             QueryExtractorErrorNamedWithSerializeDeserialize ::
//             FailedToDeserializeQueryString
//             { failed_to_deserialize_query_string, code_occurence } => Self ::
//             FailedToDeserializeQueryString
//             { failed_to_deserialize_query_string, code_occurence },
//             QueryExtractorErrorNamedWithSerializeDeserialize :: UnexpectedCase
//             { unexpected_case, code_occurence } => Self :: UnexpectedCase
//             { unexpected_case, code_occurence }
//         }
//     }
// }
// impl From<&QueryExtractorErrorNamed> for http::StatusCode {
//     fn from(val: &QueryExtractorErrorNamed) -> Self {
//         match &val {
//             QueryExtractorErrorNamed::FailedToDeserializeQueryString {
//                 failed_to_deserialize_query_string: _,
//                 code_occurence: _,
//             } => http::StatusCode::BAD_REQUEST,
//             QueryExtractorErrorNamed::UnexpectedCase {
//                 unexpected_case: _,
//                 code_occurence: _,
//             } => http::StatusCode::INTERNAL_SERVER_ERROR,
//         }
//     }
// }
// #[allow(clippy::enum_variant_names, dead_code)]
// enum QueryExtractorErrorNamedStatusCodesChecker {
//     FailedToDeserializeQueryStringTvfrr400BadRequest,
//     UnexpectedCaseTvfrr500InternalServerError,
// }
// impl std::convert::From<QueryExtractorErrorNamedStatusCodesChecker>
//     for crate::repositories_types::tufa_server::routes::api::cats::TryReadManyStatusCodesChecker
// {
//     fn from(val: QueryExtractorErrorNamedStatusCodesChecker) -> Self {
//         match val
//         {
//             QueryExtractorErrorNamedStatusCodesChecker ::
//             FailedToDeserializeQueryStringTvfrr400BadRequest => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryReadManyStatusCodesChecker ::
//             FailedToDeserializeQueryStringTvfrr400BadRequest,
//             QueryExtractorErrorNamedStatusCodesChecker ::
//             UnexpectedCaseTvfrr500InternalServerError => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryReadManyStatusCodesChecker ::
//             UnexpectedCaseTvfrr500InternalServerError
//         }
//     }
// }
// impl std::convert::From<QueryExtractorErrorNamedStatusCodesChecker>
//     for crate::repositories_types::tufa_server::routes::api::cats::TryReadOneStatusCodesChecker
// {
//     fn from(val: QueryExtractorErrorNamedStatusCodesChecker) -> Self {
//         match val
//         {
//             QueryExtractorErrorNamedStatusCodesChecker ::
//             FailedToDeserializeQueryStringTvfrr400BadRequest => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryReadOneStatusCodesChecker ::
//             FailedToDeserializeQueryStringTvfrr400BadRequest,
//             QueryExtractorErrorNamedStatusCodesChecker ::
//             UnexpectedCaseTvfrr500InternalServerError => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryReadOneStatusCodesChecker ::
//             UnexpectedCaseTvfrr500InternalServerError
//         }
//     }
// }
// impl std::convert::From<QueryExtractorErrorNamedStatusCodesChecker>
//     for crate::repositories_types::tufa_server::routes::api::cats::TryDeleteManyStatusCodesChecker
// {
//     fn from(val: QueryExtractorErrorNamedStatusCodesChecker) -> Self {
//         match val
//         {
//             QueryExtractorErrorNamedStatusCodesChecker ::
//             FailedToDeserializeQueryStringTvfrr400BadRequest => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryDeleteManyStatusCodesChecker ::
//             FailedToDeserializeQueryStringTvfrr400BadRequest,
//             QueryExtractorErrorNamedStatusCodesChecker ::
//             UnexpectedCaseTvfrr500InternalServerError => crate ::
//             repositories_types :: tufa_server :: routes :: api :: cats ::
//             TryDeleteManyStatusCodesChecker ::
//             UnexpectedCaseTvfrr500InternalServerError
//         }
//     }
// }
