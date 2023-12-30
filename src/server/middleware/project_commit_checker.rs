pub async fn project_commit_checker(//todo maybe check not wrapper repo commit id but instead create array with git modules repos ids what actually synced with current wrapper repo commit id
    axum::extract::State(app_info): axum::extract::State<crate::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync>,
    req: axum::http::Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> Result<axum::response::Response, axum::response::Response> {
    match crate::common::config::config_fields::GetEnableApiGitCommitCheck::get_enable_api_git_commit_check(app_info.get_config())  {
        true => match req
            .headers()
            .get(crate::common::git::project_git_info::PROJECT_COMMIT)
            .and_then(|header| header.to_str().ok()) 
        {
            Some(project_commit_checker_header) => match project_commit_checker_header
                == crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO
                    .project_commit
            {
                true => Ok(next.run(req).await),
                false => Err(axum::response::IntoResponse::into_response((
                    axum::http::StatusCode::BAD_REQUEST,
                    axum::Json(
                        crate::server::extractors::project_commit_extractor::ProjectCommitExtractorCheckErrorNamed::ProjectCommitExtractorNotEqual {
                            project_commit_not_equal: std::string::String::from("different project commit provided, services must work only with equal project commits"),
                            project_commit_to_use: crate::common::git::get_git_commit_link::GetGitCommitLink::get_git_commit_link(&crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO),
                            code_occurence: crate::code_occurence_tufa_common!(),
                        }.into_serialize_deserialize_version()
                    ),
                )))
            },
            None => Err(axum::response::IntoResponse::into_response((
                axum::http::StatusCode::BAD_REQUEST,
                axum::Json(
                    crate::server::extractors::project_commit_extractor::ProjectCommitExtractorCheckErrorNamed::NoProjectCommitExtractorHeader {
                        no_project_commit_header: std::string::String::from("project_commit header is not provided"),
                        code_occurence: crate::code_occurence_tufa_common!(),
                    }.into_serialize_deserialize_version()
                ),
            )))
        },
        false => Ok(next.run(req).await),
    }
    // todo!()
}
