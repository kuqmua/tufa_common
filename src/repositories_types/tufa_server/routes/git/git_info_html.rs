pub async fn git_info_html() -> actix_web::HttpResponse {
    {
        actix_web::HttpResponse::Ok()
            .content_type(actix_web::http::header::ContentType::html())
            .body({
                use crate::traits::get_git_html_info::GetGitHtmlInfo;
                crate::global_variables::compile_time::git_info::GIT_INFO.get_git_html_info()
            })
    }
}
