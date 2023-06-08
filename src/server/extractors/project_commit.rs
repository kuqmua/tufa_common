#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ProjectCommit {}

impl actix_web::FromRequest for ProjectCommit {
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
                        true => std::future::ready(Ok(ProjectCommit {})),
                        false => std::future::ready(Err(
                            actix_web::error::ErrorBadRequest(
                                actix_web::web::Json(
                                    crate::repositories_types::tufa_server::try_build_actix_web_dev_server::ProjectCommitCheckErrorNamed::ProjectCommitNotEqual {              
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
                            crate::repositories_types::tufa_server::try_build_actix_web_dev_server::ProjectCommitCheckErrorNamed::ProjectCommitToStrConversion{              
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
                        crate::repositories_types::tufa_server::try_build_actix_web_dev_server::ProjectCommitCheckErrorNamed::NoProjectCommitHeader{              
                            no_project_commit_header: "project_commit header is not provided",
                            code_occurence: crate::code_occurence_tufa_common!(),
                        }.into_serialize_deserialize_version()
                    )
                )
            ))
        }
    }
}