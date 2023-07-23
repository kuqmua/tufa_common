// pub fn common_routes() -> axum::Router
// // <
// //     std::sync::Arc<
// //         crate::repositories_types::tufa_server::try_build_actix_web_dev_server::AppInfo<'static>,
// //     >,
// // >
// {
//     axum::Router::new()
// }

// pub fn health_check_route() -> axum::Router<
//     std::sync::Arc<
//         crate::repositories_types::tufa_server::try_build_actix_web_dev_server::AppInfo<'_>,
//     >,
//     _, // Router<Arc<AppInfo<'_>>, _>
// > {
//     axum::Router::new().route(
//         "/health_check",
//         axum::routing::get(crate::repositories_types::tufa_server::routes::health_check_axum),
//     )
// }
