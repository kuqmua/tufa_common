#[derive(
    Debug,
    thiserror::Error,
    error_occurence::ErrorOccurence,
    type_variants_from_reqwest_response::TypeVariantsFromReqwestResponseFromChecker,
)]
#[type_variants_from_reqwest_response::type_variants_from_reqwest_response_from_checker_paths(
    crate::repositories_types::tufa_server::routes::api::cats::get::TryGet
)]
pub enum QueryExtractorErrorNamed<'a> {
    #[tvfrr_400_bad_request]
    FailedToDeserializeQueryString {
        #[eo_display_with_serialize_deserialize]
        failed_to_deserialize_query_string: std::string::String,
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

impl<'a> std::convert::From<axum::extract::rejection::QueryRejection>
    for QueryExtractorErrorNamed<'a>
{
    fn from(e: axum::extract::rejection::QueryRejection) -> QueryExtractorErrorNamed<'a> {
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
