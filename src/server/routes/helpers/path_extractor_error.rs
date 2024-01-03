// #[derive(
//     Debug,
//     thiserror::Error,
//     error_occurence::ErrorOccurence,
//     type_variants_from_reqwest_response::TypeVariantsFromReqwestResponseFromChecker, //(rust analyzer does not work if type_variants_from_reqwest_response::TypeVariantsFromReqwestResponseFromChecker macro works for some reason)
// )]
// #[type_variants_from_reqwest_response::type_variants_from_reqwest_response_from_checker_paths(
//     crate::repositories_types::tufa_server::routes::api::cats::TryDeleteOne
// )]
// pub enum PathExtractorErrorNamed {
//     #[tvfrr_400_bad_request]
//     FailedToDeserializePathParams {
//         #[eo_display_with_serialize_deserialize]
//         failed_to_deserialize_path_params: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     #[tvfrr_400_bad_request]
//     MissingPathParams {
//         #[eo_display_with_serialize_deserialize]
//         missing_path_params: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     //#[non_exhaustive] case
//     #[tvfrr_500_internal_server_error]
//     UnexpectedCase {
//         #[eo_display_with_serialize_deserialize]
//         unexpected_case: std::string::String,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }

// impl std::convert::From<axum::extract::rejection::PathRejection> for PathExtractorErrorNamed {
//     fn from(e: axum::extract::rejection::PathRejection) -> PathExtractorErrorNamed {
//         match e {
//             axum::extract::rejection::PathRejection::FailedToDeserializePathParams(
//                 failed_to_deserialize_path_params,
//             ) => Self::FailedToDeserializePathParams {
//                 failed_to_deserialize_path_params: failed_to_deserialize_path_params.body_text(),
//                 code_occurence: crate::code_occurence_tufa_common!(),
//             },
//             axum::extract::rejection::PathRejection::MissingPathParams(missing_path_params) => {
//                 Self::MissingPathParams {
//                     missing_path_params: missing_path_params.body_text(),
//                     code_occurence: crate::code_occurence_tufa_common!(),
//                 }
//             }
//             _ => Self::UnexpectedCase {
//                 unexpected_case: crate::server::routes::helpers::hardcode::UNKNOWN_ERROR
//                     .to_string(),
//                 code_occurence: crate::code_occurence_tufa_common!(),
//             },
//         }
//     }
// }

// pub trait PathValueResultExtractor<OkGeneric, ErrorGeneric> {
//     fn try_extract_value(
//         self,
//         app_info: &axum::extract::State<crate::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync>,
//     ) -> Result<OkGeneric, ErrorGeneric>;
// }

// impl<OkGeneric, ErrorGeneric> PathValueResultExtractor<OkGeneric, ErrorGeneric>
//     for Result<axum::extract::Path<OkGeneric>, axum::extract::rejection::PathRejection>
// where
//     ErrorGeneric: std::convert::From<
//             crate::server::routes::helpers::path_extractor_error::PathExtractorErrorNamed,
//         > + axum::response::IntoResponse,
// {
//     fn try_extract_value(
//         self,
//         app_info: &axum::extract::State<crate::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync>,
//     ) -> Result<OkGeneric, ErrorGeneric> {
//         match self {
//             Ok(axum::extract::Path(payload)) => Ok(payload),
//             Err(err) => {
//                 let error = crate::server::routes::helpers::path_extractor_error::PathExtractorErrorNamed::from(err);
//                 crate::common::error_logs_logic::error_log::ErrorLog::error_log(
//                     &error,
//                     app_info.as_ref(),
//                 );
//                 Err(ErrorGeneric::from(error))
//             }
//         }
//     }
// }
// // //////////////////////////(rust analyzer does not work if type_variants_from_reqwest_response::TypeVariantsFromReqwestResponseFromChecker macro works for some reason)
// // impl std::convert::From<PathExtractorErrorNamed>
// //     for crate::repositories_types::tufa_server::routes::api::cats::TryReadOneResponseVariants
// // {
// //     fn from(val: PathExtractorErrorNamed) -> Self {
// //         match val.into_serialize_deserialize_version() {
// //             PathExtractorErrorNamedWithSerializeDeserialize::FailedToDeserializePathParams {
// //                 failed_to_deserialize_path_params,
// //                 code_occurence,
// //             } => Self::FailedToDeserializePathParams {
// //                 failed_to_deserialize_path_params,
// //                 code_occurence,
// //             },
// //             PathExtractorErrorNamedWithSerializeDeserialize::MissingPathParams {
// //                 missing_path_params,
// //                 code_occurence,
// //             } => Self::MissingPathParams {
// //                 missing_path_params,
// //                 code_occurence,
// //             },
// //             PathExtractorErrorNamedWithSerializeDeserialize::UnexpectedCase {
// //                 unexpected_case,
// //                 code_occurence,
// //             } => Self::UnexpectedCase {
// //                 unexpected_case,
// //                 code_occurence,
// //             },
// //         }
// //     }
// // }
// // impl std::convert::From<PathExtractorErrorNamed>
// //     for crate::repositories_types::tufa_server::routes::api::cats::TryUpdateOneResponseVariants
// // {
// //     fn from(val: PathExtractorErrorNamed) -> Self {
// //         match val.into_serialize_deserialize_version() {
// //             PathExtractorErrorNamedWithSerializeDeserialize::FailedToDeserializePathParams {
// //                 failed_to_deserialize_path_params,
// //                 code_occurence,
// //             } => Self::FailedToDeserializePathParams {
// //                 failed_to_deserialize_path_params,
// //                 code_occurence,
// //             },
// //             PathExtractorErrorNamedWithSerializeDeserialize::MissingPathParams {
// //                 missing_path_params,
// //                 code_occurence,
// //             } => Self::MissingPathParams {
// //                 missing_path_params,
// //                 code_occurence,
// //             },
// //             PathExtractorErrorNamedWithSerializeDeserialize::UnexpectedCase {
// //                 unexpected_case,
// //                 code_occurence,
// //             } => Self::UnexpectedCase {
// //                 unexpected_case,
// //                 code_occurence,
// //             },
// //         }
// //     }
// // }
// // impl std::convert::From<PathExtractorErrorNamed>
// //     for crate::repositories_types::tufa_server::routes::api::cats::TryDeleteOneResponseVariants
// // {
// //     fn from(val: PathExtractorErrorNamed) -> Self {
// //         match val.into_serialize_deserialize_version() {
// //             PathExtractorErrorNamedWithSerializeDeserialize::FailedToDeserializePathParams {
// //                 failed_to_deserialize_path_params,
// //                 code_occurence,
// //             } => Self::FailedToDeserializePathParams {
// //                 failed_to_deserialize_path_params,
// //                 code_occurence,
// //             },
// //             PathExtractorErrorNamedWithSerializeDeserialize::MissingPathParams {
// //                 missing_path_params,
// //                 code_occurence,
// //             } => Self::MissingPathParams {
// //                 missing_path_params,
// //                 code_occurence,
// //             },
// //             PathExtractorErrorNamedWithSerializeDeserialize::UnexpectedCase {
// //                 unexpected_case,
// //                 code_occurence,
// //             } => Self::UnexpectedCase {
// //                 unexpected_case,
// //                 code_occurence,
// //             },
// //         }
// //     }
// // }
// // impl std :: convert :: From < PathExtractorErrorNamed > for crate ::
// // repositories_types :: tufa_server :: routes :: api :: cats ::
// // TryReadOneWithSerializeDeserialize
// // {
// //     fn from(val : PathExtractorErrorNamed) -> Self
// //     {
// //         match val.into_serialize_deserialize_version()
// //         {
// //             PathExtractorErrorNamedWithSerializeDeserialize ::
// //             FailedToDeserializePathParams
// //             { failed_to_deserialize_path_params, code_occurence } => Self ::
// //             FailedToDeserializePathParams
// //             { failed_to_deserialize_path_params, code_occurence },
// //             PathExtractorErrorNamedWithSerializeDeserialize ::
// //             MissingPathParams { missing_path_params, code_occurence } => Self
// //             :: MissingPathParams { missing_path_params, code_occurence },
// //             PathExtractorErrorNamedWithSerializeDeserialize :: UnexpectedCase
// //             { unexpected_case, code_occurence } => Self :: UnexpectedCase
// //             { unexpected_case, code_occurence }
// //         }
// //     }
// // }
// // impl std :: convert :: From < PathExtractorErrorNamed > for crate ::
// // repositories_types :: tufa_server :: routes :: api :: cats ::
// // TryUpdateOneWithSerializeDeserialize
// // {
// //     fn from(val : PathExtractorErrorNamed) -> Self
// //     {
// //         match val.into_serialize_deserialize_version()
// //         {
// //             PathExtractorErrorNamedWithSerializeDeserialize ::
// //             FailedToDeserializePathParams
// //             { failed_to_deserialize_path_params, code_occurence } => Self ::
// //             FailedToDeserializePathParams
// //             { failed_to_deserialize_path_params, code_occurence },
// //             PathExtractorErrorNamedWithSerializeDeserialize ::
// //             MissingPathParams { missing_path_params, code_occurence } => Self
// //             :: MissingPathParams { missing_path_params, code_occurence },
// //             PathExtractorErrorNamedWithSerializeDeserialize :: UnexpectedCase
// //             { unexpected_case, code_occurence } => Self :: UnexpectedCase
// //             { unexpected_case, code_occurence }
// //         }
// //     }
// // }
// // impl std :: convert :: From < PathExtractorErrorNamed > for crate ::
// // repositories_types :: tufa_server :: routes :: api :: cats ::
// // TryDeleteOneWithSerializeDeserialize
// // {
// //     fn from(val : PathExtractorErrorNamed) -> Self
// //     {
// //         match val.into_serialize_deserialize_version()
// //         {
// //             PathExtractorErrorNamedWithSerializeDeserialize ::
// //             FailedToDeserializePathParams
// //             { failed_to_deserialize_path_params, code_occurence } => Self ::
// //             FailedToDeserializePathParams
// //             { failed_to_deserialize_path_params, code_occurence },
// //             PathExtractorErrorNamedWithSerializeDeserialize ::
// //             MissingPathParams { missing_path_params, code_occurence } => Self
// //             :: MissingPathParams { missing_path_params, code_occurence },
// //             PathExtractorErrorNamedWithSerializeDeserialize :: UnexpectedCase
// //             { unexpected_case, code_occurence } => Self :: UnexpectedCase
// //             { unexpected_case, code_occurence }
// //         }
// //     }
// // }
// // impl From<&PathExtractorErrorNamed> for http::StatusCode {
// //     fn from(val: &PathExtractorErrorNamed) -> Self {
// //         match &val {
// //             PathExtractorErrorNamed::FailedToDeserializePathParams {
// //                 failed_to_deserialize_path_params: _,
// //                 code_occurence: _,
// //             } => http::StatusCode::BAD_REQUEST,
// //             PathExtractorErrorNamed::MissingPathParams {
// //                 missing_path_params: _,
// //                 code_occurence: _,
// //             } => http::StatusCode::BAD_REQUEST,
// //             PathExtractorErrorNamed::UnexpectedCase {
// //                 unexpected_case: _,
// //                 code_occurence: _,
// //             } => http::StatusCode::INTERNAL_SERVER_ERROR,
// //         }
// //     }
// // }
// // #[allow(clippy::enum_variant_names, dead_code)]
// // enum PathExtractorErrorNamedStatusCodesChecker {
// //     FailedToDeserializePathParamsTvfrr400BadRequest,
// //     MissingPathParamsTvfrr400BadRequest,
// //     UnexpectedCaseTvfrr500InternalServerError,
// // }
// // impl std::convert::From<PathExtractorErrorNamedStatusCodesChecker>
// //     for crate::repositories_types::tufa_server::routes::api::cats::TryReadOneStatusCodesChecker
// // {
// //     fn from(val: PathExtractorErrorNamedStatusCodesChecker) -> Self {
// //         match val
// //         {
// //             PathExtractorErrorNamedStatusCodesChecker ::
// //             FailedToDeserializePathParamsTvfrr400BadRequest => crate ::
// //             repositories_types :: tufa_server :: routes :: api :: cats ::
// //             TryReadOneStatusCodesChecker ::
// //             FailedToDeserializePathParamsTvfrr400BadRequest,
// //             PathExtractorErrorNamedStatusCodesChecker ::
// //             MissingPathParamsTvfrr400BadRequest => crate :: repositories_types
// //             :: tufa_server :: routes :: api :: cats ::
// //             TryReadOneStatusCodesChecker ::
// //             MissingPathParamsTvfrr400BadRequest,
// //             PathExtractorErrorNamedStatusCodesChecker ::
// //             UnexpectedCaseTvfrr500InternalServerError => crate ::
// //             repositories_types :: tufa_server :: routes :: api :: cats ::
// //             TryReadOneStatusCodesChecker ::
// //             UnexpectedCaseTvfrr500InternalServerError
// //         }
// //     }
// // }
// // impl std::convert::From<PathExtractorErrorNamedStatusCodesChecker>
// //     for crate::repositories_types::tufa_server::routes::api::cats::TryUpdateOneStatusCodesChecker
// // {
// //     fn from(val: PathExtractorErrorNamedStatusCodesChecker) -> Self {
// //         match val
// //         {
// //             PathExtractorErrorNamedStatusCodesChecker ::
// //             FailedToDeserializePathParamsTvfrr400BadRequest => crate ::
// //             repositories_types :: tufa_server :: routes :: api :: cats ::
// //             TryUpdateOneStatusCodesChecker ::
// //             FailedToDeserializePathParamsTvfrr400BadRequest,
// //             PathExtractorErrorNamedStatusCodesChecker ::
// //             MissingPathParamsTvfrr400BadRequest => crate :: repositories_types
// //             :: tufa_server :: routes :: api :: cats ::
// //             TryUpdateOneStatusCodesChecker ::
// //             MissingPathParamsTvfrr400BadRequest,
// //             PathExtractorErrorNamedStatusCodesChecker ::
// //             UnexpectedCaseTvfrr500InternalServerError => crate ::
// //             repositories_types :: tufa_server :: routes :: api :: cats ::
// //             TryUpdateOneStatusCodesChecker ::
// //             UnexpectedCaseTvfrr500InternalServerError
// //         }
// //     }
// // }
// // impl std::convert::From<PathExtractorErrorNamedStatusCodesChecker>
// //     for crate::repositories_types::tufa_server::routes::api::cats::TryDeleteOneStatusCodesChecker
// // {
// //     fn from(val: PathExtractorErrorNamedStatusCodesChecker) -> Self {
// //         match val
// //         {
// //             PathExtractorErrorNamedStatusCodesChecker ::
// //             FailedToDeserializePathParamsTvfrr400BadRequest => crate ::
// //             repositories_types :: tufa_server :: routes :: api :: cats ::
// //             TryDeleteOneStatusCodesChecker ::
// //             FailedToDeserializePathParamsTvfrr400BadRequest,
// //             PathExtractorErrorNamedStatusCodesChecker ::
// //             MissingPathParamsTvfrr400BadRequest => crate :: repositories_types
// //             :: tufa_server :: routes :: api :: cats ::
// //             TryDeleteOneStatusCodesChecker ::
// //             MissingPathParamsTvfrr400BadRequest,
// //             PathExtractorErrorNamedStatusCodesChecker ::
// //             UnexpectedCaseTvfrr500InternalServerError => crate ::
// //             repositories_types :: tufa_server :: routes :: api :: cats ::
// //             TryDeleteOneStatusCodesChecker ::
// //             UnexpectedCaseTvfrr500InternalServerError
// //         }
// //     }
// // }
