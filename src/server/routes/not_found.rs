#[derive(Debug, serde::Serialize)]
pub struct NotFoundHandle {
    // pub message: std::string::String,
    pub project_commit: std::string::String,
    // pub commit: std::string::String,
}

// pub async fn not_found(
//     axum::extract::State(app_info): axum::extract::State<
//         crate::server::routes::git_info::DynArcGitInfoRouteParametersSendSync,
//     >,
// ) -> (axum::http::StatusCode, NotFound) {
//     (
//         axum::http::StatusCode::NOT_FOUND,
//         NotFound {
//             message: std::string::String::from("404 not found. please check api through git"),
//             project_commit: app_info.get_project_git_commit_link(),
//             commit: app_info.get_git_commit_link(),
//         },
//     )
// }
//uri: http::Uri
// pub async fn not_found() -> axum::response::Response {
//     (
//         axum::http::StatusCode::NOT_FOUND,
//         NotFoundHandle {
//             // message: format!("No route for {uri}"),
//             project_commit:
//                 crate::common::git::project_git_info::GetProjectGitCommitLink::get_project_git_commit_link(
//                     &crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO
//                 ),
//         },
//     ).into_response()
// }

pub async fn not_found(uri: http::Uri) -> (axum::http::StatusCode, String) {
    (
        axum::http::StatusCode::NOT_FOUND,
        format!("No route for {}", uri),
    )
}
