#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ProjectCommitExtractor {}

#[derive(
    Debug,
    thiserror::Error,
    error_occurence::ErrorOccurence,
    type_variants_from_reqwest_response::TypeVariantsFromReqwestResponseFromChecker,
)]
#[type_variants_from_reqwest_response::type_variants_from_reqwest_response_from_checker_paths(
    crate::repositories_types::tufa_server::routes::api::cats::read::TryRead,
    crate::repositories_types::tufa_server::routes::api::cats::TryReadById,
    crate::repositories_types::tufa_server::routes::api::cats::TryCreate,
    crate::repositories_types::tufa_server::routes::api::cats::TryCreateBatch,
    crate::repositories_types::tufa_server::routes::api::cats::TryReadWithBody
)]
pub enum ProjectCommitExtractorCheckErrorNamed {
    #[tvfrr_400_bad_request]
    ProjectCommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        project_commit_not_equal: std::string::String,
        #[eo_display_with_serialize_deserialize]
        project_commit_to_use: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    ProjectCommitExtractorToStrConversion {
        #[eo_display]
        project_commit_to_str_conversion: http::header::ToStrError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    NoProjectCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_project_commit_header: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}

//todo make a proc macro for it or maybe put it into error occurence?
impl crate::common::to_default_stringified_json::ToDefaultStringifiedJson
    for ProjectCommitExtractorCheckErrorNamedWithSerializeDeserialize
{
    fn to_default_stringified_json(&self) -> std::string::String {
        match self {
            ProjectCommitExtractorCheckErrorNamedWithSerializeDeserialize::ProjectCommitExtractorNotEqual { project_commit_not_equal: _, project_commit_to_use: _, code_occurence: _ } => std::string::String::from(
                "{\"ProjectCommitExtractorNotEqual\":{\"project_commit_not_equal\":\"\",\"project_commit_to_use\":\"\",\"code_occurence\":{\"file\":\"\",\"line\":0,\"column\":0,\"git_info\":{\"git_commit_id\":\"\",\"git_repo_link\":\"\"},\"duration\":{\"secs\":0,\"nanos\":0}}}}"
            ),
            ProjectCommitExtractorCheckErrorNamedWithSerializeDeserialize::ProjectCommitExtractorToStrConversion { project_commit_to_str_conversion: _, code_occurence: _ } => std::string::String::from(
                "{\"ProjectCommitExtractorToStrConversion\":{\"project_commit_to_str_conversion\":\"\",\"code_occurence\":{\"file\":\"\",\"line\":0,\"column\":0,\"git_info\":{\"git_commit_id\":\"\",\"git_repo_link\":\"\"},\"duration\":{\"secs\":0,\"nanos\":0}}}}"
            ),
            ProjectCommitExtractorCheckErrorNamedWithSerializeDeserialize::NoProjectCommitExtractorHeader { no_project_commit_header: _, code_occurence: _ } => std::string::String::from(
                "{\"NoProjectCommitExtractorHeader\":{\"no_project_commit_header\":\"no_project_commit_header\",\"code_occurence\":{\"file\":\"\",\"line\":124,\"column\":53,\"git_info\":{\"git_commit_id\":\"\",\"git_repo_link\":\"\"},\"duration\":{\"secs\":0,\"nanos\":0}}}}"
            ),
        }
    }
}
