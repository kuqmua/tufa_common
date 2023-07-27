pub fn common_routes(
    app_info: crate::server::routes::git_info::DynArcGitInfoRouteParametersSendSync,
) -> axum::Router {
    axum::Router::new()
        .merge(crate::server::routes::health_check::health_check_route())
        .merge(crate::server::routes::git_info::git_info_route(app_info))
}
