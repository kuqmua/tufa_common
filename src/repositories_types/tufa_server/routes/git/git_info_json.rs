pub async fn git_info_json() -> impl actix_web::Responder {
    actix_web::web::Json(crate::common::git::git_info::GitInformation {
        git_commit_id: &crate::global_variables::compile_time::git_info::GIT_INFO.git_commit_id,
        git_repo_link: &crate::global_variables::compile_time::git_info::GIT_INFO.git_repo_link,
        // git_author: &crate::global_variables::compile_time::git_info::GIT_INFO.git_author,
        // git_author_email: &crate::global_variables::compile_time::git_info::GIT_INFO
        //     .git_author_email,
        // git_commit_unix_time: &crate::global_variables::compile_time::git_info::GIT_INFO
        //     .git_commit_unix_time,
        // git_timezone: &crate::global_variables::compile_time::git_info::GIT_INFO.git_timezone,
        // git_message: &crate::global_variables::compile_time::git_info::GIT_INFO.git_message,
    })
}
