#[derive(serde::Serialize)]
pub struct NotFound {
    pub message: std::string::String,
    pub project_commit: std::string::String,
    pub commit: std::string::String,
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

pub async fn not_found(uri: http::Uri) -> (axum::http::StatusCode, String) {
    (
        axum::http::StatusCode::NOT_FOUND,
        format!("No route for {}", uri),
    )
}
