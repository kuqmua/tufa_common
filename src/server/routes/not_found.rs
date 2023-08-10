pub(crate) type DynArcNotFoundRouteParametersSendSync =
    std::sync::Arc<dyn NotFoundRouteParameters + Send + Sync>;

pub trait NotFoundRouteParameters:
    crate::common::git::project_git_info::GetProjectGitCommitLink
    + crate::common::git::get_git_commit_link::GetGitCommitLink
{
}
//todo maybe use swagger instead
async fn not_found(
    uri: http::Uri,
    axum::extract::State(app_info): axum::extract::State<DynArcNotFoundRouteParametersSendSync>,
) -> impl axum::response::IntoResponse {
    #[derive(Debug, serde::Serialize)]
    struct NotFoundHandle {
        message: std::string::String,
        project_commit: std::string::String,
        commit: std::string::String,
    }
    (
        axum::http::StatusCode::NOT_FOUND,
        axum::Json(NotFoundHandle {
            message: format!("No route for {uri}"),
            project_commit: app_info.get_project_git_commit_link(),
            commit: app_info.get_git_commit_link(),
        }),
    )
}

pub fn not_found_route(app_info: DynArcNotFoundRouteParametersSendSync) -> axum::Router {
    axum::Router::new().fallback(not_found).with_state(app_info)
}
