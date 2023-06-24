#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ProjectCommitExtractor {}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence, from_enum::FromEnum)]
#[from_enum::from_enum_paths(
    crate::repositories_types::tufa_server::routes::api::cats::get::request::GetHttpResponseVariants,
    crate::repositories_types::tufa_server::routes::api::cats::get::request::TryGetErrorHttpResponseWithSerializeDeserialize
)]
pub enum ProjectCommitExtractorCheckErrorNamed<'a> {
    ProjectCommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        project_commit_not_equal: &'a str,
        #[eo_display_with_serialize_deserialize]
        project_commit_to_use: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    ProjectCommitExtractorToStrConversion {
        #[eo_display]
        project_commit_to_str_conversion: http::header::ToStrError,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
    NoProjectCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_project_commit_header: &'a str,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

impl<'a> From<ProjectCommitExtractorCheckErrorNamed<'a>> for actix_web::Error  {
    fn from(val: ProjectCommitExtractorCheckErrorNamed<'a>) -> Self {
        match &val {
            ProjectCommitExtractorCheckErrorNamed::ProjectCommitExtractorNotEqual { 
                project_commit_not_equal: _, 
                project_commit_to_use: _, 
                code_occurence: _ 
            } => actix_web::error::ErrorBadRequest(
                actix_web::web::Json(
                    val.into_serialize_deserialize_version()
                )
            ),
            ProjectCommitExtractorCheckErrorNamed::ProjectCommitExtractorToStrConversion { 
                project_commit_to_str_conversion: _, 
                code_occurence: _ 
            } => actix_web::error::ErrorBadRequest(
                actix_web::web::Json(
                    val.into_serialize_deserialize_version()
                )
            ),
            ProjectCommitExtractorCheckErrorNamed::NoProjectCommitExtractorHeader { 
                no_project_commit_header: _, 
                code_occurence: _ 
            } => actix_web::error::ErrorBadRequest(
                actix_web::web::Json(
                    val.into_serialize_deserialize_version()
                )
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
            Some(project_commit_header_value) => match project_commit_header_value.to_str() {
                Ok(possible_project_commit) => {
                    match possible_project_commit == crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO.project_commit {
                        true => std::future::ready(Ok(ProjectCommitExtractor {})),
                        false => std::future::ready(Err({
                            let error = ProjectCommitExtractorCheckErrorNamed::ProjectCommitExtractorNotEqual {              
                                project_commit_not_equal: "different project commit provided, services must work only with equal project commits", 
                                project_commit_to_use: {
                                    use crate::common::git::get_git_commit_link::GetGitCommitLink;
                                    crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO.get_git_commit_link()
                                },
                                code_occurence: crate::code_occurence_tufa_common!(),
                            };
                            eprintln!("{error}");
                            error.into()
                        })),
                    }
                }
                Err(e) => std::future::ready(Err({
                    let error = ProjectCommitExtractorCheckErrorNamed::ProjectCommitExtractorToStrConversion{              
                        project_commit_to_str_conversion: e, 
                        code_occurence: crate::code_occurence_tufa_common!(),
                    };
                    eprintln!("{error}");
                    error.into()
                })),
            },
            None => std::future::ready(Err({
                let error = ProjectCommitExtractorCheckErrorNamed::NoProjectCommitExtractorHeader{              
                    no_project_commit_header: "project_commit header is not provided",
                    code_occurence: crate::code_occurence_tufa_common!(),
                };
                eprintln!("{error}");
                error.into()
            }))
        }
    }
}