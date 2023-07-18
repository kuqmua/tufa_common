#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ProjectCommitExtractor {}

#[derive(
    Debug, 
    thiserror::Error, 
    error_occurence::ErrorOccurence,
    type_variants_from_reqwest_response::TypeVariantsFromReqwestResponseFromChecker,
)]
#[type_variants_from_reqwest_response::type_variants_from_reqwest_response_from_checker_paths(
    crate::repositories_types::tufa_server::routes::api::cats::get::TryGet,
    crate::repositories_types::tufa_server::routes::api::cats::get_by_id::TryGetById,
    crate::repositories_types::tufa_server::routes::api::cats::post::TryPost
)]
pub enum ProjectCommitExtractorCheckErrorNamed<'a> {
    #[tvfrr_400_bad_request]
    ProjectCommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        project_commit_not_equal: &'a str,
        #[eo_display_with_serialize_deserialize]
        project_commit_to_use: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    #[tvfrr_400_bad_request]
    ProjectCommitExtractorToStrConversion {
        #[eo_display]
        project_commit_to_str_conversion: http::header::ToStrError,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    #[tvfrr_400_bad_request]
    NoProjectCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_project_commit_header: &'a str,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

//todo make a proc macro for it or maybe put it into error occurence?
impl crate::common::to_default_stringified_json::ToDefaultStringifiedJson for ProjectCommitExtractorCheckErrorNamedWithSerializeDeserialize {
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

impl actix_web::FromRequest for ProjectCommitExtractor {
    type Error = actix_web::Error;
    type Future = std::future::Ready<Result<Self, Self::Error>>;
    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        match req
            .headers()
            .get(crate::common::git::project_git_info::PROJECT_COMMIT)
        {
            Some(project_commit_header_value) => {
                match project_commit_header_value.to_str() {
                    Ok(possible_project_commit) => {
                        match possible_project_commit
                        == crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO
                            .project_commit
                    {
                        true => std::future::ready(Ok(ProjectCommitExtractor {})),
                        false => {
                            std::future::ready(Err({
                                let error_with_serialize_deserialize = ProjectCommitExtractorCheckErrorNamed::ProjectCommitExtractorNotEqual {
                                    project_commit_not_equal: "different project commit provided, services must work only with equal project commits",
                                    project_commit_to_use: {
                                        use crate::common::git::get_git_commit_link::GetGitCommitLink;
                                        crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO.get_git_commit_link()
                                    },
                                    code_occurence: crate::code_occurence_tufa_common!(),
                                }.into_serialize_deserialize_version();
                                actix_web::error::ErrorBadRequest(actix_web::web::Json(serde_json::to_string(&error_with_serialize_deserialize).unwrap_or_else(|_|{
                                    crate::common::to_default_stringified_json::ToDefaultStringifiedJson::to_default_stringified_json(&error_with_serialize_deserialize)
                                })))
                            }))
                        }
                    }
                    }
                    Err(e) => std::future::ready(Err({
                        let error_with_serialize_deserialize = ProjectCommitExtractorCheckErrorNamed::ProjectCommitExtractorToStrConversion { 
                            project_commit_to_str_conversion: e, 
                            code_occurence: crate::code_occurence_tufa_common!() 
                        }.into_serialize_deserialize_version();
                        actix_web::error::ErrorBadRequest(actix_web::web::Json(serde_json::to_string(&error_with_serialize_deserialize).unwrap_or_else(|_|{
                            crate::common::to_default_stringified_json::ToDefaultStringifiedJson::to_default_stringified_json(&error_with_serialize_deserialize)
                        })))
                    })),
                }
            }
            None => std::future::ready(Err({
                let error_with_serialize_deserialize = ProjectCommitExtractorCheckErrorNamed::NoProjectCommitExtractorHeader { 
                    no_project_commit_header: "project_commit header is not provided", 
                    code_occurence: crate::code_occurence_tufa_common!() 
                }.into_serialize_deserialize_version();
                actix_web::error::ErrorBadRequest(actix_web::web::Json(serde_json::to_string(&error_with_serialize_deserialize).unwrap_or_else(|_|{
                    crate::common::to_default_stringified_json::ToDefaultStringifiedJson::to_default_stringified_json(&error_with_serialize_deserialize)
                })))
            })),
        }
    }
}

