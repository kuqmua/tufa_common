#[derive(
    Debug,
    thiserror::Error,
    error_occurence::ErrorOccurence,
    type_variants_from_reqwest_response::TypeVariantsFromReqwestResponseFromChecker,
)]
#[type_variants_from_reqwest_response::type_variants_from_reqwest_response_from_checker_paths(
    crate::repositories_types::tufa_server::routes::api::cats::delete_by_id::TryDeleteById,
    crate::repositories_types::tufa_server::routes::api::cats::get_by_id::TryGetById
)]
pub enum PathExtractorErrorNamed<'a> {
    #[tvfrr_400_bad_request]
    FailedToDeserializePathParams {
        #[eo_display_with_serialize_deserialize]
        failed_to_deserialize_path_params: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    #[tvfrr_400_bad_request]
    MissingPathParams {
        #[eo_display_with_serialize_deserialize]
        missing_path_params: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    //#[non_exhaustive] case
    #[tvfrr_500_internal_server_error]
    UnexpectedCase {
        #[eo_display_with_serialize_deserialize]
        unexpected_case: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

impl<'a> std::convert::From<axum::extract::rejection::PathRejection>
    for PathExtractorErrorNamed<'a>
{
    fn from(e: axum::extract::rejection::PathRejection) -> PathExtractorErrorNamed<'a> {
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
