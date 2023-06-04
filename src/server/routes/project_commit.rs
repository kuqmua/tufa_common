#[derive(serde::Serialize)]
pub struct ProjectGitInfo {
    project_commit: std::string::String,
}

#[actix_web::get("/project_git_info")]
pub async fn project_git_info(
    app_info: actix_web::web::Data<
        crate::repositories_types::tufa_server::try_build_actix_web_dev_server::AppInfo<'_>,
    >,
) -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().json(actix_web::web::Json(ProjectGitInfo {
        project_commit: {
            use crate::common::git::get_git_commit_link::GetGitCommitLink;
            app_info.project_git_info.get_git_commit_link()
        },
    }))
}
