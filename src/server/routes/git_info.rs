pub(crate) type DynArcGitInfoRouteParametersSendSync =
    std::sync::Arc<dyn GitInfoRouteParameters + Send + Sync>;

pub trait GitInfoRouteParameters:
    crate::common::git::project_git_info::GetProjectGitCommitLink
    + crate::common::git::get_git_commit_link::GetGitCommitLink
{
}

#[derive(serde::Serialize, utoipa::ToSchema, Clone)]
pub struct GitInfo {
    #[schema(example = "Buy groceries")]//todo
    project_commit: std::string::String,
    commit: std::string::String,
}

#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema, Clone)]
pub struct Todo {
    id: i32,
    #[schema(example = "Buy groceries")]
    value: String,
    done: bool,
}

#[utoipa::path(
    get,
    path = "/todo",
    responses(
        (status = 200, description = "List all todos successfully", body = [Todo])
    )
)]
pub async fn git_info(
    axum::extract::State(app_info): axum::extract::State<DynArcGitInfoRouteParametersSendSync>,
) -> impl axum::response::IntoResponse {
    (
        axum::http::StatusCode::OK,
        axum::Json(GitInfo {
            project_commit: app_info.get_project_git_commit_link(),
            commit: app_info.get_git_commit_link(),
        }),
    )
}

pub(crate) fn git_info_route(app_info: DynArcGitInfoRouteParametersSendSync) -> axum::Router {
    axum::Router::new()
        .route("/git_info", axum::routing::get(git_info))
        .with_state(app_info)
}
