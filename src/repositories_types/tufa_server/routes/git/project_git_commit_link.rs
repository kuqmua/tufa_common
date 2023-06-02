#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ProjectGitCommitLinkWrapper {
    project_git_commit_link: std::string::String,
}
pub async fn project_git_commit_link() -> impl actix_web::Responder {
    actix_web::web::Json(ProjectGitCommitLinkWrapper {
        project_git_commit_link: {
            use crate::common::git::get_git_commit_link::GetGitCommitLink;
            crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO
                .get_git_commit_link()
        },
    })
}
