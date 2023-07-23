#[derive(serde::Serialize)]
pub struct GitInfo {
    pub project_commit: std::string::String,
    pub repository_commit: std::string::String,
}

impl axum::response::IntoResponse for GitInfo {
    fn into_response(self) -> axum::response::Response {
        let mut res = axum::Json(self).into_response();
        *res.status_mut() = http::StatusCode::OK;
        res
    }
}

#[actix_web::get("/git_info")]
pub async fn git_info(
    app_info: actix_web::web::Data<
        crate::repositories_types::tufa_server::try_build_actix_web_dev_server::AppInfo<'_>,
    >,
) -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().json(actix_web::web::Json(GitInfo {
        project_commit:
            crate::common::git::get_git_commit_link::GetGitCommitLink::get_git_commit_link(
                app_info.project_git_info,
            ),
        repository_commit:
            crate::common::git::get_git_commit_link::GetGitCommitLink::get_git_commit_link(
                app_info.repository_git_info,
            ),
    }))
}

pub async fn git_info_axum(
    axum::extract::State(app_info): axum::extract::State<
        std::sync::Arc<
            crate::repositories_types::tufa_server::try_build_actix_web_dev_server::AppInfo<'_>,
        >,
    >,
) -> GitInfo {
    GitInfo {
        project_commit:
            crate::common::git::get_git_commit_link::GetGitCommitLink::get_git_commit_link(
                app_info.project_git_info,
            ),
        repository_commit:
            crate::common::git::get_git_commit_link::GetGitCommitLink::get_git_commit_link(
                app_info.repository_git_info,
            ),
    }
}
