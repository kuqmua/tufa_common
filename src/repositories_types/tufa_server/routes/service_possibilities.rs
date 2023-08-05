pub trait ServicePossibilities:
    crate::server::routes::git_info::GitInfoRouteParameters
    + crate::server::routes::not_found::NotFoundRouteParameters
    + crate::repositories_types::tufa_server::routes::api::cats::GetConfigGetPostgresPool
{
}
