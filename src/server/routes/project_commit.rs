#[derive(serde::Serialize)]
pub struct ProjectGitInfo {
    project_commit: std::string::String,
}

pub async fn project_git_info<'a>() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().json(actix_web::web::Json(ProjectGitInfo {
        project_commit: {
            use crate::common::git::get_git_commit_link::GetGitCommitLink;
            crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO
                .get_git_commit_link()
        },
    }))
}
