#[derive(
    Debug,
    thiserror::Error,
    error_occurence::ErrorOccurence,
    type_variants_from_reqwest_response::TypeVariantsFromReqwestResponseFromChecker,
)]
#[type_variants_from_reqwest_response::type_variants_from_reqwest_response_from_checker_paths(
    crate::repositories_types::tufa_server::routes::api::cats::post::TryPost,
    crate::repositories_types::tufa_server::routes::api::cats::put::TryPut,
    crate::repositories_types::tufa_server::routes::api::cats::patch::TryPatch
)]
pub enum JsonExtractorErrorNamed<'a> {
    #[tvfrr_400_bad_request]
    JsonDataError {
        #[eo_display_with_serialize_deserialize]
        json_data_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    #[tvfrr_400_bad_request]
    JsonSyntaxError {
        #[eo_display_with_serialize_deserialize]
        json_syntax_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    #[tvfrr_400_bad_request]
    MissingJsonContentType {
        #[eo_display_with_serialize_deserialize]
        json_syntax_error: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    #[tvfrr_500_internal_server_error]
    BytesRejection {
        #[eo_display_with_serialize_deserialize]
        bytes_rejection: std::string::String,
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

impl<'a> std::convert::From<axum::extract::rejection::JsonRejection>
    for JsonExtractorErrorNamed<'a>
{
    fn from(e: axum::extract::rejection::JsonRejection) -> JsonExtractorErrorNamed<'a> {
        match e {
            axum::extract::rejection::JsonRejection::JsonDataError(json_data_error) => JsonExtractorErrorNamed::serde_json_error_response(json_data_error),
            axum::extract::rejection::JsonRejection::JsonSyntaxError(json_syntax_error) => JsonExtractorErrorNamed::serde_json_error_response(json_syntax_error),
            axum::extract::rejection::JsonRejection::MissingJsonContentType(_) => Self::MissingJsonContentType { 
                json_syntax_error: crate::server::routes::helpers::hardcode::MISSING_CONTENT_TYPE_APPLICATION_JSON_HEADER.to_string(), 
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
impl<'a> JsonExtractorErrorNamed<'a> {
    fn serde_json_error_response<E>(err: E) -> Self 
    where
        E: std::error::Error + 'static,
    {
        if let Some(find_error_source_err) = find_error_source::<serde_path_to_error::Error<serde_json::Error>>(&err) {
            JsonExtractorErrorNamed::JsonDataError { 
                json_data_error: format!("{err}: {}", find_error_source_err.inner()), 
                code_occurence: crate::code_occurence_tufa_common!(),
            }
        } else {
            JsonExtractorErrorNamed::UnexpectedCase {
                unexpected_case: crate::server::routes::helpers::hardcode::UNKNOWN_ERROR.to_string(),
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