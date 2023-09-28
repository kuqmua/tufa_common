#[derive(
    Debug,
    thiserror::Error,
    error_occurence::ErrorOccurence,
    type_variants_from_reqwest_response::TypeVariantsFromReqwestResponseFromChecker,
)]
#[type_variants_from_reqwest_response::type_variants_from_reqwest_response_from_checker_paths(
    crate::repositories_types::tufa_server::routes::api::cats::TryRead,
    crate::repositories_types::tufa_server::routes::api::cats::TryReadById,
    crate::repositories_types::tufa_server::routes::api::cats::TryDelete
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
