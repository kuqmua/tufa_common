#[derive(serde::Serialize)]
pub struct GitInfo {
    pub project_commit: std::string::String,
    pub commit: std::string::String,
}

impl axum::response::IntoResponse for GitInfo {
    fn into_response(self) -> axum::response::Response {
        let mut res = axum::Json(self).into_response();
        *res.status_mut() = http::StatusCode::OK;
        res
    }
}

pub(crate) type DynArcGitInfoRouteParametersSendSync =
    std::sync::Arc<dyn GitInfoRouteParameters + Send + Sync>;

pub trait GitInfoRouteParameters:
    crate::common::git::project_git_info::GetProjectGitCommitLink
    + crate::common::git::get_git_commit_link::GetGitCommitLink
{
}

async fn git_info_axum(
    axum::extract::State(app_info): axum::extract::State<DynArcGitInfoRouteParametersSendSync>,
) -> GitInfo {
    GitInfo {
        project_commit: app_info.get_project_git_commit_link(),
        commit: app_info.get_git_commit_link(),
    }
}

pub(crate) fn git_info_route(app_info: DynArcGitInfoRouteParametersSendSync) -> axum::Router {
    axum::Router::new()
        .route("/git_info", axum::routing::get(git_info_axum))
        .with_state(app_info)
}
