#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GitCommitLinkWrapper {
    git_commit_link: std::string::String,
}
pub async fn git_commit_link() -> impl actix_web::Responder {
    actix_web::web::Json(GitCommitLinkWrapper {
        git_commit_link: {
            use crate::common::git::get_git_commit_link::GetGitCommitLink;
            crate::global_variables::compile_time::git_info::GIT_INFO.get_git_commit_link()
        },
    })
}
