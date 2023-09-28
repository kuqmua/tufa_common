#[derive(
    Debug,
    thiserror::Error,
    error_occurence::ErrorOccurence,
    type_variants_from_reqwest_response::TypeVariantsFromReqwestResponseFromChecker,
)]
#[type_variants_from_reqwest_response::type_variants_from_reqwest_response_from_checker_paths(
    crate::repositories_types::tufa_server::routes::api::cats::TryReadById,
    crate::repositories_types::tufa_server::routes::api::cats::update_by_id::TryUpdateById,
    crate::repositories_types::tufa_server::routes::api::cats::TryDeleteById
)]
pub enum PathExtractorErrorNamed {
    #[tvfrr_400_bad_request]
    FailedToDeserializePathParams {
        #[eo_display_with_serialize_deserialize]
        failed_to_deserialize_path_params: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    MissingPathParams {
        #[eo_display_with_serialize_deserialize]
        missing_path_params: std::string::String,
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

impl std::convert::From<axum::extract::rejection::PathRejection> for PathExtractorErrorNamed {
    fn from(e: axum::extract::rejection::PathRejection) -> PathExtractorErrorNamed {
        match e {
            axum::extract::rejection::PathRejection::FailedToDeserializePathParams(
                failed_to_deserialize_path_params,
            ) => Self::FailedToDeserializePathParams {
                failed_to_deserialize_path_params: failed_to_deserialize_path_params.body_text(),
                code_occurence: crate::code_occurence_tufa_common!(),
            },
            axum::extract::rejection::PathRejection::MissingPathParams(missing_path_params) => {
                Self::MissingPathParams {
                    missing_path_params: missing_path_params.body_text(),
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

pub trait PathValueResultExtractor<OkGeneric, ErrorGeneric> {
    fn try_extract_value(
        self,
        app_info: &axum::extract::State<crate::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync>,
    ) -> Result<OkGeneric, ErrorGeneric>;
}

impl<OkGeneric, ErrorGeneric> PathValueResultExtractor<OkGeneric, ErrorGeneric>
    for Result<axum::extract::Path<OkGeneric>, axum::extract::rejection::PathRejection>
where
    ErrorGeneric: std::convert::From<
            crate::server::routes::helpers::path_extractor_error::PathExtractorErrorNamed,
        > + axum::response::IntoResponse,
{
    fn try_extract_value(
        self,
        app_info: &axum::extract::State<crate::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync>,
    ) -> Result<OkGeneric, ErrorGeneric> {
        match self {
            Ok(axum::extract::Path(payload)) => Ok(payload),
            Err(err) => {
                let error = crate::server::routes::helpers::path_extractor_error::PathExtractorErrorNamed::from(err);
                crate::common::error_logs_logic::error_log::ErrorLog::error_log(
                    &error,
                    app_info.as_ref(),
                );
                Err(ErrorGeneric::from(error))
            }
        }
    }
}
