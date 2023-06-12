#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ProjectCommitExtractor {}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum ProjectCommitExtractorCheckErrorNamed<'a> {
    ProjectCommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        project_commit_not_equal: &'a str,
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

impl actix_web::FromRequest for ProjectCommitExtractor {
    type Error = actix_web::Error;
    type Future = std::future::Ready<Result<Self, Self::Error>>;
    fn from_request(req: &actix_web::HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        match req
            .headers()
            .get(crate::common::git::project_git_info::PROJECT_COMMIT)
        {
            Some(project_commit_header_value) => match project_commit_header_value.to_str() {
                Ok(possible_project_commit) => {
                    match possible_project_commit == crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO.project_commit {
                        true => std::future::ready(Ok(ProjectCommitExtractor {})),
                        false => std::future::ready(Err(//todo maybe add logs here?
                            actix_web::error::ErrorBadRequest(
                                actix_web::web::Json(
                                    ProjectCommitExtractorCheckErrorNamed::ProjectCommitExtractorNotEqual {              
                                        project_commit_not_equal: "different project commit provided, services must work only with equal project commits", 
                                        code_occurence: crate::code_occurence_tufa_common!(),
                                    }.into_serialize_deserialize_version()
                                )
                            )
                        )),
                    }
                }
                Err(e) => std::future::ready(Err(actix_web::error::ErrorBadRequest(//todo maybe use some different status code in this case
                    actix_web::web::Json(
                            ProjectCommitExtractorCheckErrorNamed::ProjectCommitExtractorToStrConversion{              
                                project_commit_to_str_conversion: e, 
                                code_occurence: crate::code_occurence_tufa_common!(),
                            }.into_serialize_deserialize_version()
                        )
                    )
                )),
            },
            None => std::future::ready(Err(
                actix_web::error::ErrorBadRequest(
                    actix_web::web::Json(
                        ProjectCommitExtractorCheckErrorNamed::NoProjectCommitExtractorHeader{              
                            no_project_commit_header: "project_commit header is not provided",
                            code_occurence: crate::code_occurence_tufa_common!(),
                        }.into_serialize_deserialize_version()
                    )
                )
            ))
        }
    }
}