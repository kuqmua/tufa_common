pub async fn project_commit_checker<B>(
    req: axum::http::Request<B>,
    next: axum::middleware::Next<B>,
) -> Result<axum::response::Response, axum::response::Response> {
    match req
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
                        project_commit_not_equal: "different project commit provided, services must work only with equal project commits",
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
                    no_project_commit_header: "project_commit header is not provided",
                    code_occurence: crate::code_occurence_tufa_common!(),
                }.into_serialize_deserialize_version()
            ),
        )))
    }
}