pub async fn not_found_route(
    axum::extract::State(app_info): axum::extract::State<
        std::sync::Arc<crate::repositories_types::tufa_server::routes::app_info::AppInfo<'_>>,
    >,
) -> (
    axum::http::StatusCode,
    crate::server::routes::git_info::GitInfo,
) {
    (
        axum::http::StatusCode::NOT_FOUND,
        crate::server::routes::git_info::GitInfo {
            project_commit:
                crate::common::git::get_git_commit_link::GetGitCommitLink::get_git_commit_link(
                    app_info.project_git_info,
                ),
            commit: crate::common::git::get_git_commit_link::GetGitCommitLink::get_git_commit_link(
                app_info.repository_git_info,
            ),
        },
    )
}
